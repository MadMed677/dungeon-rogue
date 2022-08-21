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
mod ron_parsers;
mod tests;
mod tutorial;
mod ui;

use std::collections::HashSet;

use bevy::{prelude::*, render::texture::ImageSettings};
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
use ron_parsers::RonParsersPlugin;
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
pub struct JumpAnimation {
    timer: Timer,
}

#[derive(Component)]
pub struct AttackAnimation {
    timer: Timer,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ApplicationState {
    /// Describes that currently a player in the game
    Game,

    /// Describes that currently a player in the menu
    Menu(ApplicationStateMenu),
}

/// If `true` than the entity in attack state
/// Otherwise - no
#[derive(Debug, Component, Inspectable)]
pub struct Attacks(bool);

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

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
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
        .insert_resource(ImageSettings::default_nearest())
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
        .add_plugin(RonParsersPlugin)
        .add_plugin(OutOfBouncePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
