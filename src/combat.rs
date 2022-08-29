use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::common::{Attackable, Attacks, Health};
use crate::{
    enemy::Enemy,
    player::{Player, SideDetector, SideSensor},
    ApplicationState, PlayerIsHitEvent,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(combat_interaction_detection)
                .with_system(player_receives_damage)
                .with_system(player_attacks)
                .with_system(attack_detection)
                .into(),
        );
    }
}

fn attack_detection(
    side_detectors: Query<(&Attacks, &GlobalTransform), With<SideDetector>>,
    side_sensors: Query<(Entity, &SideSensor)>,
    mut collisions: EventReader<CollisionEvent>,
    mut attackable_query: Query<
        (&mut Health, &mut ExternalImpulse, &GlobalTransform),
        With<Attackable>,
    >,
) {
    for (side_sensor_entity, side_sensor) in side_sensors.iter() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(collision_a, collision_b, _) => {
                    let offset_x = 50.0;
                    let offset_y = 30.0;

                    let attacker = if let Ok(pl1) = side_detectors.get(side_sensor.detection_entity)
                    {
                        Some(pl1)
                    } else {
                        None
                    };

                    let attackable = if *collision_b == side_sensor_entity {
                        if let Ok(x) = attackable_query.get_mut(*collision_a) {
                            Some(x)
                        } else {
                            None
                        }
                    } else if *collision_a == side_sensor_entity {
                        if let Ok(x) = attackable_query.get_mut(*collision_b) {
                            Some(x)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some((attacks, attacker_transform)) = attacker {
                        if let Some((
                            mut attackable_health,
                            mut attackable_impulse,
                            attackable_transform,
                        )) = attackable
                        {
                            if attacks.0 {
                                attackable_health.current -= 1;

                                // Give an impulse to the left or right depending on
                                //  where is the attacker and where is an attackable entity
                                if attacker_transform.translation().x
                                    < attackable_transform.translation().x
                                {
                                    attackable_impulse.impulse = Vec2::new(offset_x, offset_y);
                                } else {
                                    attackable_impulse.impulse = Vec2::new(-offset_x, offset_y);
                                }
                            }
                        }
                    }
                }
                CollisionEvent::Stopped(_, _, _) => {}
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn combat_interaction_detection(
    mut collisions: EventReader<CollisionEvent>,
    mut player_query: Query<
        (&mut ExternalImpulse, &Collider, &GlobalTransform),
        (With<Player>, Without<Enemy>),
    >,
    mut enemy_query: Query<
        (
            &mut Health,
            &mut ExternalImpulse,
            &Collider,
            &GlobalTransform,
        ),
        (With<Enemy>, Without<Player>),
    >,
    mut hit_the_player_event: EventWriter<PlayerIsHitEvent>,
) {
    for collision in collisions.iter() {
        // Now impulse by `x` axis looks so sharp. I have to understand how to make it
        //  smooth and only after that it would be possible to increase this value
        //  up to 100 or 300 depends on smoothness
        let impulse_force_horizontal = 120.0;
        let impulse_force_vertical = 30.0;

        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                let player = if let Ok(pl1) = player_query.get_mut(*collider_a) {
                    Some(pl1)
                } else if let Ok(pl2) = player_query.get_mut(*collider_b) {
                    Some(pl2)
                } else {
                    None
                };

                let enemy = if let Ok(en1) = enemy_query.get_mut(*collider_b) {
                    Some(en1)
                } else if let Ok(en2) = enemy_query.get_mut(*collider_a) {
                    Some(en2)
                } else {
                    None
                };

                if let Some((mut player_impulse, player_collider, player_transform)) = player {
                    if let Some((
                        mut enemy_health,
                        mut enemy_impulse,
                        enemy_collider,
                        enemy_transform,
                    )) = enemy
                    {
                        let player_half_size = player_collider
                            .as_cuboid()
                            .expect("Player collider must be cuboid")
                            .half_extents();

                        let enemy_half_size = enemy_collider
                            .as_cuboid()
                            .expect("Enemy collider must be cuboid")
                            .half_extents();

                        if ((player_transform.translation().y - player_half_size.y)
                            - (enemy_transform.translation().y + enemy_half_size.y))
                            < -3.0
                        {
                            hit_the_player_event.send(PlayerIsHitEvent(1));
                        } else if enemy_health.current > 0 {
                            enemy_health.current -= 1;
                        }

                        // We should push player on left - otherwise - on right
                        if player_transform.translation().x < enemy_transform.translation().x {
                            player_impulse.impulse =
                                Vec2::new(-impulse_force_horizontal, impulse_force_vertical);
                            enemy_impulse.impulse =
                                Vec2::new(impulse_force_horizontal, impulse_force_vertical);
                        } else {
                            player_impulse.impulse =
                                Vec2::new(impulse_force_horizontal, impulse_force_vertical);
                            enemy_impulse.impulse =
                                Vec2::new(-impulse_force_horizontal, impulse_force_vertical);
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn player_receives_damage(
    mut player_query: Query<&mut Health, With<Player>>,
    mut hit_the_player_event: EventReader<PlayerIsHitEvent>,
) {
    for damage in hit_the_player_event.iter() {
        if let Ok(mut player_health) = player_query.get_single_mut() {
            if player_health.current > 0 {
                player_health.current -= damage.0;
            }
        }
    }
}

fn player_attacks(
    mut player_query: Query<&mut Attacks, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::LShift) {
        if let Ok(mut attacks) = player_query.get_single_mut() {
            if attacks.0 {
                attacks.0 = false;
            } else {
                attacks.0 = true;
            }
        }
    }
}
