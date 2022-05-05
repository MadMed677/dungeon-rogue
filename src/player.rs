use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::Speed;

#[derive(Component, Inspectable)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_floor)
            .add_system(player_movement);
    }
}

fn spawn_floor(mut commands: Commands) {
    let sprite_width = 200.0;
    let sprite_height = 10.0;

    commands
        .spawn()
        .insert(Collider::cuboid(sprite_width / 2.0, sprite_height / 2.0))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(sprite_width, sprite_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(400.0, 200.0, 2.0),
            ..Default::default()
        });
}

fn spawn_player(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // fn spawn_player(mut commands: Commands) {
    // Set the gravity as zero
    rapier_config.gravity = Vec2::ZERO;

    let x = 150.0;
    let y = 150.0;

    let sprite_size = 10.0;

    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0))
        .insert(Velocity::zero())
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(0.5))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 3.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(200.0));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    if let Ok((speed, mut velocity)) = query.get_single_mut() {
        // Represent (x, y) coordinates
        let direction_x = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        let direction_y = if keyboard.pressed(KeyCode::Up) {
            1.0
        } else if keyboard.pressed(KeyCode::Down) {
            -1.0
        } else {
            0.0
        };

        velocity.linvel = Vec2::new(direction_x * speed.0, direction_y * speed.0);
        velocity.angvel = 0.0;
    }
}
