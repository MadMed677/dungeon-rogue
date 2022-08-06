use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    ApplicationState, Health, MovementAnimation, MovementDirection, OnMove, Speed, Sprites,
};

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

fn enemy_movement(
    mut patrol_query: Query<
        (
            &Transform,
            &Speed,
            &mut OnMove,
            &mut Velocity,
            &mut Patrol,
            &mut MovementDirection,
            &mut TextureAtlasSprite,
        ),
        With<Enemy>,
    >,
) {
    for (transform, speed, mut on_move, mut velocity, mut patrol, mut direction, mut sprite) in
        patrol_query.iter_mut()
    {
        // Do nothing if we have no patrol or it's equal to 1
        if patrol.points.len() <= 1 {
            continue;
        }

        // Say that current enemy on move
        on_move.0 = true;

        let mut new_velocity =
            (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * speed.0;

        if new_velocity.x > 0.0 {
            *direction = MovementDirection::Right;
            sprite.flip_x = false;
        } else {
            *direction = MovementDirection::Left;
            sprite.flip_x = true;
        }

        if new_velocity.dot(velocity.linvel) < 0.0 {
            if patrol.index == 0 {
                *direction = MovementDirection::Right;
            } else if patrol.index == patrol.points.len() - 1 {
                *direction = MovementDirection::Left;
            }

            if *direction == MovementDirection::Right {
                patrol.index += 1;
            } else {
                patrol.index -= 1;
            }

            new_velocity = (patrol.points[patrol.index] - transform.translation.truncate())
                .normalize()
                * speed.0;
        }

        velocity.linvel = new_velocity;
    }
}

fn enemy_movement_animation(
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut MovementAnimation,
            &OnMove,
        ),
        With<Enemy>,
    >,
) {
    for (mut sprite, texture_atlas_handle, mut movement_animation, on_move) in query.iter_mut() {
        // Do not animate if the player is not on move
        if !on_move.0 {
            continue;
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

fn ldtk_pixel_coords_to_translation_pivoted(
    ldtk_coords: IVec2,
    ldtk_pixel_height: i32,
    entity_size: IVec2,
    pivot: Vec2,
) -> Vec2 {
    let pivot_point = IVec2::new(ldtk_coords.x, ldtk_pixel_height - ldtk_coords.y).as_vec2();
    let adjusted_pivot = Vec2::new(0.5 - pivot.x, pivot.y - 0.5);
    let offset = entity_size.as_vec2() * adjusted_pivot;

    pivot_point + offset
}

/// Describes patrolling for an enemy
/// E.g. movement on the map
#[derive(Debug, Component, Inspectable)]
pub struct Patrol {
    /// Points which describes when the monster should stop
    /// Describes the main points for patroling
    pub points: Vec<Vec2>,

    /// Describes current index in points.
    /// The index should be inside `points.len()` because
    /// `index` is index inside points vector
    pub index: usize,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut points = vec![ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        )];

        let ldtk_patrol = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"patrol")
            .expect("Should have 'patrol' field");

        if let FieldValue::Points(ldtk_points) = &ldtk_patrol.value {
            for ldtk_point in ldtk_points.iter().flatten() {
                let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 1.0))
                    * Vec2::splat(layer_instance.grid_size as f32);

                points.push(ldtk_pixel_coords_to_translation_pivoted(
                    pixel_coords.as_ivec2(),
                    layer_instance.c_hei * layer_instance.grid_size,
                    IVec2::new(entity_instance.width, entity_instance.height),
                    entity_instance.pivot,
                ));
            }
        }

        Self { points, index: 1 }
    }
}

#[derive(Bundle, LdtkEntity)]
struct EnemyBundle {
    pub enemy: Enemy,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    #[from_entity_instance]
    pub enemy_type: EnemyType,

    #[ldtk_entity]
    pub patrol: Patrol,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_enemy)
                .with_system(enemy_movement)
                .with_system(enemy_movement_animation)
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

        // Setup a default scale for an entity. It cannot be less then 1.0 for `x` and `y` axis
        let scale = if transform.scale.x < 1.0 || transform.scale.y < 1.0 {
            Vec3::new(1.0, 1.0, 1.0)
        } else {
            transform.scale
        };

        commands
            .entity(enemy)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(sprite_width / 2.0, sprite_height / 2.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Velocity::zero())
            .insert(Friction::new(3.0))
            // Set a default movement direction in on right. We will change it later in the system
            .insert(MovementDirection::Right)
            .insert(MovementAnimation {
                timer: Timer::from_seconds(0.12, true),
            })
            .insert(Speed(80.0))
            // By default enemy are not on move
            .insert(OnMove(false))
            .insert(Health { current: 2, max: 2 })
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: enemy_material.texture.clone(),
                // transform: *transform,
                transform: Transform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    scale,
                },
                ..Default::default()
            });
    }
}
