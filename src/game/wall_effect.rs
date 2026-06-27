use bevy::prelude::*;

const EFFECT_DURATION: f32 = 8.0;
const RAMP_IN: f32 = 0.5;
const RAMP_OUT: f32 = 1.5;

#[derive(Event)]
pub struct WallEffectEvent;

#[derive(Resource, Default)]
pub struct WallEffectState {
    pub timer: f32,
    pub active: bool,
}

impl WallEffectState {
    pub fn trigger(&mut self) {
        self.timer = 0.0;
        self.active = true;
    }

    /// Returns saturation multiplier 0.0–1.0 based on ramp in/hold/ramp out curve
    pub fn intensity(&self) -> f32 {
        if !self.active { return 0.0; }
        let t = self.timer;
        if t < RAMP_IN {
            t / RAMP_IN
        } else if t < EFFECT_DURATION - RAMP_OUT {
            1.0
        } else {
            let remaining = EFFECT_DURATION - t;
            (remaining / RAMP_OUT).clamp(0.0, 1.0)
        }
    }
}

pub fn handle_wall_effect_events(
    mut events: EventReader<WallEffectEvent>,
    mut state: ResMut<WallEffectState>,
) {
    for _ in events.read() {
        state.trigger();
    }
}

pub fn tick_wall_effect(mut state: ResMut<WallEffectState>, time: Res<Time>) {
    if !state.active { return; }
    state.timer += time.delta_secs();
    if state.timer >= EFFECT_DURATION {
        state.active = false;
        state.timer = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_activates_effect() {
        let mut state = WallEffectState::default();
        assert!(!state.active);
        state.trigger();
        assert!(state.active);
        assert_eq!(state.timer, 0.0);
    }

    #[test]
    fn intensity_ramps_in() {
        let mut state = WallEffectState { active: true, timer: 0.25 };
        let intensity = state.intensity();
        assert!((intensity - 0.5).abs() < 0.01);
    }

    #[test]
    fn intensity_full_during_hold() {
        let state = WallEffectState { active: true, timer: 3.0 };
        assert_eq!(state.intensity(), 1.0);
    }

    #[test]
    fn intensity_ramps_out() {
        let state = WallEffectState { active: true, timer: EFFECT_DURATION - RAMP_OUT * 0.5 };
        let intensity = state.intensity();
        assert!(intensity > 0.0 && intensity < 1.0);
    }

    #[test]
    fn inactive_state_zero_intensity() {
        let state = WallEffectState::default();
        assert_eq!(state.intensity(), 0.0);
    }

    #[test]
    fn timer_duration_8_seconds() {
        assert_eq!(EFFECT_DURATION, 8.0);
    }
}
