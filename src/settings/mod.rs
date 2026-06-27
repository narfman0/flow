use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::state::GameState;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = load_settings();
        app.insert_resource(settings)
           .add_systems(OnEnter(GameState::Settings), spawn_settings_ui)
           .add_systems(OnExit(GameState::Settings), despawn_settings_ui);
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

#[derive(Component)]
struct SettingsUi;

fn spawn_settings_ui(mut commands: Commands) {
    commands.spawn((SettingsUi, Node::default()));
}

fn despawn_settings_ui(mut commands: Commands, query: Query<Entity, With<SettingsUi>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
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
