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
                .into(),
        );
    }
}

fn combat_by_keyboard(
    keyboard: Res<Input<KeyCode>>,
    mut health_query: Query<&mut Health, With<Player>>,
) {
    for mut health in health_query.iter_mut() {
        if keyboard.just_pressed(KeyCode::H) {
            if health.current != 0 {
                health.current -= 1;
            }
        }
    }
}

fn combat_interaction_detection(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut player_query: Query<&mut Health, With<Player>>,
    enemy_query: Query<With<Enemy>>,
) {
    for collision in collisions.iter() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let Ok(mut player_health) = player_query.get_mut(*collider_a) {
                    if let Ok(_) = enemy_query.get(*collider_b) {
                        if player_health.current > 0 {
                            player_health.current -= 1;
                        }
                    }
                } else if let Ok(mut player_health) = player_query.get_mut(*collider_b) {
                    if let Ok(_) = enemy_query.get(*collider_a) {
                        if player_health.current > 0 {
                            player_health.current -= 1;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
