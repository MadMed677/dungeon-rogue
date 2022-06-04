use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ApplicationState;

pub struct EnemyPlugin;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, LdtkEntity)]
struct EnemyBundle {
    pub enemy: Enemy,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationState::Game).with_system(spawn_enemy))
            // app.add_system(spawn_enemy)
            // Use the same name as it's covered in "LdtkMap"
            .register_ldtk_entity::<EnemyBundle>("Mob");
    }
}

fn spawn_enemy(mut commands: Commands, enemies_query: Query<(Entity, &Transform), Added<Enemy>>) {
    for (enemy, transform) in enemies_query.iter() {
        let sprite_width = 10.0;
        let sprite_height = 10.0;

        commands
            .entity(enemy)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(sprite_width / 2.0, sprite_height / 2.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Friction::new(3.0))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.4, 0.2, 0.9),
                    custom_size: Some(Vec2::new(sprite_width, sprite_height)),
                    ..Default::default()
                },
                transform: *transform,
                ..Default::default()
            });
    }
}
