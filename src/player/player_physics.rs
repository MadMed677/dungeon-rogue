use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{map::WallCollision, ron_parsers::GameTextures, ApplicationState, PlayerIsDeadEvent};

use crate::common::{
    Attackable, Attacks, Climbable, Climber, Health, MovementDirection, OnMove, Speed,
};

#[derive(Component, Default, Inspectable)]
pub struct Player;

#[derive(Component, Debug, Inspectable)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component, Debug, Inspectable)]
pub struct SideDetector {
    pub on_side: bool,
}
#[derive(Component)]
struct GroundSensor {
    pub detection_entity: Entity,
    pub intersecting_entities: HashSet<Entity>,
}

#[derive(Component)]
pub struct SideSensor {
    pub detection_entity: Entity,
    pub intersecting_entities: HashSet<Entity>,
}

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    pub player: Player,

    #[worldly]
    pub worldly: Worldly,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_player)
                .with_system(player_movement)
                .with_system(player_jump)
                .with_system(detect_climb)
                .with_system(ignore_gravity_during_climbing)
                .with_system(spawn_ground_sensor)
                .with_system(spawn_side_sensor)
                .with_system(ground_detection)
                .with_system(wall_detection)
                .with_system(dead)
                .into(),
        )
        .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

fn spawn_player(
    mut commands: Commands,
    materials: Res<GameTextures>,
    player_query: Query<(Entity, &Transform), Added<Player>>,
) {
    if let Ok((player_entity, transform)) = player_query.get_single() {
        let sprite_asset_info = &materials.player.idle;

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
            .insert(Attacks(false))
            .insert(Attackable)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: sprite_asset_info.texture.clone(),
                // Take the Wordly coordinates to place
                // spritesheet correctly
                transform: Transform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    // scale: transform.scale,
                    scale: Vec3::new(0.7, 0.7, 1.0),
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
            .insert(player_direction)
            .insert(OnMove(false))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Climber {
                intersaction_elements: HashSet::new(),
                climbing: false,
            })
            .insert(GroundDetection { on_ground: false })
            .insert(SideDetector { on_side: false })
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
        // But we need to change `OnMove` component
        //  only if we need it. Do not update OnMove
        //  component every time. Otherwise `Changed<OnMove>`
        //  signal will be called everytime
        if move_delta_x != 0.0 {
            if !on_move.0 {
                on_move.0 = true;
            }
        } else if on_move.0 {
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
            if climber.climbing {
                climber.climbing = false;
            }
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
            external_impulse.impulse = Vec2::new(0.0, 65.0);
            climber.climbing = false;
        }
    }
}

/// Spawn a ground sensor under the player
/// It helps to understand player collision with
///  the ground / enemies and other entities which
///  are placed under the current entity
fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_query: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, collider) in detect_ground_query.iter() {
        if let Some(cuboid) = collider.as_cuboid() {
            let half_extents = &cuboid.half_extents();

            let detector_shape = Collider::cuboid(half_extents.x - 2.0, 2.0);
            let sensor_translation = Vec3::new(0.0, -half_extents.y, 0.0);

            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn()
                    .insert(Sensor)
                    .insert(detector_shape)
                    // .insert(detector_shape)
                    .insert(Transform::from_translation(sensor_translation))
                    // We should make the weight of this rigid body as 0 because
                    //  otherwise it will affect the user but we want to make it
                    //  just as trigger for ground detection reaction
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(ColliderMassProperties::Density(0.0))
                    .insert(GroundSensor {
                        detection_entity: entity,
                        intersecting_entities: HashSet::new(),
                    });
            });
        }
    }
}

/// Spawns a side sensor from the left and right sides of an entity
fn spawn_side_sensor(
    mut commands: Commands,
    detect_side_query: Query<(Entity, &Collider), Added<SideDetector>>,
) {
    for (entity, collider) in detect_side_query.iter() {
        if let Some(cuboid) = collider.as_cuboid() {
            let half_extents = &cuboid.half_extents();

            // Create one huge side sensor which will be more by `x` axis
            //  but less by `y` axis
            let detector_shape = Collider::cuboid(half_extents.x + 4.0, half_extents.y - 3.0);

            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn()
                    .insert(Sensor)
                    .insert(detector_shape.clone())
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(ColliderMassProperties::Density(0.0))
                    .insert(SideSensor {
                        detection_entity: entity,
                        intersecting_entities: HashSet::new(),
                    });
            });

            return;
        }
    }
}

fn wall_detection(
    mut side_detectors: Query<&mut SideDetector>,
    side_sensors: Query<(Entity, &SideSensor)>,
    mut collisions: EventReader<CollisionEvent>,
    walls_query: Query<Entity, With<WallCollision>>,
) {
    for (sensor_entity, sensor) in side_sensors.iter() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(collision_a, collision_b, _) => {
                    if *collision_b == sensor_entity && walls_query.get(*collision_a).is_ok() {
                        if let Ok(mut detector) = side_detectors.get_mut(sensor.detection_entity) {
                            detector.on_side = true;
                            println!("[start] Collision with the wall");
                        }
                    }
                }
                CollisionEvent::Stopped(collision_a, collision_b, _) => {
                    if *collision_b == sensor_entity && walls_query.get(*collision_a).is_ok() {
                        if let Ok(mut detector) = side_detectors.get_mut(sensor.detection_entity) {
                            detector.on_side = false;
                            println!("[stop] Collision with the wall");
                        }
                    }
                }
            }
        }
    }
}

fn ground_detection(
    mut ground_detectors: Query<(Entity, &mut GroundDetection)>,
    mut ground_sensors: Query<(Entity, &mut GroundSensor)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for (ground_sensor_entity, mut ground_sensor) in ground_sensors.iter_mut() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(collision_a, collision_b, _) => {
                    if (*collision_b == ground_sensor_entity)
                        && ground_detectors
                            .get_mut(ground_sensor.detection_entity)
                            .is_ok()
                    {
                        ground_sensor.intersecting_entities.insert(*collision_a);

                        return;
                    }

                    if (*collision_a == ground_sensor_entity)
                        && ground_detectors
                            .get_mut(ground_sensor.detection_entity)
                            .is_ok()
                    {
                        ground_sensor.intersecting_entities.insert(*collision_b);

                        return;
                    }
                }
                CollisionEvent::Stopped(collision_a, collision_b, _) => {
                    if (*collision_b == ground_sensor_entity)
                        && ground_detectors
                            .get_mut(ground_sensor.detection_entity)
                            .is_ok()
                    {
                        ground_sensor.intersecting_entities.remove(collision_a);

                        return;
                    }

                    if (*collision_a == ground_sensor_entity)
                        && ground_detectors
                            .get_mut(ground_sensor.detection_entity)
                            .is_ok()
                    {
                        ground_sensor.intersecting_entities.remove(collision_b);

                        return;
                    }
                }
            }
        }

        if let Ok((_, mut ground_detection)) =
            ground_detectors.get_mut(ground_sensor.detection_entity)
        {
            let next_on_ground = !ground_sensor.intersecting_entities.is_empty();

            if ground_detection.on_ground != next_on_ground {
                ground_detection.on_ground = next_on_ground;
            }
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

                        continue;
                    }
                }

                if let Ok(mut climber) = climbers.get_mut(*collider_b) {
                    if climbables.get(*collider_a).is_ok() {
                        climber.intersaction_elements.insert(*collider_a);

                        continue;
                    }
                }
            }
            CollisionEvent::Stopped(collider_a, collider_b, _) => {
                if let Ok(mut climber) = climbers.get_mut(*collider_a) {
                    if climbables.get(*collider_b).is_ok() {
                        climber.intersaction_elements.remove(collider_b);

                        continue;
                    }
                }

                if let Ok(mut climber) = climbers.get_mut(*collider_b) {
                    if climbables.get(*collider_a).is_ok() {
                        climber.intersaction_elements.remove(collider_a);

                        continue;
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
    use crate::common::{Climber, Health, MovementDirection, Speed};
    use crate::player::player_physics::{player_jump, spawn_player, PlayerBundle};
    use crate::player::GroundDetection;
    use crate::player::Player;
    use crate::tests::sprites_textures::prepare_sprites;
    use crate::PlayerIsDeadEvent;
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

        assert_eq!(impulse.impulse, Vec2::new(0.0, 65.0));
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
