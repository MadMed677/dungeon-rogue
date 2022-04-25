mod debug;
mod map;
mod player;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use debug::DebugPlugin;
use map::MapPlugin;
use player::PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Speed(f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: String::from("Dungeon Rogue"),
            width: 1024.0,
            height: 576.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
