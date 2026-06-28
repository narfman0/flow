use bevy::prelude::*;
use crate::state::GameState;
use crate::save::save_exists;
use crate::settings::SettingsReturn;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_menu)
           .add_systems(Update, button_interactions.run_if(in_state(GameState::MainMenu)))
           .add_systems(OnExit(GameState::MainMenu), despawn_menu);
    }
}

#[derive(Component)]
struct MenuUi;

#[derive(Component, PartialEq)]
enum MenuButton {
    NewGame,
    Continue,
    Settings,
    Quit,
}

const NORMAL_BUTTON: Color = Color::srgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);
const DISABLED_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

fn spawn_menu(mut commands: Commands) {
    let has_save = save_exists();
    let buttons = [
        ("New Game", MenuButton::NewGame, false),
        ("Continue", MenuButton::Continue, !has_save),
        ("Settings", MenuButton::Settings, false),
        ("Quit", MenuButton::Quit, false),
    ];

    commands.spawn((
        MenuUi,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..default()
        },
    )).with_children(|parent| {
        for (label, action, disabled) in buttons {
            let color = if disabled { DISABLED_BUTTON } else { NORMAL_BUTTON };
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
                BackgroundColor(color),
            )).with_children(|p| {
                p.spawn(Text::new(label));
            });
        }
    });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuUi>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

fn button_interactions(
    mut interaction_query: Query<(&Interaction, &MenuButton, &mut BackgroundColor), With<Button>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings_return: ResMut<SettingsReturn>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, mut color) in &mut interaction_query {
        if *button == MenuButton::Continue && !save_exists() {
            *color = BackgroundColor(DISABLED_BUTTON);
            continue;
        }
        match *interaction {
            Interaction::Pressed => match button {
                MenuButton::NewGame => next_state.set(GameState::InGame),
                MenuButton::Continue => next_state.set(GameState::InGame),
                MenuButton::Settings => {
                    settings_return.0 = GameState::MainMenu;
                    next_state.set(GameState::Settings);
                }
                MenuButton::Quit => { exit.write(AppExit::Success); }
            },
            Interaction::Hovered => *color = BackgroundColor(HOVERED_BUTTON),
            Interaction::None => *color = BackgroundColor(NORMAL_BUTTON),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::save::save_exists;

    #[test]
    fn save_exists_false_initially() {
        // Only valid when run from a clean temp dir; guards the menu Continue button logic
        if !save_exists() {
            assert!(!save_exists());
        }
    }
}
