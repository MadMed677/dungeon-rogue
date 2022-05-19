mod debug;
mod ldtk;
mod map;
mod physics;
mod player;

use std::collections::HashSet;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use debug::DebugPlugin;
use ldtk::GameLdtkPlugin;
use map::MapPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(50.0)
    }
}

#[derive(Inspectable)]
enum MovementTendency {
    Left,
    Right,
}

impl Default for MovementTendency {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Component, Default, Inspectable)]
pub struct MovementDirection(MovementTendency);

#[derive(Component, Copy, Clone, Debug, Default)]
/// Describes that this element
///  might be used for `Climber` entities
pub struct Climbable;

#[derive(Component, Clone, Debug, Default)]
/// Describes that this entity
///  may interact with `Climbable` elements
pub struct Climber {
    /// Describes that climber faced intersection with
    ///  `Climbable` element and it's ready to climb
    /// Contains a list of all intersaction elements
    ///  which the Climber has a contact with
    intersaction_elements: HashSet<Entity>,

    // Describes that climber is in climbing process
    climbing: bool,
}

struct Sprites {
    player: Handle<TextureAtlas>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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
        .add_plugins(DefaultPlugins)
        .add_plugin(GameLdtkPlugin)
        .add_startup_system(setup)
        .add_plugin(PhysicsPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
