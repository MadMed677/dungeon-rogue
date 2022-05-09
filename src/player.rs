use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{MovementDirection, MovementTendency, Speed, Sprites};

#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component)]
struct MovementAnimation {
    timer: Timer,
}

#[derive(Component)]
/// Describes that entity on move or not
struct OnMove(bool);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_actors", SystemStage::single(spawn_player))
            .add_system(player_movement)
            .add_system(player_movement_animation)
            .add_system(player_jump);
    }
}

fn spawn_player(mut commands: Commands, materials: Res<Sprites>) {
    let x = 150.0;
    let y = 150.0;

    let sprite_width = 16.0;
    let sprite_height = 32.0;

    let player_direction = MovementTendency::Right;

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
                flip_x: match &player_direction {
                    MovementTendency::Left => true,
                    MovementTendency::Right => false,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MovementDirection(player_direction))
        .insert(Player)
        .insert(MovementAnimation {
            timer: Timer::from_seconds(0.1, true),
        })
        .insert(OnMove(false))
        .insert(Speed(120.0));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<
        (
            Entity,
            &Speed,
            &mut OnMove,
            &mut MovementDirection,
            &mut TextureAtlasSprite,
            &mut Velocity,
        ),
        With<Player>,
    >,
) {
    if let Ok((player_entity, speed, mut on_move, mut direction, mut sprite, mut velocity)) =
        query.get_single_mut()
    {
        let direction_x = if keyboard.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        let move_delta_x = direction_x * speed.0;

        // Update player velocity
        velocity.linvel.x = move_delta_x;

        // Change `OnMove` component
        if move_delta_x > 0.0 || move_delta_x < 0.0 {
            on_move.0 = true;
        } else {
            on_move.0 = false;
        }

        // Change player direction
        if move_delta_x > 0.0 {
            direction.0 = MovementTendency::Right;
            sprite.flip_x = false;
        } else if move_delta_x < 0.0 {
            direction.0 = MovementTendency::Left;
            sprite.flip_x = true;
        }
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

fn player_movement_animation(
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<
        (
            &OnMove,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut MovementAnimation,
        ),
        With<Player>,
    >,
) {
    for (on_move, mut sprite, texture_atlas_handle, mut movement_animation) in query.iter_mut() {
        // If the player is not on move
        //  set the first sprite which is equal to
        //  player default state and do nothing
        if on_move.0 == false {
            sprite.index = 0;

            return;
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
