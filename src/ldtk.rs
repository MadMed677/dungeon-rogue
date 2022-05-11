use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct GameLdtkPlugin;

impl Plugin for GameLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(LdtkWorldBundle {
        // ldtk_handle: asset_server.load("top_down_map.ldtk"),
        ldtk_handle: asset_server.load("Typical_2D_platformer_example.ldtk"),
        ..Default::default()
    });
}
