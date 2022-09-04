#![allow(clippy::forget_non_drop)]

mod audio;
mod combat;
mod common;
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

use bevy::{prelude::*, render::texture::ImageSettings};
use iyes_loopless::prelude::*;

use audio::GameAudioPlugin;
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
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RonParsersPlugin)
        .add_plugin(OutOfBouncePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(DebugPlugin)
        .run();
}
