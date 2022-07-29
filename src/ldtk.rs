use bevy::{app::AppExit, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use iyes_loopless::prelude::*;

use crate::{
    ApplicationState, ApplicationStateMenu, ExitTheGameEvent, PauseTheGameEvent, ResumeTheGameEvent,
};

pub struct GameLdtkPlugin;

impl Plugin for GameLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_startup_system(setup)
            .add_system(keyboard_state_changer)
            .add_system(change_game_state);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("Typical_2D_platformer_wip.ldtk"),
        ..Default::default()
    });
}

fn keyboard_state_changer(
    app_state: Res<CurrentState<ApplicationState>>,
    mut pause_game_event: EventWriter<PauseTheGameEvent>,
    mut resume_game_event: EventWriter<ResumeTheGameEvent>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match app_state.0 {
            ApplicationState::Game => {
                pause_game_event.send(PauseTheGameEvent);
            }
            ApplicationState::Menu(_) => {
                resume_game_event.send(ResumeTheGameEvent);
            }
        }
    }
}

fn change_game_state(
    mut commands: Commands,
    mut pause_game_event: EventReader<PauseTheGameEvent>,
    mut resume_game_event: EventReader<ResumeTheGameEvent>,
    mut exit_game_event: EventReader<ExitTheGameEvent>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut exit: EventWriter<AppExit>,
) {
    for _ in pause_game_event.iter() {
        commands.insert_resource(NextState(ApplicationState::Menu(
            ApplicationStateMenu::Main,
        )));

        // Turn off the physics when we pause the game
        rapier_config.physics_pipeline_active = false;
    }

    for _ in resume_game_event.iter() {
        commands.insert_resource(NextState(ApplicationState::Game));

        // Turn on the physics when we resume the game
        rapier_config.physics_pipeline_active = true;
    }

    for _ in exit_game_event.iter() {
        exit.send(AppExit);
    }
}
