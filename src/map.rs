use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::player::Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Inspectable)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        // ldtk_handle: asset_server.load("top_down_map.ldtk"),
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

fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }
    // Store all GridCoords by specific level
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();
    for (&grid_coords, &Parent(parent)) in wall_query.iter() {
        if let Ok(&Parent(level_entity)) = parent_query.get(parent) {
            level_to_wall_locations
                .entry(level_entity)
                .or_insert(HashSet::new())
                .insert(grid_coords);
        }
    }

    if !wall_query.is_empty() {
        for (level_entity, level_handle) in level_query.iter() {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // Combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right
                    // edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });

                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut wall_rects: Vec<Rect<i32>> = Vec::new();
                let mut previous_rects: HashMap<Plate, Rect<i32>> = HashMap::new();

                // an extra empty row so the algorithm "terminates" the rects that
                // touch the top edge
                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, Rect<i32>> = HashMap::new();

                    for plate in row {
                        if let Some(previous_rect) = previous_rects.remove(plate) {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: y as i32,
                                    bottom: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                            );
                        }
                    }

                    wall_rects.append(&mut previous_rects.values().copied().collect());
                    previous_rects = current_rects;
                }

                for wall_rect in wall_rects {
                    commands
                        .spawn()
                        .insert(Collider::cuboid(
                            (wall_rect.right as f32 - wall_rect.left as f32 + 1.0)
                                * grid_size as f32
                                / 2.0,
                            (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.0)
                                * grid_size as f32
                                / 2.0,
                        ))
                        .insert(RigidBody::Fixed)
                        .insert(Friction::new(0.1))
                        .insert(Transform::from_xyz(
                            (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.0,
                            (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.0,
                            0.0,
                        ))
                        .insert(GlobalTransform::default())
                        .insert(Parent(level_entity));
                }
            }
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system(setup)
            .add_system(fit_camera_inside_current_level)
            .add_system(spawn_wall_collision)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WallBundle>(3);
    }
}
