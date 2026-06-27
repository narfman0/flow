use bevy::prelude::*;
use super::flow::FlowMeter;
use super::movement::MovementState;

const DISRUPTION_WINDOW: f64 = 15.0;
const SHAKEN_DURATION: f64 = 20.0;
const SHAKEN_SPEED_MULT: f32 = 0.5;

#[derive(Event)]
pub struct DisruptionEvent;

#[derive(Resource, Default)]
pub struct DisruptionState {
    pub hit_times: Vec<f64>,
    pub shaken_until: f64,
}

impl DisruptionState {
    pub fn is_shaken(&self, now: f64) -> bool {
        now < self.shaken_until
    }

    pub fn add_hit(&mut self, now: f64) -> bool {
        self.hit_times.retain(|&t| now - t < DISRUPTION_WINDOW);
        self.hit_times.push(now);
        if self.hit_times.len() >= 3 {
            self.shaken_until = now + SHAKEN_DURATION;
            self.hit_times.clear();
            return true;
        }
        false
    }
}

pub fn handle_disruption_events(
    mut events: EventReader<DisruptionEvent>,
    mut disruption: ResMut<DisruptionState>,
    mut flow_query: Query<&mut FlowMeter>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();
    for _ in events.read() {
        let became_shaken = disruption.add_hit(now);
        if became_shaken {
            for mut flow in &mut flow_query {
                flow.disrupt();
                flow.locked = true;
            }
        } else {
            for mut flow in &mut flow_query {
                flow.disrupt();
            }
        }
    }
}

pub fn clear_shaken_state(
    mut disruption: ResMut<DisruptionState>,
    mut flow_query: Query<&mut FlowMeter>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();
    if disruption.shaken_until > 0.0 && !disruption.is_shaken(now) {
        disruption.shaken_until = 0.0;
        for mut flow in &mut flow_query {
            flow.locked = false;
        }
    }
}

pub fn apply_shaken_movement(disruption: Res<DisruptionState>, mut query: Query<&mut MovementState>, time: Res<Time>) {
    if disruption.is_shaken(time.elapsed_secs_f64()) {
        for mut movement in &mut query {
            movement.velocity.x *= SHAKEN_SPEED_MULT;
            movement.velocity.z *= SHAKEN_SPEED_MULT;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_hits_in_window_triggers_shaken() {
        let mut state = DisruptionState::default();
        assert!(!state.add_hit(0.0));
        assert!(!state.add_hit(5.0));
        assert!(state.add_hit(10.0));
        assert!(state.is_shaken(11.0));
    }

    #[test]
    fn hits_spread_out_dont_trigger_shaken() {
        let mut state = DisruptionState::default();
        state.add_hit(0.0);
        state.add_hit(20.0); // first hit expired
        state.add_hit(35.0); // second hit expired
        assert!(!state.is_shaken(36.0));
    }

    #[test]
    fn shaken_expires_after_duration() {
        let mut state = DisruptionState::default();
        state.add_hit(0.0);
        state.add_hit(1.0);
        state.add_hit(2.0);
        assert!(state.is_shaken(5.0));
        assert!(!state.is_shaken(25.0)); // 20s shaken duration
    }

    #[test]
    fn shaken_speed_mult_value() {
        assert_eq!(SHAKEN_SPEED_MULT, 0.5);
    }
}
