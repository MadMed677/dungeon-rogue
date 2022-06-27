use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{ApplicationState, Sprites};

pub struct EnemyPlugin;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Debug, Eq, PartialEq, Inspectable)]
pub enum EnemyType {
    Durt,
    LongHair,
}

impl Default for EnemyType {
    fn default() -> Self {
        Self::Durt
    }
}

impl From<EntityInstance> for EnemyType {
    fn from(entity_instance: EntityInstance) -> Self {
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|enemy_type| enemy_type.identifier == *"enemy_type")
        {
            let enemy_type_option = match &field_instance.value {
                FieldValue::Enum(val) => match val.as_deref() {
                    Some("Durt") => Self::Durt,
                    Some("LongHair") => Self::LongHair,
                    _ => {
                        panic!("This is impossible option");
                    }
                },
                _ => {
                    panic!("Cound't find any covered enum");
                }
            };

            return enemy_type_option;
        }

        panic!("Coundn't find any available options. Please check Ldtk map 'enemy_type' enum");
    }
}

#[derive(Bundle, LdtkEntity)]
struct EnemyBundle {
    pub enemy: Enemy,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[from_entity_instance]
    pub enemy_type: EnemyType,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_enemy)
                .into(),
        )
        // Use the same name as it's covered in "LdtkMap"
        .register_ldtk_entity::<EnemyBundle>("Mob");
    }
}

fn spawn_enemy(
    mut commands: Commands,
    materials: Res<Sprites>,
    enemies_query: Query<(Entity, &Transform, &EnemyType), Added<Enemy>>,
) {
    for (enemy, transform, enemy_type) in enemies_query.iter() {
        let enemy_material = match enemy_type {
            EnemyType::Durt => &materials.monsters.gray,
            EnemyType::LongHair => &materials.monsters.long,
        };

        let sprite_width = enemy_material.width;
        let sprite_height = enemy_material.height;

        commands
            .entity(enemy)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(sprite_width / 2.0, sprite_height / 2.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Friction::new(3.0))
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: enemy_material.texture.clone(),
                // transform: *transform,
                transform: Transform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    scale: Vec3::new(1.0, 1.0, 1.0),
                },
                ..Default::default()
            });
    }
}
