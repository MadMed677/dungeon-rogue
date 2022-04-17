use bevy::prelude::*;

use crate::{Player, Speed};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_player_plugin", SystemStage::single(spawn_player))
            .add_system(player_movement);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(3.0))
        .with_children(|parent| {
            parent.spawn_bundle(OrthographicCameraBundle::new_2d());
        });
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    if let Ok((speed, mut transform)) = query.get_single_mut() {
        let direction = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        transform.translation.x += direction * speed.0;
    }
}
