use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{map::Wall, physics::GRAPHICS_TO_PHYSICS, Speed};
use std::collections::{HashMap, HashSet};

#[derive(Component, Inspectable)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(player_collision);
    }
}

fn spawn_player(mut commands: Commands) {
    // Set the gravity as zero
    // rapier_config.gravity = Vector::zeros();

    let x = 150.0;
    let y = 150.0;

    let sprite_size = 10.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec2::new(x / GRAPHICS_TO_PHYSICS, y / GRAPHICS_TO_PHYSICS).into(),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(
                sprite_size / GRAPHICS_TO_PHYSICS / 2.0,
                sprite_size / GRAPHICS_TO_PHYSICS / 2.0,
            )
            .into(),
            material: ColliderMaterial {
                restitution: 0.7,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(Player)
        // .insert(ColliderPositionSync::Discrete)
        .insert(RigidBodyPositionSync::Discrete)
        // .insert(ColliderDebugRender::with_id(1))
        .insert(Speed(5.0));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut RigidBodyVelocityComponent), With<Player>>,
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

        velocity.linvel = Vector::new(direction_x * speed.0, direction_y * speed.0);
    }
}

fn player_collision(
    player_query: Query<&Transform, With<Player>>,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    // levels: Res<Assets<LdtkLevel>>,
) {
    let player = player_query.get_single();

    if let Ok(player_transform) = player {
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
    }
}
