use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::ApplicationState;

pub struct TutorialPlugin;

/// A Tutorial component which has
///  many UI's entities
#[derive(Component, Default)]
pub struct Tutorial {
    pub ui_entities: HashSet<Entity>,
}

#[derive(Component, Debug, Eq, PartialEq, Inspectable)]
pub enum TutorialType {
    Movement,
    Climbing,
}

impl Default for TutorialType {
    fn default() -> Self {
        Self::Movement
    }
}

#[derive(Component, Debug, Default, Eq, PartialEq, Inspectable)]
pub struct TutorialPassed(pub bool);

impl From<EntityInstance> for TutorialType {
    fn from(entity_instance: EntityInstance) -> Self {
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|tutor_type| tutor_type.identifier == *"tutorial_type")
        {
            let tutorial_type_option = match &field_instance.value {
                FieldValue::Enum(val) => match val.as_deref() {
                    Some("Movement") => TutorialType::Movement,
                    Some("Climbing") => TutorialType::Climbing,
                    _ => {
                        panic!("This is impossible option");
                    }
                },
                _ => {
                    panic!("Coundn't find any covered enum");
                }
            };

            return tutorial_type_option;
        }

        panic!("Cound'n find any available options. Please check Ldtk map `tutarial_type` enum");
    }
}

impl From<EntityInstance> for TutorialPassed {
    fn from(_entity_instance: EntityInstance) -> Self {
        Self(false)
    }
}

#[derive(Bundle, LdtkEntity)]
struct TutorialBundle {
    pub tutorial: Tutorial,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[from_entity_instance]
    passed: TutorialPassed,

    #[from_entity_instance]
    tutorial_type: TutorialType,
}

fn spawn_tutorial(
    mut commands: Commands,
    tutorials_query: Query<(Entity, &Transform), Added<Tutorial>>,
) {
    for (tutorial_entity, tutorial_transform) in tutorials_query.iter() {
        commands
            .entity(tutorial_entity)
            .insert(Sensor(true))
            .insert(Collider::cuboid(30.0, 8.0))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    // color: Color::rgb(0.1, 0.1, 0.1),
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..Default::default()
                },
                transform: *tutorial_transform,
                ..Default::default()
            });
    }
}

fn tutorial_interaction_detection(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut tutorials_query: Query<(Entity, &mut TutorialPassed, &Tutorial), With<Tutorial>>,
) {
    for collision in collisions.iter() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let Ok((_, mut tutorial_active, _)) = tutorials_query.get_mut(*collider_a) {
                    tutorial_active.0 = true;
                } else if let Ok((_, mut tutorial_active, _)) = tutorials_query.get_mut(*collider_b)
                {
                    tutorial_active.0 = true;
                }
            }

            CollisionEvent::Stopped(collider_a, collider_b, _) => {
                if let Ok((tutorial_entity, _, tutorial)) = tutorials_query.get(*collider_a) {
                    // Dispawn all UI related entities into this entity
                    for &tutorial_ui_id in tutorial.ui_entities.iter() {
                        commands.entity(tutorial_ui_id).despawn_recursive();
                    }

                    // And despawn the entity itself
                    commands.entity(tutorial_entity).despawn();
                } else if let Ok((tutorial_entity, _, tutorial)) = tutorials_query.get(*collider_b)
                {
                    // Dispawn all UI related entities into this entity
                    for &tutorial_ui_id in tutorial.ui_entities.iter() {
                        commands.entity(tutorial_ui_id).despawn_recursive();
                    }

                    // And despawn the entity itself
                    commands.entity(tutorial_entity).despawn();
                }
            }
        }
    }
}

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_tutorial)
                .with_system(tutorial_interaction_detection)
                .into(),
        )
        .register_ldtk_entity::<TutorialBundle>("Tutorial");
    }
}
