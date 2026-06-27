use bevy::prelude::*;
use crate::state::GameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause.run_if(in_state(GameState::InGame)))
           .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
           .add_systems(Update, pause_button_interactions.run_if(in_state(GameState::Paused)))
           .add_systems(OnExit(GameState::Paused), despawn_pause_menu);
    }
}

#[derive(Component)]
struct PauseUi;

#[derive(Component)]
enum PauseButton {
    Resume,
    Settings,
    MainMenu,
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        PauseUi,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
    )).with_children(|parent| {
        for (label, action) in [
            ("Resume", PauseButton::Resume),
            ("Settings", PauseButton::Settings),
            ("Main Menu", PauseButton::MainMenu),
        ] {
            parent.spawn((
                action,
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            )).with_children(|p| {
                p.spawn(Text::new(label));
            });
        }
    });
}

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseUi>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

fn pause_button_interactions(
    mut interaction_query: Query<(&Interaction, &PauseButton), With<Button>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                PauseButton::Resume => next_state.set(GameState::InGame),
                PauseButton::Settings => next_state.set(GameState::Settings),
                PauseButton::MainMenu => next_state.set(GameState::MainMenu),
            }
        }
    }
}
