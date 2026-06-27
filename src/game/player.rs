use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerInput {
    pub move_dir: Vec2,
    pub jump_pressed: bool,
    pub dash_pressed: bool,
    pub grab_pressed: bool,
}

pub fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Player,
        PlayerInput::default(),
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.5, 0.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}

pub fn read_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerInput, With<Player>>,
) {
    for mut input in &mut query {
        let mut dir = Vec2::ZERO;
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) { dir.y += 1.0; }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) { dir.y -= 1.0; }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) { dir.x -= 1.0; }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) { dir.x += 1.0; }
        input.move_dir = if dir != Vec2::ZERO { dir.normalize() } else { Vec2::ZERO };
        input.jump_pressed = keys.just_pressed(KeyCode::Space);
        input.dash_pressed = keys.just_pressed(KeyCode::ShiftLeft);
        input.grab_pressed = keys.pressed(KeyCode::KeyE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_input_default_zero() {
        let input = PlayerInput::default();
        assert_eq!(input.move_dir, Vec2::ZERO);
        assert!(!input.jump_pressed);
        assert!(!input.dash_pressed);
    }

    #[test]
    fn move_dir_normalizes() {
        let raw = Vec2::new(1.0, 1.0);
        let normalized = raw.normalize();
        assert!((normalized.length() - 1.0).abs() < 0.001);
    }
}
