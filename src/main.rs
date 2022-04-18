mod map;
mod player;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use map::MapPlugin;
use player::PlayerPlugin;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn setup(mut commands: Commands) {
    // commands.spawn_bundle(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.2, 0.7, 0.2),
    //         custom_size: Some(Vec2::new(100.0, 5.0)),
    //         ..Default::default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, -10.0, 0.0),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: String::from("Dungeon Rogue"),
            width: 700.0,
            height: 680.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .run();
}
