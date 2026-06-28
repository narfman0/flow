use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::state::GameState;
use crate::settings::SettingsReturn;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause.run_if(in_state(GameState::InGame)))
           .add_systems(Update, pause_menu_ui.run_if(in_state(GameState::Paused)));
    }
}

/// Escape pauses the game from active play.
fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

/// Centered egui pause panel. Escape resumes; buttons drive state transitions.
fn pause_menu_ui(
    mut contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings_return: ResMut<SettingsReturn>,
) {
    // Escape toggles back out of the pause menu.
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
        return;
    }

    let ctx = contexts.ctx_mut();
    egui::Window::new("Paused")
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.set_min_width(220.0);
            ui.vertical_centered_justified(|ui| {
                ui.add_space(4.0);
                if ui.button("Resume").clicked() {
                    next_state.set(GameState::InGame);
                }
                if ui.button("Settings").clicked() {
                    settings_return.0 = GameState::Paused;
                    next_state.set(GameState::Settings);
                }
                if ui.button("Quit to Menu").clicked() {
                    next_state.set(GameState::MainMenu);
                }
                ui.add_space(4.0);
            });
        });
}
