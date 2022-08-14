#![allow(clippy::forget_non_drop)]

mod audio;
mod combat;
mod debug;
mod enemy;
mod hud;
mod ldtk;
mod map;
mod out_of_bounce;
mod physics;
mod player;
mod settings;
mod tests;
mod tutorial;
mod ui;

use std::collections::HashSet;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use audio::GameAudioPlugin;
use bevy_inspector_egui::Inspectable;
use combat::CombatPlugin;
use debug::DebugPlugin;
use enemy::EnemyPlugin;
use hud::HudPlugin;
use ldtk::GameLdtkPlugin;
use map::MapPlugin;
use out_of_bounce::OutOfBouncePlugin;
use physics::PhysicsPlugin;
use player::{PlayerAnimationState, PlayerPlugin};
use settings::SettingsPlugin;
use ui::UIPlugin;

#[derive(Component, Inspectable, Debug)]
pub struct Speed(f32);

#[derive(Component)]
pub struct IdleAnimation {
    timer: Timer,
}

#[derive(Component)]
pub struct ClimbAnimation {
    timer: Timer,
    index: usize,
}

#[derive(Component)]
pub struct MovementAnimation {
    timer: Timer,
    index: usize,
}

#[derive(Component)]
pub struct HurtAnimation {
    timer: Timer,
}

#[derive(Component)]
pub struct DeathAnimation {
    timer: Timer,
}

#[derive(Component, Default, Inspectable)]
/// Describes that entity on move or not
pub struct OnMove(bool);

#[derive(Component, Inspectable, PartialEq, Clone, Debug)]
enum MovementDirection {
    Left,
    Right,
}

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

#[derive(Component, Clone, Debug, Default, Inspectable)]
pub struct Health {
    /// Describes current health
    pub current: i32,

    /// Describes maximum health
    pub max: i32,
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

struct PlayerSprites {
    idle: SpriteAssetInfo,
    run: SpriteAssetInfo,
    climb: SpriteAssetInfo,
    hurt: SpriteAssetInfo,
    death: SpriteAssetInfo,
}

struct MonstersSprites {
    gray: SpriteAssetInfo,
    long: SpriteAssetInfo,
}

struct TutorialSprites {
    movement: Handle<Image>,
}

pub struct Sprites {
    player: PlayerSprites,
    monsters: MonstersSprites,
    tutorial: TutorialSprites,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ApplicationState {
    /// Describes that currently a player in the game
    Game,

    /// Describes that currently a player in the menu
    Menu(ApplicationStateMenu),
}

/// Describes all states for a menu
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ApplicationStateMenu {
    /// Describes that currently a player in the main menu
    Main,

    /// Describes that currently a player in a dead menu (when the player is dead)
    Dead,

    /// Describes that currently a player in a settings menu (turn on/off, change volume of the music, etc...)
    Settings,
}

pub struct PauseTheGameEvent;
pub struct ResumeTheGameEvent;

pub struct ExitTheGameEvent;

#[derive(Debug)]
pub struct PlayerIsDeadEvent;

/// Should be fire when the player get hit but not dead
/// Accepts how many points the player receives
#[derive(Debug)]
pub struct PlayerIsHitEvent(i32);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let apple_idle_texture_width = 34.0;
    let apple_idle_texture_height = 32.0;
    let apple_idle_texture_handle = asset_server.load("atlas/player/apple@idle-sheet.png");
    let apple_idle_texture_atlas = TextureAtlas::from_grid_with_padding(
        apple_idle_texture_handle,
        Vec2::new(apple_idle_texture_width, apple_idle_texture_height),
        10,
        3,
        Vec2::new(30.0, 32.0),
    );

    let apple_run_texture_width = 24.0;
    let apple_run_texture_height = 34.0;
    let apple_run_texture_handle = asset_server.load("atlas/player/apple@run-sheet.png");
    let apple_run_texture_atlas = TextureAtlas::from_grid_with_padding(
        apple_run_texture_handle,
        Vec2::new(apple_run_texture_width, apple_run_texture_height),
        10,
        1,
        Vec2::new(40.0, 30.0),
    );

    let apple_climb_texture_width = 34.0;
    let apple_climb_texture_height = 32.0;
    let apple_climb_texture_handle = asset_server.load("atlas/player/apple@climb-sheet.png");
    let apple_climb_texture_atlas = TextureAtlas::from_grid_with_padding(
        apple_climb_texture_handle,
        Vec2::new(apple_climb_texture_width, apple_climb_texture_height),
        10,
        2,
        Vec2::new(30.0, 32.0),
    );

    let apple_hurt_texture_width = 34.0;
    let apple_hurt_texture_height = 32.0;
    let apple_hurt_texture_handle = asset_server.load("atlas/player/apple@hurt-sheet.png");
    let apple_hurt_texture_atlas = TextureAtlas::from_grid_with_padding(
        apple_hurt_texture_handle,
        Vec2::new(apple_hurt_texture_width, apple_hurt_texture_height),
        8,
        1,
        Vec2::new(30.0, 0.0),
    );

    let apple_death_texture_width = 42.0;
    let apple_death_texture_height = 42.0;
    let apple_death_texture_handle = asset_server.load("atlas/player/apple@death-sheet_2.png");
    let apple_death_texture_atlas = TextureAtlas::from_grid_with_padding(
        apple_death_texture_handle,
        Vec2::new(apple_death_texture_width, apple_death_texture_height),
        10,
        4,
        Vec2::new(22.0, 20.0),
    );

    let gray_monster_texture_width = 16.0;
    let gray_monster_texture_height = 16.0;
    let gray_monster_texture_handle = asset_server.load("atlas/enemies/gray_monster.png");
    let gray_monster_texture_atlas = TextureAtlas::from_grid(
        gray_monster_texture_handle,
        Vec2::new(gray_monster_texture_width, gray_monster_texture_height),
        4,
        1,
    );

    let long_green_monster_texture_width = 15.0;
    let long_green_monster_texture_height = 16.0;
    let long_green_monster_texture_handle =
        asset_server.load("atlas/enemies/long_hair_monster.png");
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
        player: PlayerSprites {
            idle: SpriteAssetInfo {
                width: apple_idle_texture_width,
                height: apple_idle_texture_height,
                texture: texture_atlases.add(apple_idle_texture_atlas),
            },
            run: SpriteAssetInfo {
                width: apple_run_texture_width,
                height: apple_run_texture_height,
                texture: texture_atlases.add(apple_run_texture_atlas),
            },
            climb: SpriteAssetInfo {
                width: apple_climb_texture_width,
                height: apple_climb_texture_height,
                texture: texture_atlases.add(apple_climb_texture_atlas),
            },
            hurt: SpriteAssetInfo {
                width: apple_hurt_texture_width,
                height: apple_hurt_texture_height,
                texture: texture_atlases.add(apple_hurt_texture_atlas),
            },
            death: SpriteAssetInfo {
                width: apple_death_texture_width,
                height: apple_death_texture_height,
                texture: texture_atlases.add(apple_death_texture_atlas),
            },
        },
        monsters: MonstersSprites {
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
        tutorial: TutorialSprites {
            movement: asset_server.load("atlas/tutorial/keyboard_arrows.png"),
        },
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
        .add_loopless_state(ApplicationState::Menu(ApplicationStateMenu::Main))
        .add_loopless_state(PlayerAnimationState::Idle)
        .add_event::<PauseTheGameEvent>()
        .add_event::<ResumeTheGameEvent>()
        .add_event::<ExitTheGameEvent>()
        .add_event::<PlayerIsDeadEvent>()
        .add_event::<PlayerIsHitEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameLdtkPlugin)
        .add_startup_system(setup)
        .add_plugin(UIPlugin)
        .add_plugin(GameAudioPlugin)
        // Deactivate tutorial for now. Because there is no ability to
        //  spawn tutorial and change visibility on menu and do not
        //  destroy the whole UI at all
        // .add_plugin(TutorialPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(SettingsPlugin)
        .add_plugin(OutOfBouncePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
