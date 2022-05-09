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

#[derive(Inspectable)]
enum MovementTendency {
    Left,
    Right,
}

#[derive(Component, Inspectable)]
pub struct MovementDirection(MovementTendency);

struct Sprites {
    player: Handle<TextureAtlas>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("atlas/pumpkin_dude.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 32.0), 4, 1);

    commands.insert_resource(Sprites {
        player: texture_atlases.add(texture_atlas),
    });
}

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
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
