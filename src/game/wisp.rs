use bevy::prelude::*;
use super::disruption::DisruptionEvent;

#[derive(Component)]
pub struct Wisp {
    pub amplitude: f32,
    pub frequency: f32,
    pub origin: Vec3,
    pub time: f32,
}

impl Default for Wisp {
    fn default() -> Self {
        Self {
            amplitude: 2.0,
            frequency: 0.5,
            origin: Vec3::ZERO,
            time: 0.0,
        }
    }
}

pub fn update_wisps(mut query: Query<(&mut Wisp, &mut Transform)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut wisp, mut tf) in &mut query {
        wisp.time += dt;
        tf.translation = wisp.origin + Vec3::new(
            wisp.amplitude * (wisp.time * wisp.frequency * std::f32::consts::TAU).sin(),
            0.5 + 0.3 * (wisp.time * wisp.frequency * 0.7 * std::f32::consts::TAU).sin(),
            wisp.amplitude * (wisp.time * wisp.frequency * std::f32::consts::TAU * 0.8).cos(),
        );
    }
}

pub fn check_wisp_player_contact(
    wisp_query: Query<&Transform, With<Wisp>>,
    player_query: Query<&Transform, With<super::player::Player>>,
    mut disruption_events: EventWriter<DisruptionEvent>,
) {
    let Ok(player_tf) = player_query.single() else { return };
    for wisp_tf in &wisp_query {
        let dist = wisp_tf.translation.distance(player_tf.translation);
        if dist < 1.0 {
            disruption_events.write(DisruptionEvent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wisp_drift_changes_position() {
        let mut wisp = Wisp::default();
        wisp.origin = Vec3::ZERO;
        wisp.time = 0.0;

        let pos0 = Vec3::new(
            wisp.amplitude * (wisp.time * wisp.frequency * std::f32::consts::TAU).sin(),
            0.5,
            wisp.amplitude * (wisp.time * wisp.frequency * std::f32::consts::TAU * 0.8).cos(),
        );

        wisp.time = 1.0;
        let pos1 = Vec3::new(
            wisp.amplitude * (wisp.time * wisp.frequency * std::f32::consts::TAU).sin(),
            0.5,
            wisp.amplitude * (wisp.time * wisp.frequency * std::f32::consts::TAU * 0.8).cos(),
        );

        assert_ne!(pos0, pos1);
    }

    #[test]
    fn wisp_default_amplitude() {
        let w = Wisp::default();
        assert_eq!(w.amplitude, 2.0);
        assert_eq!(w.frequency, 0.5);
    }
}
