use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{player::Player, ApplicationState, Health};

pub struct OutOfBouncePlugin;

#[derive(Component, Default)]
pub struct DeathOutOfBounce;

#[derive(Bundle, LdtkEntity)]
struct DeadOutOfBounceBundle {
    pub dead: DeathOutOfBounce,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

impl Plugin for OutOfBouncePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_dead_oob)
                .with_system(dead_interaction_detection)
                .into(),
        );
        app.register_ldtk_entity::<DeadOutOfBounceBundle>("Dead");
    }
}

/// Spawn out-of bounce entity which should kill the player
fn spawn_dead_oob(
    mut commands: Commands,
    dead_oob: Query<(Entity, &Transform), Added<DeathOutOfBounce>>,
) {
    for (dead_entity, dead_transform) in dead_oob.iter() {
        commands
            .entity(dead_entity)
            .insert(Sensor(true))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Collider::cuboid(8.0, 8.0))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    // color: Color::Rgba {
                    //     red: 0.1,
                    //     green: 0.1,
                    //     blue: 1.0,
                    //     alpha: 1.0,
                    // },
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..Default::default()
                },
                transform: *dead_transform,
                ..Default::default()
            });
    }
}

fn dead_interaction_detection(
    mut collisions: EventReader<CollisionEvent>,
    mut player_query: Query<&mut Health, With<Player>>,
    death_collision_query: Query<With<DeathOutOfBounce>>,
) {
    for collision in collisions.iter() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let Ok(mut health) = player_query.get_mut(*collider_a) {
                    if let Ok(_) = death_collision_query.get(*collider_b) {
                        health.current = 0;
                    }
                } else if let Ok(mut health) = player_query.get_mut(*collider_b) {
                    if let Ok(_) = death_collision_query.get(*collider_a) {
                        health.current = 0;
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
