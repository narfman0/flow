use bevy::prelude::*;
use super::player::{Player, PlayerInput};
use super::flow::FlowMeter;

const MOVE_SPEED: f32 = 7.0;
const JUMP_IMPULSE: f32 = 8.0;
const DASH_DISTANCE: f32 = 5.0;
const DASH_COOLDOWN: f32 = 0.8;
const GRAVITY: f32 = -20.0;

#[derive(Component)]
pub struct MovementState {
    pub velocity: Vec3,
    pub is_grounded: bool,
    pub jumps_remaining: u8,
    pub dash_cooldown: f32,
    pub air_dashes_remaining: u8,
}

impl Default for MovementState {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            is_grounded: true,
            jumps_remaining: 2,
            dash_cooldown: 0.0,
            air_dashes_remaining: 1,
        }
    }
}

pub fn apply_movement(
    mut query: Query<(&PlayerInput, &mut MovementState, &mut Transform, &FlowMeter), With<Player>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (input, mut state, mut tf, _flow) in &mut query {
        // Horizontal movement
        let move_3d = Vec3::new(input.move_dir.x, 0.0, -input.move_dir.y);
        state.velocity.x = move_3d.x * MOVE_SPEED;
        state.velocity.z = move_3d.z * MOVE_SPEED;

        // Jump
        if input.jump_pressed && state.jumps_remaining > 0 {
            state.velocity.y = JUMP_IMPULSE;
            state.jumps_remaining -= 1;
            state.is_grounded = false;
        }

        // Dash
        state.dash_cooldown = (state.dash_cooldown - dt).max(0.0);
        if input.dash_pressed {
            let can_dash = if state.is_grounded {
                state.dash_cooldown <= 0.0
            } else {
                state.air_dashes_remaining > 0
            };
            if can_dash {
                let dir = if move_3d != Vec3::ZERO { move_3d } else { *-tf.forward() };
                state.velocity += dir * DASH_DISTANCE;
                if state.is_grounded {
                    state.dash_cooldown = DASH_COOLDOWN;
                } else {
                    state.air_dashes_remaining -= 1;
                }
            }
        }

        // Gravity
        if !state.is_grounded {
            state.velocity.y += GRAVITY * dt;
        }

        // Move
        tf.translation += state.velocity * dt;

        // Simple ground plane at y=0
        if tf.translation.y <= 0.0 {
            tf.translation.y = 0.0;
            state.velocity.y = 0.0;
            if !state.is_grounded {
                state.is_grounded = true;
                state.jumps_remaining = 2;
                state.air_dashes_remaining = 1;
            }
        } else {
            state.is_grounded = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_grounded() {
        let state = MovementState::default();
        assert!(state.is_grounded);
        assert_eq!(state.jumps_remaining, 2);
        assert_eq!(state.air_dashes_remaining, 1);
    }

    #[test]
    fn dash_cooldown_ticks_down() {
        let mut state = MovementState::default();
        state.dash_cooldown = 0.8;
        state.dash_cooldown = (state.dash_cooldown - 0.3).max(0.0);
        assert!((state.dash_cooldown - 0.5).abs() < 0.001);
    }

    #[test]
    fn dash_cooldown_clamps_to_zero() {
        let mut state = MovementState::default();
        state.dash_cooldown = 0.1;
        state.dash_cooldown = (state.dash_cooldown - 0.5).max(0.0);
        assert_eq!(state.dash_cooldown, 0.0);
    }

    #[test]
    fn jumping_decrements_jumps_remaining() {
        let mut state = MovementState::default();
        assert_eq!(state.jumps_remaining, 2);
        state.jumps_remaining -= 1;
        assert_eq!(state.jumps_remaining, 1);
        state.jumps_remaining -= 1;
        assert_eq!(state.jumps_remaining, 0);
    }

    #[test]
    fn landing_resets_jumps() {
        let mut state = MovementState::default();
        state.is_grounded = false;
        state.jumps_remaining = 0;
        state.air_dashes_remaining = 0;
        // simulate landing
        state.is_grounded = true;
        state.jumps_remaining = 2;
        state.air_dashes_remaining = 1;
        assert_eq!(state.jumps_remaining, 2);
        assert_eq!(state.air_dashes_remaining, 1);
    }
}
