mod map;
mod player;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use map::MapPlugin;
use player::PlayerPlugin;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Component, Default)]
pub struct Player;

#[derive(Component)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(0.0)
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: String::from("Dungeon Rogue"),
            width: 1024.0,
            height: 576.0,
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
