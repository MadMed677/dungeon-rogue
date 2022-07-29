use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{ApplicationState, Health};

/// Show the Heads-up Display for the entities who have a Health component
pub struct HudPlugin;

#[derive(Component)]
struct HudCurrentHealthUI;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_hud_ui)
                .with_system(update_hud_ui)
                .into(),
        );

        // Save all parameters to use it in `update_hud_ui` system and others
        app.insert_resource(HudResourse {
            health_ui_width_px: 20.0,
            health_ui_height_px: 2.5,
            health_ui_margin_top_px: 15.0,
        });
    }
}

struct HudResourse {
    health_ui_width_px: f32,
    health_ui_height_px: f32,
    health_ui_margin_top_px: f32,
}

/// Spawn HuD (Heads-up Display) above the enemies which has more `Health` component
fn spawn_hud_ui(
    mut commands: Commands,
    health_query: Query<(Entity, &Health), Added<Health>>,
    hud_resource: Res<HudResourse>,
) {
    for (health_entity, health) in health_query.iter() {
        let pixel_per_point = hud_resource.health_ui_width_px / health.max as f32;

        let health_current_width_px = pixel_per_point * health.current as f32;

        // We have to make a translation by X axis because current health bar
        //  centralized by center-center, but we have to make start current health
        //  bar with the same coordinate as the first bar began.
        // To do so we have to calculate the difference between them and devide by 2
        //  becase we have the same gap from left and right but we have to translate
        //  it only from one side
        let current_health_translation_px =
            (hud_resource.health_ui_width_px - health_current_width_px) / 2.0;

        commands.entity(health_entity).with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 0.7),
                        custom_size: Some(Vec2::new(
                            health_current_width_px,
                            hud_resource.health_ui_height_px,
                        )),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        -current_health_translation_px,
                        hud_resource.health_ui_margin_top_px,
                        1.0,
                    ),
                    ..Default::default()
                })
                .insert(HudCurrentHealthUI);

            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 0.0, 0.0, 0.3),
                    custom_size: Some(Vec2::new(
                        hud_resource.health_ui_width_px,
                        hud_resource.health_ui_height_px,
                    )),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, hud_resource.health_ui_margin_top_px, 1.0),
                ..Default::default()
            });
        });
    }
}

/// Update HuD (Heads-up Display) when an entity got damage
/// and `Health` component changed
fn update_hud_ui(
    health_query: Query<(Entity, &Health, &Children), Changed<Health>>,
    mut health_ui_query: Query<(&mut Sprite, &mut Transform), With<HudCurrentHealthUI>>,
    hud_resource: Res<HudResourse>,
) {
    for (_, health, health_children) in health_query.iter() {
        for &health_child in health_children.iter() {
            if let Ok((mut health_ui_sprite, mut health_ui_transform)) =
                health_ui_query.get_mut(health_child)
            {
                let pixel_per_point = hud_resource.health_ui_width_px / health.max as f32;

                let health_current_width_px = pixel_per_point * health.current as f32;
                let current_health_translation_px =
                    (hud_resource.health_ui_width_px - health_current_width_px) / 2.0;

                health_ui_sprite.custom_size = Some(Vec2::new(
                    health_current_width_px,
                    hud_resource.health_ui_height_px,
                ));
                health_ui_transform.translation = Vec3::new(
                    -current_health_translation_px,
                    hud_resource.health_ui_margin_top_px,
                    1.0,
                );
            }
        }
    }
}
