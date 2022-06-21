use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    ApplicationState, Climbable, Climber, MovementDirection, MovementTendency, Speed, Sprites,
};

#[derive(Debug, Inspectable)]
enum PlayerNames {
    Pumpkin,
    Dragon,
}

#[derive(Component, Debug, Inspectable)]
struct PlayerName(PlayerNames);

#[derive(Component, Default, Inspectable)]
pub struct Player;

#[derive(Component)]
struct MovementAnimation {
    timer: Timer,
}

#[derive(Component, Default, Inspectable)]
/// Describes that entity on move or not
struct OnMove(bool);

#[derive(Component, Debug, Inspectable)]
struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    pub player: Player,

    // #[sprite_sheet_bundle]
    // #[bundle]
    // pub sprite_bundle: SpriteSheetBundle,
    #[worldly]
    pub worldly: Worldly,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_player)
                .with_system(player_movement)
                .with_system(player_movement_animation)
                .with_system(player_jump)
                .with_system(detect_climb)
                .with_system(ignore_gravity_during_climbing)
                .with_system(change_player_texture)
                .with_system(spawn_ground_sensor)
                .with_system(ground_detection)
                .into(),
        )
        .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

fn spawn_player(
    mut commands: Commands,
    materials: Res<Sprites>,
    player_query: Query<(Entity, &Transform), Added<Player>>,
) {
    if let Ok((player_entity, transform)) = player_query.get_single() {
        let sprite_asset_info = &materials.pumpkin;

        let sprite_width = sprite_asset_info.width;
        let sprite_height = sprite_asset_info.height;

        let player_direction = MovementTendency::Right;

        commands
            .entity(player_entity)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(sprite_width / 2.0, sprite_height / 2.0))
            // Add Velocity component to iterate via it but with zero value
            .insert(Velocity::zero())
            .insert(ExternalImpulse::default())
            .insert(Friction::new(0.01))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(GravityScale(3.0))
            .insert(ColliderMassProperties::Density(1.0))
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: sprite_asset_info.texture.clone(),
                // Take the Wordly coordinates to place
                // spritesheet correctly
                transform: Transform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    // scale: transform.scale,
                    scale: Vec3::new(0.8, 0.8, 1.0),
                },
                sprite: TextureAtlasSprite {
                    flip_x: match &player_direction {
                        MovementTendency::Left => true,
                        MovementTendency::Right => false,
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayerName(PlayerNames::Pumpkin))
            .insert(MovementDirection(player_direction))
            .insert(MovementAnimation {
                timer: Timer::from_seconds(0.1, true),
            })
            .insert(OnMove(false))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Climber {
                intersaction_elements: HashSet::new(),
                climbing: false,
            })
            .insert(GroundDetection { on_ground: false })
            .insert(Speed(120.0));
    }
}

/// System which manipulates with
///  moving to left / right (change `x` velocity)
///  climbing (change `y` velocity)
fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &Speed,
            &mut OnMove,
            &mut Climber,
            &mut MovementDirection,
            &mut TextureAtlasSprite,
            &mut Velocity,
        ),
        With<Player>,
    >,
) {
    if let Ok((speed, mut on_move, mut climber, mut direction, mut sprite, mut velocity)) =
        query.get_single_mut()
    {
        /* Moving logic */
        let direction_x = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        let move_delta_x = direction_x * speed.0;

        // Update player velocity
        velocity.linvel.x = move_delta_x;

        // Change `OnMove` component
        if move_delta_x > 0.0 || move_delta_x < 0.0 {
            on_move.0 = true;
        } else {
            on_move.0 = false;
        }

        // Change player direction
        if move_delta_x > 0.0 {
            direction.0 = MovementTendency::Right;
            sprite.flip_x = false;
        } else if move_delta_x < 0.0 {
            direction.0 = MovementTendency::Left;
            sprite.flip_x = true;
        }

        /* Climbing logic */
        if climber.intersaction_elements.is_empty() {
            climber.climbing = false;
        } else if keyboard.just_pressed(KeyCode::Up) || keyboard.just_pressed(KeyCode::Down) {
            climber.climbing = true;
        }

        if climber.climbing {
            let direction_y = if keyboard.pressed(KeyCode::Up) {
                1.0
            } else if keyboard.pressed(KeyCode::Down) {
                -1.0
            } else {
                0.0
            };

            velocity.linvel.y = direction_y * speed.0;
        }
    }
}

fn player_jump(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut ExternalImpulse, &mut Climber, &GroundDetection), With<Player>>,
) {
    if let Ok((mut external_impulse, mut climber, ground_detection)) = player_query.get_single_mut()
    {
        if keyboard.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing)
        {
            external_impulse.impulse = Vec2::new(0.0, 35.0);
            climber.climbing = false;
        }
    }
}

fn player_movement_animation(
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<
        (
            &OnMove,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut MovementAnimation,
        ),
        With<Player>,
    >,
) {
    for (on_move, mut sprite, texture_atlas_handle, mut movement_animation) in query.iter_mut() {
        // If the player is not on move
        //  set the first sprite which is equal to
        //  player default state and do nothing
        if on_move.0 == false {
            sprite.index = 0;

            return;
        }

        movement_animation.timer.tick(time.delta());

        if movement_animation.timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index += 1;

            if sprite.index == texture_atlas.textures.len() {
                // Loop the animation
                sprite.index = 0;
            }
        }
    }
}

fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_query: Query<(Entity, &Collider, &Transform), Added<GroundDetection>>,
) {
    for (entity, collider, transform) in detect_ground_query.iter() {
        if let Some(cuboid) = collider.as_cuboid() {
            let half_extents = &cuboid.half_extents();

            let detector_shape = Collider::cuboid(half_extents.x, half_extents.y);
            let sensor_translation = Vec3::new(0.0, -half_extents.y, 0.0) / transform.scale;

            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn()
                    .insert(Sensor(true))
                    .insert(detector_shape)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    // We should make the weight of this rigid body as 0 because
                    //  otherwise it will affect the user but we want to make it
                    //  just as trigger for ground detection reaction
                    .insert(ColliderMassProperties::Density(0.0))
                    // .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        }
    }
}

fn ground_detection(
    mut ground_detectors: Query<&mut GroundDetection>,
    mut ground_sensors: Query<(Entity, &mut GroundSensor)>,
    mut collisions: EventReader<CollisionEvent>,
    rigid_bodies: Query<&RigidBody>,
) {
    for (_, mut ground_sensor) in ground_sensors.iter_mut() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(ground, player, _) => {
                    if let Ok(_) = rigid_bodies.get(*player) {
                        if player == &ground_sensor.ground_detection_entity {
                            ground_sensor.intersecting_ground_entities.insert(*ground);
                        }
                    }
                }
                CollisionEvent::Stopped(ground, player, _) => {
                    if player == &ground_sensor.ground_detection_entity {
                        ground_sensor.intersecting_ground_entities.remove(ground);
                    }
                }
            }
        }

        if let Ok(mut ground_detection) =
            ground_detectors.get_mut(ground_sensor.ground_detection_entity)
        {
            ground_detection.on_ground = ground_sensor.intersecting_ground_entities.len() > 0;
        }
    }
}

fn detect_climb(
    mut climbers: Query<&mut Climber>,
    climbables: Query<&Climbable>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.iter() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let Ok(mut climber) = climbers.get_mut(*collider_a) {
                    if climbables.get(*collider_b).is_ok() {
                        climber.intersaction_elements.insert(*collider_b);
                    }
                }

                if let Ok(mut climber) = climbers.get_mut(*collider_b) {
                    if climbables.get(*collider_a).is_ok() {
                        climber.intersaction_elements.insert(*collider_a);
                    }
                }
            }
            CollisionEvent::Stopped(collider_a, collider_b, _) => {
                if let Ok(mut climber) = climbers.get_mut(*collider_a) {
                    if climbables.get(*collider_b).is_ok() {
                        climber.intersaction_elements.remove(collider_b);
                    }
                }

                if let Ok(mut climber) = climbers.get_mut(*collider_b) {
                    if climbables.get(*collider_a).is_ok() {
                        climber.intersaction_elements.remove(collider_a);
                    }
                }
            }
        }
    }
}

fn ignore_gravity_during_climbing(
    mut query: Query<(&mut GravityScale, &Climber), Changed<Climber>>,
) {
    for (mut gravity, climber) in query.iter_mut() {
        if climber.climbing {
            gravity.0 = 0.0;
        } else {
            gravity.0 = 3.0;
        }
    }
}

fn change_player_texture(
    keyboard: Res<Input<KeyCode>>,
    materials: Res<Sprites>,
    mut player_query: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
            &mut Collider,
            &mut PlayerName,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut texture_atlas, mut sprite, mut collider, mut player_name, mut transform)) =
        player_query.get_single_mut()
    {
        // If player want to change player texture by pressing `Q` character
        if keyboard.just_pressed(KeyCode::Q) {
            let sprite_asset_info = match &player_name.0 {
                PlayerNames::Pumpkin => {
                    player_name.0 = PlayerNames::Dragon;

                    &materials.dragon
                }
                PlayerNames::Dragon => {
                    player_name.0 = PlayerNames::Pumpkin;

                    &materials.pumpkin
                }
            };

            // Change the texture
            *texture_atlas = sprite_asset_info.texture.clone();

            // Reset the animation
            sprite.index = 0;

            // Change the object bounds
            *collider = Collider::cuboid(
                sprite_asset_info.width / 2.0,
                sprite_asset_info.height / 2.0,
            );

            // Push the player a little bit up
            //  to avoid the problem when the one sprite
            //  is less than another sprite and we may
            //  have a collision mismatch with the ground
            transform.translation.y += 8.0;
        }
    }
}
