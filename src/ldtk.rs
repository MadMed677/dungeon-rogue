use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::{ApplicationState, PauseTheGameEvent, ResumeTheGameEvent};

pub struct GameLdtkPlugin;

impl Plugin for GameLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                // level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            // .add_startup_stage("game_setup_ldtk", SystemStage::single(setup));
            .add_startup_system(setup)
            .add_system(keyboard_state_changer)
            .add_system(change_game_state);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        // ldtk_handle: asset_server.load("top_down_map.ldtk"),
        ldtk_handle: asset_server.load("Typical_2D_platformer_example.ldtk"),
        ..Default::default()
    });
}

fn keyboard_state_changer(
    app_state: Res<State<ApplicationState>>,
    mut pause_game_event: EventWriter<PauseTheGameEvent>,
    mut resume_game_event: EventWriter<ResumeTheGameEvent>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match app_state.current() {
            ApplicationState::Game => {
                pause_game_event.send(PauseTheGameEvent);
            }
            ApplicationState::Menu => {
                resume_game_event.send(ResumeTheGameEvent);
            }
        }
    }
}

fn change_game_state(
    mut app_state: ResMut<State<ApplicationState>>,
    mut pause_game_event: EventReader<PauseTheGameEvent>,
    mut resume_game_event: EventReader<ResumeTheGameEvent>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    for _ in pause_game_event.iter() {
        if app_state.set(ApplicationState::Menu).is_ok() {
            // Turn off the physics when we pause the game
            rapier_config.physics_pipeline_active = false;
        }
    }

    for _ in resume_game_event.iter() {
        if app_state.set(ApplicationState::Game).is_ok() {
            // Turn on the physics when we resume the game
            rapier_config.physics_pipeline_active = true;
        }
    }
}
