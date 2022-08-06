use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    ApplicationState, Climbable, Climber, Health, MovementAnimation, MovementDirection, OnMove,
    PlayerIsDeadEvent, Speed, Sprites,
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
                .with_system(dead)
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
        let sprite_asset_info = &materials.player.pumpkin;

        let sprite_width = sprite_asset_info.width;
        let sprite_height = sprite_asset_info.height;

        let player_direction = MovementDirection::Right;

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
                        MovementDirection::Left => true,
                        MovementDirection::Right => false,
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayerName(PlayerNames::Pumpkin))
            .insert(player_direction)
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
            .insert(Health {
                current: 10,
                max: 10,
            })
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
        if move_delta_x != 0.0 {
            on_move.0 = true;
        } else {
            on_move.0 = false;
        }

        // Change player direction
        if move_delta_x > 0.0 {
            *direction = MovementDirection::Right;
            sprite.flip_x = false;
        } else if move_delta_x < 0.0 {
            *direction = MovementDirection::Left;
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
        if !on_move.0 {
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
                    if rigid_bodies.get(*player).is_ok()
                        && player == &ground_sensor.ground_detection_entity
                    {
                        ground_sensor.intersecting_ground_entities.insert(*ground);
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
            ground_detection.on_ground = !ground_sensor.intersecting_ground_entities.is_empty();
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

                    &materials.player.dragon
                }
                PlayerNames::Dragon => {
                    player_name.0 = PlayerNames::Pumpkin;

                    &materials.player.pumpkin
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

fn dead(
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
    mut kill_the_player_event: EventWriter<PlayerIsDeadEvent>,
) {
    if let Ok(health) = player_query.get_single() {
        // If we reach player health lower or equal to 0 we have to change
        //  state to Death
        if health.current <= 0 {
            kill_the_player_event.send(PlayerIsDeadEvent);
        }
    }
}

#[cfg(test)]
mod player_tests {
    use crate::player::{player_jump, GroundDetection};
    use crate::tests::sprites_textures::prepare_sprites;
    use crate::{
        player::{spawn_player, Player, PlayerBundle},
        Speed,
    };
    use crate::{Climber, Health, MovementDirection, PlayerIsDeadEvent};
    use bevy::ecs::event::Events;
    use bevy::prelude::*;
    use bevy_ecs_ldtk::prelude::*;
    use bevy_rapier2d::prelude::{ExternalImpulse, GravityScale, Velocity};

    use super::{dead, ignore_gravity_during_climbing, player_movement};

    #[test]
    fn should_spawn_a_player_with_speed() {
        let mut app = App::new();

        app.insert_resource(prepare_sprites())
            .add_system(spawn_player)
            .register_ldtk_entity::<PlayerBundle>("Player");

        let player_id = app
            .world
            .spawn()
            .insert(Player)
            .insert(Transform::from_xyz(0.0, 0.0, 1.0))
            .id();

        app.update();

        assert!(app.world.get::<Player>(player_id).is_some());
        assert!(app.world.get::<Speed>(player_id).is_some());
    }

    #[test]
    fn should_spawn_a_player_with_zero_velocity() {
        let mut app = App::new();

        app.insert_resource(prepare_sprites())
            .add_system(spawn_player)
            .register_ldtk_entity::<PlayerBundle>("Player");

        let player_id = app
            .world
            .spawn()
            .insert(Player)
            .insert(Transform::from_xyz(0.0, 0.0, 1.0))
            .id();

        app.update();

        let player_velocity = app.world.get::<Velocity>(player_id).cloned();

        assert_eq!(player_velocity, Some(Velocity::zero()));
    }

    #[test]
    fn should_increase_player_speed_by_keyboard() {
        let mut app = App::new();

        app.insert_resource(prepare_sprites())
            .add_system(spawn_player)
            .add_system(player_movement)
            .register_ldtk_entity::<PlayerBundle>("Player");

        let player_id = app
            .world
            .spawn()
            .insert(Player)
            .insert(Transform::from_xyz(0.0, 0.0, 1.0))
            .id();

        let input = Input::<KeyCode>::default();
        app.insert_resource(input);

        // We should call first update to spawn an entity
        // Because we don't know which system will run first
        //  `spawn_player` or `player_movement` we should call first
        // update and let it be
        app.update();

        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::Right);
        app.insert_resource(input);

        app.update();

        let player_velocity = app.world.get::<Velocity>(player_id).cloned();
        let player_speed = app
            .world
            .get::<Speed>(player_id)
            .expect("Player must have a speed");

        assert_eq!(
            player_velocity,
            Some(Velocity::linear(Vec2::new(player_speed.0, 0.0)))
        );
    }

    #[test]
    fn should_change_movement_direction_on_move() {
        let mut app = App::new();

        app.insert_resource(prepare_sprites())
            .add_system(spawn_player)
            .add_system(player_movement)
            .register_ldtk_entity::<PlayerBundle>("Player");

        let player_id = app
            .world
            .spawn()
            .insert(Player)
            .insert(Transform::from_xyz(0.0, 0.0, 1.0))
            .id();

        let input = Input::<KeyCode>::default();
        app.insert_resource(input);

        // We should call first update to spawn an entity
        // Because we don't know which system will run first
        //  `spawn_player` or `player_movement` we should call first
        // update and let it be
        app.update();

        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::Right);
        app.insert_resource(input);

        app.update();

        assert_eq!(
            app.world.get::<MovementDirection>(player_id).cloned(),
            Some(MovementDirection::Right)
        );

        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::Left);
        app.insert_resource(input);

        app.update();

        assert_eq!(
            app.world.get::<MovementDirection>(player_id).cloned(),
            Some(MovementDirection::Left)
        );

        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::Down);
        app.insert_resource(input);

        app.update();

        // Player should keep the same direction (Left or Right)
        //  even if we press `Down` or any other key code
        //  except left or right
        assert_eq!(
            app.world.get::<MovementDirection>(player_id).cloned(),
            Some(MovementDirection::Left)
        );
    }

    #[test]
    fn player_should_not_jump_without_ground_detection() {
        let mut app = App::new();

        app.insert_resource(prepare_sprites())
            .add_system(spawn_player)
            .add_system(player_jump)
            .register_ldtk_entity::<PlayerBundle>("Player");

        let player_id = app
            .world
            .spawn()
            .insert(Player)
            .insert(Transform::from_xyz(0.0, 0.0, 1.0))
            .id();

        let input = Input::<KeyCode>::default();
        app.insert_resource(input);

        // We should call first update to spawn an entity
        // Because we don't know which system will run first
        //  `spawn_player` or `player_movement` we should call first
        // update and let it be
        app.update();

        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::Space);
        app.insert_resource(input);

        let mut ground_detection = app
            .world
            .get_mut::<GroundDetection>(player_id)
            .expect("Should have external impulse");

        // Change ground detection to `false` to NOT be able to jump
        //  without ground detection player can't jump
        ground_detection.on_ground = false;

        app.update();

        let impulse = app
            .world
            .get::<ExternalImpulse>(player_id)
            .expect("Should have external impulse");

        // We shouldn't have an impulse - this means that player don't jump
        assert_eq!(impulse.impulse, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn player_should_jump_by_space() {
        let mut app = App::new();

        app.insert_resource(prepare_sprites())
            .add_system(spawn_player)
            .add_system(player_jump)
            .register_ldtk_entity::<PlayerBundle>("Player");

        let player_id = app
            .world
            .spawn()
            .insert(Player)
            .insert(Transform::from_xyz(0.0, 0.0, 1.0))
            .id();

        let input = Input::<KeyCode>::default();
        app.insert_resource(input);

        // We should call first update to spawn an entity
        // Because we don't know which system will run first
        //  `spawn_player` or `player_movement` we should call first
        // update and let it be
        app.update();

        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::Space);
        app.insert_resource(input);

        let mut ground_detection = app
            .world
            .get_mut::<GroundDetection>(player_id)
            .expect("Should have external impulse");

        // Change ground detection to `true` to be able to jump
        //  without ground detection player can't jump
        ground_detection.on_ground = true;

        app.update();

        let impulse = app
            .world
            .get::<ExternalImpulse>(player_id)
            .expect("Should have external impulse");

        assert_eq!(impulse.impulse, Vec2::new(0.0, 35.0));
    }

    #[test]
    fn player_should_dead_when_the_health_is_gone() {
        let mut app = App::new();

        app.insert_resource(prepare_sprites())
            .add_system(spawn_player)
            .add_system(dead)
            .register_ldtk_entity::<PlayerBundle>("Player");

        let player_id = app
            .world
            .spawn()
            .insert(Player)
            .insert(Transform::from_xyz(0.0, 0.0, 1.0))
            .id();

        app.add_event::<PlayerIsDeadEvent>();

        app.update();

        let mut player_health = app
            .world
            .get_mut::<Health>(player_id)
            .expect("Player must have a health component");

        // Set playear health to 0 to kill it
        player_health.current = 0;

        app.update();

        let player_died_events = app.world.resource::<Events<PlayerIsDeadEvent>>();
        let mut player_died_reader = player_died_events.get_reader();
        let player_died = player_died_reader.iter(player_died_events).next();

        assert!(player_died.is_some());
    }

    #[test]
    fn should_disable_gravity_during_climbing() {
        let mut app = App::new();

        app.insert_resource(prepare_sprites())
            .add_system(spawn_player)
            .add_system(ignore_gravity_during_climbing)
            .register_ldtk_entity::<PlayerBundle>("Player");

        let player_id = app
            .world
            .spawn()
            .insert(Player)
            .insert(Transform::from_xyz(0.0, 0.0, 1.0))
            .id();

        app.update();

        let mut player_climber = app
            .world
            .get_mut::<Climber>(player_id)
            .expect("Player must have a climber component");

        // Add climbing as `true` to disable the gravity
        player_climber.climbing = true;

        app.update();

        let player_gravity = app
            .world
            .get::<GravityScale>(player_id)
            .expect("Player must have a gravity component");

        // Gravity should be 0.0 when player is climbing
        assert_eq!(player_gravity.0, 0.0);
    }
}
