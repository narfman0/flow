use bevy::prelude::*;
use crate::state::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_loading_ui)
           .add_systems(Update, transition_when_ready.run_if(in_state(GameState::Loading)))
           .add_systems(OnExit(GameState::Loading), despawn_loading_ui);
    }
}

#[derive(Component)]
struct LoadingUi;

fn spawn_loading_ui(mut commands: Commands) {
    commands.spawn((
        LoadingUi,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|p| {
        p.spawn(Text::new("Loading..."));
    });
}

fn despawn_loading_ui(mut commands: Commands, query: Query<Entity, With<LoadingUi>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

fn transition_when_ready(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::MainMenu);
}
