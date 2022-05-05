use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{map::Wall, Speed};
use std::collections::{HashMap, HashSet};

#[derive(Component, Inspectable)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_floor)
            .add_system(player_collision)
            .add_system(player_movement);
    }
}

fn spawn_floor(mut commands: Commands) {
    let sprite_width = 200.0;
    let sprite_height = 10.0;

    commands
        .spawn()
        .insert(Collider::cuboid(sprite_width / 2.0, sprite_height / 2.0))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(sprite_width, sprite_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(400.0, 200.0, 2.0),
            ..Default::default()
        });
}

fn spawn_player(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // fn spawn_player(mut commands: Commands) {
    // Set the gravity as zero
    rapier_config.gravity = Vec2::ZERO;

    let x = 150.0;
    let y = 150.0;

    let sprite_size = 10.0;

    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0))
        .insert(Velocity::zero())
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(0.5))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 2.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(200.0));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    if let Ok((speed, mut velocity)) = query.get_single_mut() {
        // Represent (x, y) coordinates
        let direction_x = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        let direction_y = if keyboard.pressed(KeyCode::Up) {
            1.0
        } else if keyboard.pressed(KeyCode::Down) {
            -1.0
        } else {
            0.0
        };

        velocity.linvel = Vec2::new(direction_x * speed.0, direction_y * speed.0);
        velocity.angvel = 0.0;
    }
}

fn player_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
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

    let player = player_query.get_single();

    if let Ok(_player_transform) = player {
        // for grid_coords in wall_query.iter() {
        //     if player_transform.translation.x as i32 == grid_coords.x
        //         && player_transform.translation.y as i32 == grid_coords.y
        //     {
        //         println!("Collision has been detected!");
        //     }
        // }

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
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.0,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.0,
                                0.0,
                            ))
                            .insert(GlobalTransform::default())
                            .insert(Parent(level_entity));
                    }
                }
            }
        }
    }
}
