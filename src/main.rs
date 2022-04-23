mod map;
mod player;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use map::MapPlugin;
use player::PlayerPlugin;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: String::from("Dungeon Rogue"),
            width: 800.0,
            height: 680.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
