use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{player::Player, Climbable};

enum CollisionId {
    Dirt = 1,
    Ladder = 2,
    Stone = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Inspectable)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct DirtBundle {
    pub wall: Wall,
}
#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct StoneBundle {
    pub wall: Wall,
}

#[derive(Clone, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub sensor: Sensor,
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> Self {
        if int_grid_cell.value == CollisionId::Ladder as i32 {
            Self {
                collider: Collider::cuboid(2.0, 2.0),
                sensor: Sensor,
            }
        } else {
            unimplemented!();
        }
    }
}

#[derive(Clone, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    #[from_int_grid_cell]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub climbable: Climbable,
}

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn fit_camera_inside_current_level(
    mut camera_query: Query<
        (&mut OrthographicProjection, &mut Transform),
        (Without<Player>, With<Camera2d>),
    >,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    // Get player transform to handle `position`
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation;
        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;

                // Check the specific current level
                if level_selection.is_match(&0, level) {
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
                    } else {
                        // If the level is taller than the screen
                        orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
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
    for (&grid_coords, parent) in wall_query.iter() {
        if let Ok(level_entity) = parent_query.get(parent.get()) {
            std::collections::hash_map::Entry::or_insert(
                level_to_wall_locations.entry(level_entity.get()),
                HashSet::new(),
            )
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
                let mut wall_rects: Vec<UiRect<i32>> = Vec::new();
                let mut previous_rects: HashMap<Plate, UiRect<i32>> = HashMap::new();

                // an extra empty row so the algorithm "terminates" the rects that
                // touch the top edge
                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, UiRect<i32>> = HashMap::new();

                    for plate in row {
                        if let Some(previous_rect) = previous_rects.remove(plate) {
                            current_rects.insert(
                                *plate,
                                UiRect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                UiRect {
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
                    let x = (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.0;
                    let y = (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.0;

                    commands.entity(level_entity).with_children(|level| {
                        level
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
                            .insert(Transform::from_xyz(x, y, 0.0))
                            .insert(GlobalTransform::default())
                            .insert(WallCollision)
                            .insert(CollisionGroups::new(0b1101, 0b0100));
                    });
                }
            }
        }
    }
}

#[derive(Component)]
pub struct WallCollision;

fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in level_query.iter() {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = UiRect {
                bottom: level_transform.translation.y,
                top: level_transform.translation.y + ldtk_level.level.px_hei as f32,
                left: level_transform.translation.x,
                right: level_transform.translation.x + ldtk_level.level.px_wid as f32,
            };

            for player_transform in player_query.iter() {
                if player_transform.translation.x < level_bounds.right
                    && player_transform.translation.x > level_bounds.left
                    && player_transform.translation.y < level_bounds.top
                    && player_transform.translation.y > level_bounds.bottom
                    && !level_selection.is_match(&0, &ldtk_level.level)
                {
                    *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}

/// During loading the map we should pause the physics
///  to avoid the situation when user may fall from the
///  ground because the ground is not loaded yet
///
/// To avoid this type of bugs we have to deactivate
///  the physics pipeline when we have an event
///  of rendering the map and turn it on when
///  the map has been loaded
fn pause_physics_during_map_load(
    mut level_events: EventReader<LevelEvent>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    for event in level_events.iter() {
        match event {
            LevelEvent::SpawnTriggered(_) => rapier_config.physics_pipeline_active = false,
            LevelEvent::Transformed(_) => rapier_config.physics_pipeline_active = true,
            _ => (),
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fit_camera_inside_current_level)
            .add_system(pause_physics_during_map_load)
            .add_system(spawn_wall_collision)
            .add_system(update_level_selection)
            .register_ldtk_int_cell::<DirtBundle>(CollisionId::Dirt as i32)
            .register_ldtk_int_cell::<LadderBundle>(CollisionId::Ladder as i32)
            .register_ldtk_int_cell::<StoneBundle>(CollisionId::Stone as i32);
    }
}
