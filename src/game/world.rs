use bevy::prelude::*;

#[derive(Component)]
pub struct ColliderMarker;

#[derive(Component)]
pub struct TriggerMarker;

#[derive(Component)]
pub struct TriggerData(pub std::collections::HashMap<String, String>);

pub fn spawn_placeholder_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        ColliderMarker,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            ..default()
        })),
        Transform::default(),
        Name::new("Collider_Ground"),
    ));

    // A few platforms
    let platform_positions = [
        Vec3::new(3.0, 1.5, 0.0),
        Vec3::new(6.0, 3.0, -2.0),
        Vec3::new(9.0, 5.0, 1.0),
    ];
    for (i, pos) in platform_positions.iter().enumerate() {
        commands.spawn((
            ColliderMarker,
            Mesh3d(meshes.add(Cuboid::new(2.0, 0.3, 2.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.4, 0.5),
                ..default()
            })),
            Transform::from_translation(*pos),
            Name::new(format!("Collider_Platform_{i}")),
        ));
    }

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
}

pub fn parse_name_convention(name: &str) -> Option<NameConvention> {
    if name.starts_with("Collider_") {
        Some(NameConvention::Collider)
    } else if name.starts_with("Trigger_") {
        Some(NameConvention::Trigger)
    } else {
        None
    }
}

pub enum NameConvention {
    Collider,
    Trigger,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collider_prefix_recognized() {
        assert!(matches!(parse_name_convention("Collider_Ground"), Some(NameConvention::Collider)));
        assert!(matches!(parse_name_convention("Collider_Platform_0"), Some(NameConvention::Collider)));
    }

    #[test]
    fn trigger_prefix_recognized() {
        assert!(matches!(parse_name_convention("Trigger_Zone"), Some(NameConvention::Trigger)));
        assert!(matches!(parse_name_convention("Trigger_Wisp_01"), Some(NameConvention::Trigger)));
    }

    #[test]
    fn unrecognized_name_returns_none() {
        assert!(parse_name_convention("Mesh_Rock").is_none());
        assert!(parse_name_convention("").is_none());
    }
}
