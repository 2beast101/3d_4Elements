#![allow(non_snake_case)]
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;

mod player;
use player::PlayerPlugin;

mod camera;
use camera::CameraPlugin;

mod world;
use world::WorldPlugin;

mod physics;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin,
            PhysicsPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
