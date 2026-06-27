mod cutscene;
mod game;
mod loading;
mod menu;
mod pause;
mod save;
mod settings;
mod state;

use bevy::prelude::*;

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
