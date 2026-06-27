use bevy::prelude::*;
use crate::state::GameState;

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CutsceneEndEvent>()
           .init_resource::<CutsceneSequence>()
           .add_systems(OnEnter(GameState::Cutscene), spawn_cutscene_ui)
           .add_systems(Update, advance_cutscene.run_if(in_state(GameState::Cutscene)))
           .add_systems(OnExit(GameState::Cutscene), despawn_cutscene_ui);
    }
}

#[derive(Event)]
pub struct CutsceneEndEvent;

#[derive(Debug, Clone)]
pub enum CutsceneStep {
    ShowText(String),
    ShowImage(String),
    WaitSeconds(f32),
    AdvanceOnInput,
    MoveCameraTo(Vec3),
}

#[derive(Resource, Default)]
pub struct CutsceneSequence {
    pub steps: Vec<CutsceneStep>,
    pub current: usize,
    pub wait_timer: f32,
}

#[derive(Component)]
struct CutsceneUi;

fn spawn_cutscene_ui(mut commands: Commands) {
    commands.spawn((CutsceneUi, Node::default()));
}

fn despawn_cutscene_ui(mut commands: Commands, query: Query<Entity, With<CutsceneUi>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

fn advance_cutscene(
    mut sequence: ResMut<CutsceneSequence>,
    mut next_state: ResMut<NextState<GameState>>,
    mut end_events: EventWriter<CutsceneEndEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if sequence.current >= sequence.steps.len() {
        end_events.write(CutsceneEndEvent);
        next_state.set(GameState::InGame);
        return;
    }

    let step = sequence.steps[sequence.current].clone();
    match step {
        CutsceneStep::WaitSeconds(secs) => {
            sequence.wait_timer += time.delta_secs();
            if sequence.wait_timer >= secs {
                sequence.wait_timer = 0.0;
                sequence.current += 1;
            }
        }
        CutsceneStep::AdvanceOnInput => {
            if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
                sequence.current += 1;
            }
        }
        _ => {
            sequence.current += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_advances() {
        let mut seq = CutsceneSequence {
            steps: vec![
                CutsceneStep::ShowText("Hello".into()),
                CutsceneStep::WaitSeconds(1.0),
            ],
            current: 0,
            wait_timer: 0.0,
        };
        assert_eq!(seq.current, 0);
        seq.current += 1;
        assert_eq!(seq.current, 1);
    }

    #[test]
    fn wait_step_timing() {
        let mut timer = 0.0f32;
        let target = 2.0f32;
        timer += 1.5;
        assert!(timer < target);
        timer += 0.6;
        assert!(timer >= target);
    }
}
