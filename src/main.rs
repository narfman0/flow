mod cutscene;
mod game;
mod loading;
mod menu;
mod pause;
mod save;
mod settings;
mod state;

use bevy::prelude::*;
use avian3d::prelude::PhysicsPlugins;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "flow".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false })
        .init_state::<state::GameState>()
        .add_plugins((
            loading::LoadingPlugin,
            menu::MenuPlugin,
            settings::SettingsPlugin,
            pause::PausePlugin,
            game::GamePlugin,
            cutscene::CutscenePlugin,
        ))
        .run();
}
