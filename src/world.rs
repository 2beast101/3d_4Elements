use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_object, spawn_light));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.0))),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Name::new("Floor"),
    );
    commands.spawn((RigidBody::Fixed, Collider::cuboid(50.0, 0.0, 50.0), floor));
}

// fn setup_terrain_scene(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     // Configure a properly scaled cascade shadow map for this scene (defaults are too large, mesh units are in km)
//     let cascade_shadow_config = CascadeShadowConfigBuilder {
//         first_cascade_far_bound: 0.3,
//         maximum_distance: 3.0,
//         ..default()
//     }
//     .build();

//     // Sun
//     commands.spawn(DirectionalLightBundle {
//         directional_light: DirectionalLight {
//             color: Color::rgb(0.98, 0.95, 0.82),
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(0.0, 0.0, 0.0)
//             .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
//         cascade_shadow_config,
//         ..default()
//     });

//     // Terrain
//     commands.spawn((
//         SceneBundle {
//             scene: asset_server.load("Mountains.gltf#Scene0"),
//             transform: Transform::from_scale(Vec3::new(100.0, 80.0, 100.0)),
//             ..default()
//         },
//         Name::new("Terrain"),
//         RigidBody::Fixed,
//     ));

//     // Sky
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Box::default())),
//             material: materials.add(StandardMaterial {
//                 base_color: Color::hex("888888").unwrap(),
//                 unlit: true,
//                 cull_mode: None,
//                 ..default()
//             }),
//             transform: Transform::from_scale(Vec3::splat(1000.0)),
//             ..default()
//         },
//         NotShadowCaster,
//     ));
// }

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                color: Color::rgba(1.0, 0.78, 0.0, 1.0),
                intensity: 100.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("Mainlight"),
    );
    commands.spawn(light);
}

fn spawn_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_cube =
        |size: f32, color: Color, xyz: (f32, f32, f32), name: String| -> (PbrBundle, Name) {
            (
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
                    material: materials.add(color.into()),
                    transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                    ..default()
                },
                Name::new(name),
            )
        };
    commands.spawn((
        create_cube(4.0, Color::BLUE, (-5.0, 2.0, 5.0), "Blue Cube".to_string()),
        RigidBody::Dynamic,
        Collider::cuboid(2.0, 2.0, 2.0),
    ));
    commands.spawn((
        create_cube(2.0, Color::RED, (-0.5, 6.0, 0.0), "Red Cube".to_string()),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));
}
