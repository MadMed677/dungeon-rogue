mod camera;
mod debug;
mod map;
mod physics;
mod player;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use camera::CameraPlugin;
use debug::DebugPlugin;
use map::MapPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Speed(f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: String::from("Dungeon Rogue"),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
