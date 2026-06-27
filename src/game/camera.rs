use bevy::prelude::*;
use super::player::Player;

#[derive(Component)]
pub struct GameCamera {
    pub offset: Vec3,
    pub smoothing: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 4.0, -8.0),
            smoothing: 8.0,
        }
    }
}

pub fn spawn_game_camera(mut commands: Commands) {
    commands.spawn((
        GameCamera::default(),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&GameCamera, &mut Transform), Without<Player>>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_query.single() else { return };
    for (cam, mut cam_tf) in &mut camera_query {
        let target = player_tf.translation + cam.offset;
        cam_tf.translation = cam_tf.translation.lerp(target, cam.smoothing * time.delta_secs());
        cam_tf.look_at(player_tf.translation, Vec3::Y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_default_offset() {
        let cam = GameCamera::default();
        assert_eq!(cam.offset.y, 4.0);
        assert_eq!(cam.offset.z, -8.0);
    }

    #[test]
    fn lerp_moves_toward_target() {
        let start = Vec3::ZERO;
        let target = Vec3::new(10.0, 0.0, 0.0);
        let result = start.lerp(target, 0.5);
        assert!((result.x - 5.0).abs() < 0.001);
    }
}
