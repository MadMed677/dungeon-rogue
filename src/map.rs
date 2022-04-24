use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(LdtkWorldBundle {
        // ldtk_handle: asset_server.load("test.ldtk"),
        ldtk_handle: asset_server.load("Typical_2D_platformer_example.ldtk"),
        ..Default::default()
    });
}

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn fit_camera_inside_current_level(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    // Get player transform to handle `position`
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation;
        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                let level_ratio = level.px_wid as f32 / level.px_hei as f32;

                orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                orthographic_projection.bottom = 0.0;
                orthographic_projection.left = 0.0;

                // If the level is wider than the screen
                if level_ratio > ASPECT_RATIO {
                    orthographic_projection.top = (level.px_hei as f32 / 9.0).round() * 9.0;
                    orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;

                    // Update camera translation
                    camera_transform.translation.x = (player_translation.x
                        - level_transform.translation.x
                        - orthographic_projection.right / 2.0)
                        .clamp(0.0, level.px_wid as f32 - orthographic_projection.right);
                    camera_transform.translation.y = 0.0;
                }

                // camera_transform.translation.x += level_transform.translation.x;
                // camera_transform.translation.y += level_transform.translation.x;
            }
        }
    };
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system(setup)
            .add_system(fit_camera_inside_current_level)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<WallBundle>(1);
    }
}
