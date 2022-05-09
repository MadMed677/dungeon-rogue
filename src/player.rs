use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{Speed, Sprites};

#[derive(Component, Inspectable)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_actors", SystemStage::single(spawn_player))
            .add_system(player_movement)
            .add_system(player_jump);
    }
}

fn spawn_player(mut commands: Commands, materials: Res<Sprites>) {
    let x = 150.0;
    let y = 150.0;

    let sprite_width = 16.0;
    let sprite_height = 32.0;

    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(sprite_width / 2.0, sprite_height / 2.0))
        // Add Velocity component to iterate via it but with zero value
        .insert(Velocity::zero())
        .insert(ExternalImpulse::default())
        // .insert(Restitution::coefficient(1.0))
        .insert(Friction::new(0.1))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(3.0))
        .insert(ColliderMassProperties::Density(1.0))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: materials.player.clone(),
            transform: Transform::from_xyz(x, y, 3.0),
            sprite: TextureAtlasSprite {
                flip_x: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(120.0));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    if let Ok((speed, mut velocity)) = query.get_single_mut() {
        let direction_x = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        let move_delta_x = direction_x * speed.0;

        velocity.linvel.x = move_delta_x;
    }
}

fn player_jump(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut ExternalImpulse, With<Player>>,
) {
    if let Ok(mut external_impulse) = player_query.get_single_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            external_impulse.impulse = Vec2::new(0.0, 50.0);
        }
    }
}
