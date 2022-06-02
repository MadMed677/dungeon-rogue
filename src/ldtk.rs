use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        // ldtk_handle: asset_server.load("top_down_map.ldtk"),
        ldtk_handle: asset_server.load("Typical_2D_platformer_example.ldtk"),
        ..Default::default()
    });
}
