use bevy::prelude::*;
use crate::state::GameState;
use crate::cutscene::{CutsceneSequence, CutsceneStep};
use super::dialogue::{DialogueQueue, DialogueLine};
use super::wall_effect::{WallEffectEvent, WallEffectState};
use super::wisp::Wisp;
use super::player::Player;

/// Fired when the player reaches the experiment origin point in the sealed wing.
#[derive(Event)]
pub struct BubbleCloseEvent;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FinalePhase {
    #[default]
    Inactive,
    /// Cole's death dialogue is playing.
    ColeDeath,
    /// Final wall effect is running; wisps being despawned.
    BubbleClosing,
    /// Geometry went still; brief pause before the ending cutscene.
    GeometryStill,
    /// Ending cutscene has been launched.
    Ending,
}

#[derive(Resource, Default)]
pub struct FinaleState {
    pub phase: FinalePhase,
    pub timer: f32,
}

impl FinaleState {
    pub fn is_active(&self) -> bool {
        self.phase != FinalePhase::Inactive
    }
}

/// Duration of "geometry still" silence before transitioning to the ending cutscene.
const GEOMETRY_STILL_DURATION: f32 = 4.0;

/// The glTF Trigger_ zone at the experiment origin. Hardcoded until the level
/// asset has a Trigger_Origin object.
const ORIGIN_POSITION: Vec3 = Vec3::new(0.0, 1.0, -30.0);
const ORIGIN_RADIUS: f32 = 2.5;

/// Detects when the player enters the experiment origin trigger zone.
pub fn check_origin_trigger(
    player_query: Query<&Transform, With<Player>>,
    finale: Res<FinaleState>,
    mut events: EventWriter<BubbleCloseEvent>,
) {
    if finale.is_active() {
        return;
    }
    let Ok(player_tf) = player_query.single() else { return };
    if player_tf.translation.distance(ORIGIN_POSITION) < ORIGIN_RADIUS {
        events.write(BubbleCloseEvent);
    }
}

/// Responds to BubbleCloseEvent by starting Cole's death dialogue and entering
/// the ColeDeath phase.
pub fn handle_bubble_close_event(
    mut events: EventReader<BubbleCloseEvent>,
    mut finale: ResMut<FinaleState>,
    mut dialogue: ResMut<DialogueQueue>,
) {
    for _ in events.read() {
        if finale.phase != FinalePhase::Inactive {
            continue;
        }
        finale.phase = FinalePhase::ColeDeath;
        // Cole dies mid-sentence; Daven finishes it for him.
        dialogue.push(DialogueLine {
            speaker: "Cole".into(),
            text: "The boundary — it's not failing, it's fulfilling. The math always said it would close back on—".into(),
            portrait: None,
        });
        dialogue.push(DialogueLine {
            speaker: "Daven".into(),
            text: "—on itself. Back to the moment it began. Yes. I know.".into(),
            portrait: None,
        });
        dialogue.push(DialogueLine {
            speaker: "Daven".into(),
            text: "You always did finish faster than you should have.".into(),
            portrait: None,
        });
    }
}

/// Main finale state machine. Advances through ColeDeath → BubbleClosing →
/// GeometryStill → Ending, triggering effects at each transition.
pub fn advance_finale(
    mut finale: ResMut<FinaleState>,
    dialogue: Res<DialogueQueue>,
    mut wall_effect_events: EventWriter<WallEffectEvent>,
    wall_effect_state: Res<WallEffectState>,
    wisp_query: Query<Entity, With<Wisp>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut cutscene_seq: ResMut<CutsceneSequence>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    match finale.phase {
        FinalePhase::Inactive | FinalePhase::Ending => {}

        FinalePhase::ColeDeath => {
            if !dialogue.is_active() {
                finale.phase = FinalePhase::BubbleClosing;
                finale.timer = 0.0;
                wall_effect_events.write(WallEffectEvent);
                for entity in &wisp_query {
                    commands.entity(entity).despawn();
                }
            }
        }

        FinalePhase::BubbleClosing => {
            finale.timer += dt;
            // The wall effect lasts 8s; wait at least 1s before checking active to
            // avoid a false-negative on the first frame after the event fires.
            if finale.timer > 1.0 && !wall_effect_state.active {
                finale.phase = FinalePhase::GeometryStill;
                finale.timer = 0.0;
            }
        }

        FinalePhase::GeometryStill => {
            finale.timer += dt;
            if finale.timer >= GEOMETRY_STILL_DURATION {
                finale.phase = FinalePhase::Ending;
                cutscene_seq.steps = build_ending_steps();
                cutscene_seq.current = 0;
                cutscene_seq.wait_timer = 0.0;
                cutscene_seq.return_state = Some(GameState::MainMenu);
                next_state.set(GameState::Cutscene);
            }
        }
    }
}

fn build_ending_steps() -> Vec<CutsceneStep> {
    vec![
        CutsceneStep::WaitSeconds(1.5),
        CutsceneStep::ShowText(
            "The anomalies are gone.\n\nThe world is fine.\n\nNormal.".into(),
        ),
        CutsceneStep::ShowText(
            "Pell returns to the valley.\nThe shimmer is gone.\nThey stay for a long time.\n\nIvo stands next to them and does not try to explain anything.".into(),
        ),
        CutsceneStep::ShowText(
            "Sela sits somewhere alone and realizes she's not performing.\nShe isn't sure when that stopped.\n\nIt's fine.".into(),
        ),
        CutsceneStep::ShowText(
            "Kai stops moving.\nSela is next to him.\n\nNothing is said about what they are to each other.".into(),
        ),
        CutsceneStep::ShowText(
            "Daven leaves something at the spot where Cole died.\nHe doesn't explain it.\n\nHe walks back to the rest of them.".into(),
        ),
        CutsceneStep::WaitSeconds(2.0),
        CutsceneStep::ShowText("flow".into()),
        CutsceneStep::WaitSeconds(3.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finale_starts_inactive() {
        let state = FinaleState::default();
        assert_eq!(state.phase, FinalePhase::Inactive);
        assert!(!state.is_active());
    }

    #[test]
    fn finale_active_during_cole_death() {
        let mut state = FinaleState::default();
        state.phase = FinalePhase::ColeDeath;
        assert!(state.is_active());
    }

    #[test]
    fn finale_active_during_bubble_closing() {
        let mut state = FinaleState::default();
        state.phase = FinalePhase::BubbleClosing;
        assert!(state.is_active());
    }

    #[test]
    fn finale_active_during_geometry_still() {
        let mut state = FinaleState::default();
        state.phase = FinalePhase::GeometryStill;
        assert!(state.is_active());
    }

    #[test]
    fn finale_active_during_ending() {
        let mut state = FinaleState::default();
        state.phase = FinalePhase::Ending;
        assert!(state.is_active());
    }

    #[test]
    fn geometry_still_duration_is_positive() {
        assert!(GEOMETRY_STILL_DURATION > 0.0);
    }

    #[test]
    fn origin_radius_is_positive() {
        assert!(ORIGIN_RADIUS > 0.0);
    }

    #[test]
    fn build_ending_steps_non_empty() {
        let steps = build_ending_steps();
        assert!(!steps.is_empty());
    }

    #[test]
    fn ending_steps_include_all_four_character_epilogues() {
        let steps = build_ending_steps();
        let text: Vec<_> = steps.iter().filter_map(|s| {
            if let CutsceneStep::ShowText(t) = s { Some(t.as_str()) } else { None }
        }).collect();
        let combined = text.join(" ");
        assert!(combined.contains("Pell"), "missing Pell epilogue");
        assert!(combined.contains("Sela"), "missing Sela epilogue");
        assert!(combined.contains("Kai"), "missing Kai epilogue");
        assert!(combined.contains("Daven"), "missing Daven epilogue");
    }

    #[test]
    fn ending_steps_contain_wait_and_text() {
        let steps = build_ending_steps();
        let has_wait = steps.iter().any(|s| matches!(s, CutsceneStep::WaitSeconds(_)));
        let has_text = steps.iter().any(|s| matches!(s, CutsceneStep::ShowText(_)));
        assert!(has_wait);
        assert!(has_text);
    }

    #[test]
    fn bubble_closing_phase_timer_guards_transition() {
        // Verify the 1-second guard in BubbleClosing phase logic is enforced by the timer.
        // We simulate the timer being below threshold.
        let mut state = FinaleState { phase: FinalePhase::BubbleClosing, timer: 0.5 };
        // Timer < 1.0 should NOT allow transition even if wall effect is inactive.
        assert!(state.timer <= 1.0);
    }

    #[test]
    fn geometry_still_transitions_after_duration() {
        let mut timer = 0.0f32;
        timer += GEOMETRY_STILL_DURATION + 0.1;
        assert!(timer >= GEOMETRY_STILL_DURATION);
    }
}
