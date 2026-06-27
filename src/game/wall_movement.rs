use bevy::prelude::*;
use super::flow::{FlowMeter, FlowTier};
use super::movement::MovementState;
use super::player::{Player, PlayerInput};

#[derive(Component, Default)]
pub struct WallState {
    pub on_wall: bool,
    pub wall_normal: Vec3,
    pub wall_run_timer: f32,
    pub is_wall_climbing: bool,
}

pub fn wall_max_run_time(tier: FlowTier) -> f32 {
    match tier {
        FlowTier::Low => 1.5,
        FlowTier::Mid => 3.0,
        FlowTier::Full => 3.0,
    }
}

pub fn can_wall_climb(flow: &FlowMeter) -> bool {
    flow.tier() == FlowTier::Full
}

pub fn update_wall_movement(
    mut query: Query<(&PlayerInput, &mut WallState, &mut MovementState, &FlowMeter), With<Player>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (input, mut wall, mut movement, flow) in &mut query {
        if !wall.on_wall {
            wall.is_wall_climbing = false;
            wall.wall_run_timer = 0.0;
            continue;
        }

        let max_time = wall_max_run_time(flow.tier());

        // Wall climb: grab key + full flow
        if input.grab_pressed && can_wall_climb(flow) {
            wall.is_wall_climbing = true;
            wall.wall_run_timer += dt;
            if wall.wall_run_timer > 2.0 {
                // Max climb time reached, must jump off
                wall.on_wall = false;
                wall.is_wall_climbing = false;
            } else {
                movement.velocity.y = 4.0; // climb upward
                movement.velocity.x = 0.0;
                movement.velocity.z = 0.0;
            }
        } else {
            // Horizontal wall run
            wall.is_wall_climbing = false;
            wall.wall_run_timer += dt;
            if wall.wall_run_timer > max_time {
                wall.on_wall = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wall_run_time_by_tier() {
        assert_eq!(wall_max_run_time(FlowTier::Low), 1.5);
        assert_eq!(wall_max_run_time(FlowTier::Mid), 3.0);
        assert_eq!(wall_max_run_time(FlowTier::Full), 3.0);
    }

    #[test]
    fn wall_climb_requires_full_flow() {
        let mut flow = FlowMeter::default();
        flow.value = 50.0;
        assert!(!can_wall_climb(&flow));
        flow.value = 85.0;
        assert!(can_wall_climb(&flow));
    }

    #[test]
    fn wall_state_defaults_off_wall() {
        let state = WallState::default();
        assert!(!state.on_wall);
        assert!(!state.is_wall_climbing);
        assert_eq!(state.wall_run_timer, 0.0);
    }
}
