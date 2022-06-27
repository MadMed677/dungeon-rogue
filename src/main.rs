mod debug;
mod enemy;
mod ldtk;
mod main_menu_ui;
mod map;
mod physics;
mod player;
mod tutorial;

use std::collections::HashSet;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use bevy_inspector_egui::Inspectable;
use debug::DebugPlugin;
use enemy::EnemyPlugin;
use ldtk::GameLdtkPlugin;
use main_menu_ui::MainMenuUIPlugin;
use map::MapPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use tutorial::TutorialPlugin;

#[derive(Component, Inspectable)]
pub struct Speed(f32);

#[derive(Inspectable)]
enum MovementTendency {
    Left,
    Right,
}

#[derive(Component, Inspectable)]
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

/// Describes the sprite assets information
///
/// !!Note!! Works only with TextureAtlas
#[derive(Clone, Debug)]
struct SpriteAssetInfo {
    /// The `width` of the sprite cell (not the whole atlas texture)
    width: f32,

    /// The `height` of the sprite cell (not the whole atlas texture)
    height: f32,

    /// TextureAtlas
    texture: Handle<TextureAtlas>,
}

struct Monsters {
    gray: SpriteAssetInfo,
    long: SpriteAssetInfo,
}

struct Sprites {
    pumpkin: SpriteAssetInfo,
    dragon: SpriteAssetInfo,
    monsters: Monsters,
    tutorial_movement: Handle<Image>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ApplicationState {
    /// Describes that currently a player in the game
    Game,

    /// Describes that currently a player in the menu
    Menu,
}

pub struct PauseTheGameEvent;
pub struct ResumeTheGameEvent;

pub struct ExitTheGameEvent;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let pumpkin_texture_width = 16.0;
    let pumpkin_texture_height = 24.0;
    let pumpkin_texture_handle = asset_server.load("atlas/pumpkin_dude_16_24.png");
    let pumpkin_texture_atlas = TextureAtlas::from_grid(
        pumpkin_texture_handle,
        Vec2::new(pumpkin_texture_width, pumpkin_texture_height),
        8,
        1,
    );

    let dragon_texture_width = 16.0;
    let dragon_texture_height = 22.0;
    let dragon_texture_handle = asset_server.load("atlas/dragon_dude_16_22.png");
    let dragon_texture_atlas = TextureAtlas::from_grid(
        dragon_texture_handle,
        Vec2::new(dragon_texture_width, dragon_texture_height),
        9,
        1,
    );

    let gray_monster_texture_width = 16.0;
    let gray_monster_texture_height = 16.0;
    let gray_monster_texture_handle = asset_server.load("atlas/gray_monster.png");
    let gray_monster_texture_atlas = TextureAtlas::from_grid(
        gray_monster_texture_handle,
        Vec2::new(gray_monster_texture_width, gray_monster_texture_height),
        4,
        1,
    );

    let long_green_monster_texture_width = 15.0;
    let long_green_monster_texture_height = 16.0;
    let long_green_monster_texture_handle = asset_server.load("atlas/long_hair_monster.png");
    let long_green_monster_texture_atlas = TextureAtlas::from_grid(
        long_green_monster_texture_handle,
        Vec2::new(
            long_green_monster_texture_width,
            long_green_monster_texture_height,
        ),
        4,
        1,
    );

    commands.insert_resource(Sprites {
        pumpkin: SpriteAssetInfo {
            width: pumpkin_texture_width,
            height: pumpkin_texture_height,
            texture: texture_atlases.add(pumpkin_texture_atlas),
        },
        dragon: SpriteAssetInfo {
            width: dragon_texture_width,
            height: dragon_texture_height,
            texture: texture_atlases.add(dragon_texture_atlas),
        },
        monsters: Monsters {
            gray: SpriteAssetInfo {
                width: gray_monster_texture_width,
                height: gray_monster_texture_height,
                texture: texture_atlases.add(gray_monster_texture_atlas),
            },
            long: SpriteAssetInfo {
                width: long_green_monster_texture_width,
                height: long_green_monster_texture_height,
                texture: texture_atlases.add(long_green_monster_texture_atlas),
            },
        },
        tutorial_movement: asset_server.load("atlas/keyboard_arrows.png"),
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
        .add_loopless_state(ApplicationState::Menu)
        .add_event::<PauseTheGameEvent>()
        .add_event::<ResumeTheGameEvent>()
        .add_event::<ExitTheGameEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameLdtkPlugin)
        .add_startup_system(setup)
        .add_plugin(MainMenuUIPlugin)
        .add_plugin(TutorialPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
