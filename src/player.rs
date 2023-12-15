use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                player_movement,
                player_shoot,
                velocity,
                bullet_hit_object,
                bullet_miss,
            ),
        );
    }
}

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query_player: Query<(&mut Transform, &Speed), With<Player>>,
    query_camera: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, speed) in query_player.iter_mut() {
        let cam = match query_camera.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };
        let mut direction: Vec3 = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * speed.0 * time.delta_seconds();
        player_transform.translation += movement;
        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(-direction, Vec3::Y);
        }
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let flashlight = (
        Name::new("Flashlight"),
        SpotLightBundle {
            spot_light: SpotLight {
                color: Color::rgba(1.0, 0.96, 0.37, 1.0),
                intensity: 4000.0,
                outer_angle: 0.6,
                inner_angle: 0.5,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.5, -0.5),
                rotation: Quat::from_rotation_y(std::f32::consts::PI),
                ..default()
            },
            ..default()
        },
    );
    let scene0 = assets.load("BASEmodel.glb#Scene0");
    let player = (
        SceneBundle {
            scene: scene0,
            // transform: Transform::from_xyz(0.0, 0.5, 0.0),
            transform: Transform {
                scale: (Vec3::splat(0.3)),
                rotation: Quat::from_rotation_y(std::f32::consts::PI),
                ..default()
            },
            ..default()
        },
        Player,
        Speed(4.0),
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );
    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}

#[derive(Component)]
pub struct Velocity {
    linvel: Vec3,
}

#[derive(Component)]
struct BulletSpeed(f32);

#[derive(Component)]
struct Bullet;

#[derive(Component)]
pub struct BulletLifeTime {
    pub life: Timer,
}

fn player_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut create_bullet =
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
    if mouse.just_pressed(MouseButton::Left) {
        let player = match player_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving player: {}", e)).unwrap(),
        };
        let direction = player.rotation.mul_vec3(Vec3::Z).normalize();

        commands
            .spawn((
                create_bullet(
                    0.5,
                    Color::RED,
                    (
                        player.translation.x,
                        player.translation.y + 0.8,
                        player.translation.z,
                    ),
                    "FIRED".to_string(),
                ),
                Bullet,
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(0.25, 0.25, 0.25),
                ActiveEvents::COLLISION_EVENTS,
                BulletLifeTime {
                    life: Timer::new(Duration::from_secs(5), TimerMode::Once),
                },
            ))
            .insert(Velocity {
                linvel: direction * 15.0,
            });
    }
}

fn bullet_miss(
    mut commands: Commands,
    mut player_q: Query<(&mut BulletLifeTime, Entity), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut bullet, ent) in player_q.iter_mut() {
        bullet.life.tick(time.delta());
        if bullet.life.just_finished() {
            commands.entity(ent).despawn_recursive();
        }
    }
}

fn bullet_hit_object(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    for event in events.read() {
        match event {
            CollisionEvent::Started(_, b, _) => {
                commands.entity(*b).despawn();
            }
            CollisionEvent::Stopped(_a, _b, _) => {}
        }
    }
}

fn velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.linvel * time.delta_seconds();
    }
}
