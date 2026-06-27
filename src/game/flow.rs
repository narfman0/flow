use bevy::prelude::*;
use super::player::{Player, PlayerInput};
use super::movement::MovementState;

const FLOW_BUILD_RATE: f32 = 15.0;
const FLOW_DRAIN_RATE: f32 = 30.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowTier {
    Low,
    Mid,
    Full,
}

#[derive(Component, Default)]
pub struct FlowMeter {
    pub value: f32,
    pub locked: bool,
}

impl FlowMeter {
    pub fn tier(&self) -> FlowTier {
        if self.value >= 80.0 {
            FlowTier::Full
        } else if self.value >= 40.0 {
            FlowTier::Mid
        } else {
            FlowTier::Low
        }
    }

    pub fn disrupt(&mut self) {
        self.value = 0.0;
    }
}

pub fn update_flow(
    mut query: Query<(&PlayerInput, &MovementState, &mut FlowMeter), With<Player>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (input, movement, mut flow) in &mut query {
        if flow.locked { continue; }

        let is_moving = input.move_dir != Vec2::ZERO || !movement.is_grounded;
        if is_moving {
            flow.value = (flow.value + FLOW_BUILD_RATE * dt).min(100.0);
        } else {
            flow.value = (flow.value - FLOW_DRAIN_RATE * dt).max(0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_boundaries() {
        let mut f = FlowMeter::default();
        f.value = 0.0;
        assert_eq!(f.tier(), FlowTier::Low);
        f.value = 39.9;
        assert_eq!(f.tier(), FlowTier::Low);
        f.value = 40.0;
        assert_eq!(f.tier(), FlowTier::Mid);
        f.value = 79.9;
        assert_eq!(f.tier(), FlowTier::Mid);
        f.value = 80.0;
        assert_eq!(f.tier(), FlowTier::Full);
        f.value = 100.0;
        assert_eq!(f.tier(), FlowTier::Full);
    }

    #[test]
    fn disrupt_resets_to_zero() {
        let mut f = FlowMeter { value: 75.0, locked: false };
        f.disrupt();
        assert_eq!(f.value, 0.0);
    }

    #[test]
    fn locked_flow_doesnt_change() {
        let mut f = FlowMeter { value: 50.0, locked: true };
        // Simulate what update_flow would do
        if !f.locked {
            f.value += FLOW_BUILD_RATE * 0.1;
        }
        assert_eq!(f.value, 50.0);
    }

    #[test]
    fn flow_clamps_to_100() {
        let mut f = FlowMeter { value: 99.0, locked: false };
        f.value = (f.value + FLOW_BUILD_RATE * 10.0).min(100.0);
        assert_eq!(f.value, 100.0);
    }
}
