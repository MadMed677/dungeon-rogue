use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{ApplicationState, Health};

/// Show the Heads-up Display for the entities who have a Health component
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_player_hud)
                .into(),
        );
    }
}

fn spawn_player_hud(mut commands: Commands, health_query: Query<(Entity, &Health), Added<Health>>) {
    if let Ok((health_entity, health)) = health_query.get_single() {
        let health_max_width_px = 30.0;
        let pixel_per_point = health_max_width_px / health.max as f32;

        let health_current_width_px = pixel_per_point * health.current as f32;

        // We have to make a translation by X axis because current health bar
        //  centralized by center-center, but we have to make start current health
        //  bar with the same coordinate as the first bar began.
        // To do so we have to calculate the difference between them and devide by 2
        //  becase we have the same gap from left and right but we have to translate
        //  it only from one side
        let current_health_translation_px = (health_max_width_px - health_current_width_px) / 2.0;

        commands.entity(health_entity).with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 0.0, 0.0, 0.3),
                        custom_size: Some(Vec2::new(health_max_width_px, 5.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 20.0, 1.0),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.0, 1.0, 0.0, 0.7),
                            custom_size: Some(Vec2::new(health_current_width_px, 5.0)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(-current_health_translation_px, 0.0, 1.0),
                        ..Default::default()
                    });
                });
        });
    }
}
