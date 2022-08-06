use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{enemy::Enemy, player::Player, ApplicationState, Health};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(combat_by_keyboard)
                .with_system(combat_interaction_detection)
                .with_system(impulse_by_keyboard)
                .into(),
        );
    }
}

fn impulse_by_keyboard(
    keyboard: Res<Input<KeyCode>>,
    mut player_impulse: Query<&mut ExternalImpulse, With<Player>>,
) {
    for mut impulse in player_impulse.iter_mut() {
        if keyboard.just_pressed(KeyCode::I) {
            impulse.impulse = Vec2::new(-300.0, 0.0);
        }
    }

    // for mut force in player_force.iter_mut() {
    //     if keyboard.just_pressed(KeyCode::I) {
    //         force.force = Vec2::new(100.0, 0.0);
    //     }
    // }
}

fn combat_by_keyboard(
    keyboard: Res<Input<KeyCode>>,
    mut health_query: Query<&mut Health, With<Player>>,
) {
    for mut health in health_query.iter_mut() {
        if keyboard.just_pressed(KeyCode::H) && health.current != 0 {
            health.current -= 1;
        }
    }
}

fn combat_interaction_detection(
    mut collisions: EventReader<CollisionEvent>,
    mut player_query: Query<
        (&mut Health, &mut ExternalImpulse, &GlobalTransform),
        (With<Player>, Without<Enemy>),
    >,
    mut enemy_query: Query<
        (&mut Health, &mut ExternalImpulse, &GlobalTransform),
        (With<Enemy>, Without<Player>),
    >,
) {
    for collision in collisions.iter() {
        // Now impulse by `x` axis looks so sharp. I have to understand how to make it
        //  smooth and only after that it would be possible to increase this value
        //  up to 100 or 300 depends on smoothness
        let impulse_force = 0.0;

        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let Ok((mut player_health, mut player_impulse, player_transform)) =
                    player_query.get_mut(*collider_a)
                {
                    if let Ok((mut enemy_health, mut enemy_impulse, enemy_transform)) =
                        enemy_query.get_mut(*collider_b)
                    {
                        // If player below enemy - it should take hit. Otherwise - enemy
                        if player_transform.translation.y < enemy_transform.translation.y {
                            if player_health.current > 0 {
                                player_health.current -= 1;
                            }
                        } else if enemy_health.current > 0 {
                            enemy_health.current -= 1;
                        }

                        // We should push player on left - otherwise - on right
                        if player_transform.translation.x < enemy_transform.translation.x {
                            player_impulse.impulse = Vec2::new(-impulse_force, 0.0);
                            enemy_impulse.impulse = Vec2::new(impulse_force, 0.0);
                        } else {
                            player_impulse.impulse = Vec2::new(impulse_force, 0.0);
                            enemy_impulse.impulse = Vec2::new(-impulse_force, 0.0);
                        }

                        continue;
                    }
                }

                if let Ok((mut player_health, mut player_impulse, player_transform)) =
                    player_query.get_mut(*collider_b)
                {
                    if let Ok((mut enemy_health, mut enemy_impulse, enemy_transform)) =
                        enemy_query.get_mut(*collider_a)
                    {
                        // If player below enemy - it should take hit. Otherwise - enemy
                        if player_transform.translation.y - enemy_transform.translation.y < 5.0 {
                            if player_health.current > 0 {
                                player_health.current -= 1;
                            }
                        } else if enemy_health.current > 0 {
                            enemy_health.current -= 1;
                        }

                        // We should push player on left - otherwise - on right
                        if player_transform.translation.x < enemy_transform.translation.x {
                            player_impulse.impulse = Vec2::new(-impulse_force, 0.0);
                            enemy_impulse.impulse = Vec2::new(impulse_force, 0.0);
                        } else {
                            player_impulse.impulse = Vec2::new(impulse_force, 0.0);
                            enemy_impulse.impulse = Vec2::new(-impulse_force, 0.0);
                        }

                        continue;
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
