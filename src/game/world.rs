use bevy::prelude::*;
use bevy::asset::LoadState;
use avian3d::prelude::*;

#[derive(Component)]
pub struct ColliderMarker;

#[derive(Component)]
pub struct TriggerMarker;

#[derive(Component)]
pub struct TriggerData(pub std::collections::HashMap<String, String>);

/// Path (relative to the `assets/` root) of the primary glTF level.
const LEVEL_PATH: &str = "levels/level_01.glb";

/// Tracks the runtime-loaded glTF level scene.
///
/// `spawned` guards against spawning the scene (or fallback) more than once.
/// `fallback` records that the hardcoded platforms were used because the glTF
/// was absent or failed to load.
#[derive(Resource, Default)]
pub struct LevelHandle {
    pub scene: Option<Handle<Scene>>,
    pub spawned: bool,
    pub fallback: bool,
}

/// On entering gameplay: spawn lighting, then attempt to load the glTF level.
///
/// If the asset file is missing from disk we warn immediately and spawn the
/// hardcoded fallback platforms. Otherwise we kick off the async load and let
/// [`spawn_level_scene`] place it once it is ready.
pub fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Directional light, needed regardless of which level source is used.
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));

    if std::path::Path::new("assets").join(LEVEL_PATH).exists() {
        let scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(LEVEL_PATH));
        commands.insert_resource(LevelHandle { scene: Some(scene), spawned: false, fallback: false });
    } else {
        warn!("'{LEVEL_PATH}' not found under assets/; falling back to hardcoded platforms");
        spawn_hardcoded_platforms(&mut commands, &mut meshes, &mut materials);
        commands.insert_resource(LevelHandle { scene: None, spawned: true, fallback: true });
    }
}

/// Once the glTF scene finishes loading, spawn it as a `SceneRoot`. If the load
/// fails at runtime, warn and fall back to the hardcoded platforms.
pub fn spawn_level_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level: ResMut<LevelHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if level.spawned {
        return;
    }
    let Some(handle) = level.scene.clone() else {
        return;
    };
    match asset_server.get_load_state(&handle) {
        Some(LoadState::Loaded) => {
            commands.spawn((SceneRoot(handle), Name::new("Level_01")));
            level.spawned = true;
            info!("Loaded glTF level '{LEVEL_PATH}'");
        }
        Some(LoadState::Failed(err)) => {
            warn!("Failed to load '{LEVEL_PATH}' ({err}); falling back to hardcoded platforms");
            spawn_hardcoded_platforms(&mut commands, &mut meshes, &mut materials);
            level.spawned = true;
            level.fallback = true;
        }
        _ => {}
    }
}

/// Spawns the hardcoded ground plane and platforms, each with a static physics
/// collider. Used as the fallback when the glTF level is unavailable.
pub fn spawn_hardcoded_platforms(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
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
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
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
            RigidBody::Static,
            Collider::cuboid(2.0, 0.3, 2.0),
            Name::new(format!("Collider_Platform_{i}")),
        ));
    }
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
