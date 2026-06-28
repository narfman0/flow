use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use serde::{Deserialize, Serialize};
use crate::state::GameState;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = load_settings();
        app.insert_resource(settings)
           .init_resource::<SettingsReturn>()
           .add_systems(Update, settings_ui.run_if(in_state(GameState::Settings)));
    }
}

/// The state to return to when the Settings screen's Back button is pressed.
/// Set by whichever screen opened Settings (main menu or pause menu).
#[derive(Resource)]
pub struct SettingsReturn(pub GameState);

impl Default for SettingsReturn {
    fn default() -> Self {
        Self(GameState::MainMenu)
    }
}

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsData {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub fullscreen: bool,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 1.0,
            sfx_volume: 1.0,
            fullscreen: false,
        }
    }
}

pub fn save_settings(settings: &SettingsData) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("saves")?;
    let serialized = ron::to_string(settings)?;
    std::fs::write("saves/settings.ron", serialized)?;
    Ok(())
}

pub fn load_settings() -> SettingsData {
    std::fs::read_to_string("saves/settings.ron")
        .ok()
        .and_then(|s| ron::from_str(&s).ok())
        .unwrap_or_default()
}

/// Centered egui settings panel. Volume sliders write straight into
/// [`SettingsData`]; Back persists the settings and returns to the prior screen.
fn settings_ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<SettingsData>,
    settings_return: Res<SettingsReturn>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Settings")
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.set_min_width(320.0);
            ui.add(egui::Slider::new(&mut settings.master_volume, 0.0..=1.0).text("Master Volume"));
            ui.add(egui::Slider::new(&mut settings.music_volume, 0.0..=1.0).text("Music Volume"));
            ui.add(egui::Slider::new(&mut settings.sfx_volume, 0.0..=1.0).text("SFX Volume"));
            ui.checkbox(&mut settings.fullscreen, "Fullscreen");
            ui.separator();
            if ui.button("Back").clicked() {
                if let Err(e) = save_settings(&settings) {
                    warn!("Failed to save settings: {e}");
                }
                next_state.set(settings_return.0.clone());
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values_sane() {
        let s = SettingsData::default();
        assert_eq!(s.master_volume, 1.0);
        assert_eq!(s.music_volume, 1.0);
        assert_eq!(s.sfx_volume, 1.0);
        assert!(!s.fullscreen);
    }

    #[test]
    fn settings_round_trip() {
        let mut s = SettingsData::default();
        s.master_volume = 0.5;
        s.fullscreen = true;

        let serialized = ron::to_string(&s).unwrap();
        let loaded: SettingsData = ron::from_str(&serialized).unwrap();
        assert_eq!(loaded.master_volume, 0.5);
        assert!(loaded.fullscreen);
    }
}
