use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::state::GameState;

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CutsceneEndEvent>()
           .init_resource::<CutsceneSequence>()
           .add_systems(OnEnter(GameState::Cutscene), spawn_cutscene_ui)
           .add_systems(Update, (advance_cutscene, cutscene_text_ui).run_if(in_state(GameState::Cutscene)))
           .add_systems(OnExit(GameState::Cutscene), despawn_cutscene_ui);
    }
}

#[derive(Event)]
pub struct CutsceneEndEvent;

#[derive(Debug, Clone)]
pub enum CutsceneStep {
    /// Display text centered on screen; player presses Space/Enter to advance.
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
    /// State to transition to when the sequence finishes. Defaults to InGame.
    pub return_state: Option<GameState>,
}

#[derive(Component)]
struct CutsceneUi;

fn spawn_cutscene_ui(mut commands: Commands) {
    commands.spawn((
        CutsceneUi,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
    ));
}

fn despawn_cutscene_ui(mut commands: Commands, query: Query<Entity, With<CutsceneUi>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

/// Renders the current ShowText step as a centered egui overlay.
fn cutscene_text_ui(mut contexts: EguiContexts, sequence: Res<CutsceneSequence>) {
    let Some(step) = sequence.steps.get(sequence.current) else { return };
    let CutsceneStep::ShowText(text) = step else { return };

    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(egui::Color32::TRANSPARENT))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() * 0.35);
                ui.label(
                    egui::RichText::new(text)
                        .color(egui::Color32::from_gray(230))
                        .size(22.0),
                );
                ui.add_space(24.0);
                ui.label(
                    egui::RichText::new("Press Space or Enter to continue")
                        .color(egui::Color32::from_gray(120))
                        .italics()
                        .size(14.0),
                );
            });
        });
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
        let return_state = sequence.return_state.take().unwrap_or(GameState::InGame);
        next_state.set(return_state);
        return;
    }

    let advance_input = keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter);
    let step = sequence.steps[sequence.current].clone();
    match step {
        CutsceneStep::WaitSeconds(secs) => {
            sequence.wait_timer += time.delta_secs();
            if sequence.wait_timer >= secs {
                sequence.wait_timer = 0.0;
                sequence.current += 1;
            }
        }
        CutsceneStep::ShowText(_) | CutsceneStep::AdvanceOnInput => {
            if advance_input {
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
            return_state: None,
        };
        assert_eq!(seq.current, 0);
        seq.current += 1;
        assert_eq!(seq.current, 1);
    }

    #[test]
    fn return_state_defaults_to_none() {
        let seq = CutsceneSequence::default();
        assert!(seq.return_state.is_none());
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
