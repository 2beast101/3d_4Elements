use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec3::new(0.0, -3.0, 0.0), // Vec3::Y = -9.8
                ..default()
            })
            .add_plugins(RapierDebugRenderPlugin::default().disabled())
            .add_systems(Update, velocity);
    }
}

#[derive(Component)]
pub struct Velocity {
    linvel: Vec3,
}

pub fn velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.linvel * time.delta_seconds();
    }
}
