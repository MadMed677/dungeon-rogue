use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    ron_parsers::GameTextures, ApplicationState, AttackAnimation, Attacks, ClimbAnimation, Climber,
    DeathAnimation, HurtAnimation, IdleAnimation, JumpAnimation, MovementAnimation, OnMove,
    PlayerIsDeadEvent, PlayerIsHitEvent,
};

use super::{GroundDetection, Player};
pub struct PlayerAnimationPlugin;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Inspectable)]
pub enum PlayerProcessAnimation {
    Start,
    End,
}

impl Default for PlayerProcessAnimation {
    fn default() -> Self {
        Self::Start
    }
}

/// Describes animation state of the player
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Inspectable)]
pub enum PlayerAnimationState {
    /// Player does nothing
    Idle,

    /// Player run
    Run,

    /// Player climbs
    Climb,

    /// Player has taken damage but didn't die
    Hit(PlayerProcessAnimation),

    /// Player died
    Death(PlayerProcessAnimation),

    /// Player jumps
    Jump,

    /// Player attacks
    Attack(PlayerProcessAnimation),
}

impl Default for PlayerAnimationState {
    fn default() -> Self {
        Self::Idle
    }
}

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(setup)
                .with_system(
                    player_animation_processor.run_not_in_state(PlayerAnimationState::Death(
                        PlayerProcessAnimation::Start,
                    )),
                )
                .with_system(player_animation_textures_processor)
                .with_system(player_idle_animation.run_in_state(PlayerAnimationState::Idle))
                .with_system(player_climb_animation.run_in_state(PlayerAnimationState::Climb))
                .with_system(player_run_animation.run_in_state(PlayerAnimationState::Run))
                .with_system(player_jump_animation.run_in_state(PlayerAnimationState::Jump))
                .with_system(
                    player_attack_animation
                        .run_in_state(PlayerAnimationState::Attack(PlayerProcessAnimation::Start)),
                )
                .with_system(
                    player_hurt_animation
                        .run_in_state(PlayerAnimationState::Hit(PlayerProcessAnimation::Start)),
                )
                .with_system(
                    player_death_animation
                        .run_in_state(PlayerAnimationState::Death(PlayerProcessAnimation::Start)),
                )
                .into(),
        );
    }
}

/// Setup player animation TTLs and useful components for animation
fn setup(mut commands: Commands, player_query: Query<Entity, Added<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands
            .entity(player_entity)
            .insert(IdleAnimation {
                timer: Timer::from_seconds(0.1, true),
            })
            .insert(MovementAnimation {
                timer: Timer::from_seconds(0.1, true),
            })
            .insert(ClimbAnimation {
                timer: Timer::from_seconds(0.15, true),
                index: 0,
            })
            .insert(HurtAnimation {
                timer: Timer::from_seconds(0.1, true),
            })
            .insert(JumpAnimation {
                timer: Timer::from_seconds(0.05, true),
            })
            .insert(AttackAnimation {
                timer: Timer::from_seconds(0.04, true),
            });
    }
}

/// Triggers when `animation_state` has changed and update user texture
fn player_animation_textures_processor(
    mut commands: Commands,
    materials: Res<GameTextures>,
    animation_state: Res<CurrentState<PlayerAnimationState>>,
    mut player_query: Query<
        (Entity, &Transform, &mut TextureAtlasSprite, &mut Attacks),
        With<Player>,
    >,
    death_animation_query: Query<Entity, With<DeathAnimation>>,
) {
    if animation_state.is_changed() {
        if let Ok((entity, transform, mut sprite, mut attacks)) = player_query.get_single_mut() {
            sprite.index = 0;

            match animation_state.0 {
                PlayerAnimationState::Idle => {
                    commands
                        .entity(entity)
                        .insert(materials.player.idle.texture.clone());
                }
                PlayerAnimationState::Run => {
                    commands
                        .entity(entity)
                        .insert(materials.player.run.texture.clone());
                }
                PlayerAnimationState::Climb => {
                    commands
                        .entity(entity)
                        .insert(materials.player.climb.texture.clone());
                }
                PlayerAnimationState::Hit(hit_animation) => match hit_animation {
                    PlayerProcessAnimation::Start => {
                        commands
                            .entity(entity)
                            .insert(materials.player.hurt.texture.clone());
                    }
                    PlayerProcessAnimation::End => {
                        // Stop user attack state when user received hit
                        //  to avoid the problem when the user will attack
                        //  after hit animation will be stopped
                        attacks.0 = false;

                        commands.insert_resource(NextState(PlayerAnimationState::Idle));
                    }
                },
                PlayerAnimationState::Jump => {
                    commands
                        .entity(entity)
                        .insert(materials.player.jump.texture.clone());
                }
                PlayerAnimationState::Attack(attack_animation) => match attack_animation {
                    PlayerProcessAnimation::Start => {
                        commands
                            .entity(entity)
                            .insert(materials.player.attack.texture.clone());
                    }
                    PlayerProcessAnimation::End => {
                        attacks.0 = false;
                    }
                },
                PlayerAnimationState::Death(death_animation) => match death_animation {
                    PlayerProcessAnimation::Start => {
                        // Spawn player death animation
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                texture_atlas: materials.player.death.texture.clone(),
                                transform: *transform,
                                sprite: TextureAtlasSprite {
                                    flip_x: sprite.flip_x,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(DeathAnimation {
                                timer: Timer::from_seconds(0.1, true),
                            });

                        // Remove the player from the scene
                        commands.entity(entity).despawn_recursive();
                    }
                    PlayerProcessAnimation::End => {
                        unreachable!();
                    }
                },
            }

            return;
        }

        // This match should be only when the Player is destroyed
        // This is happening only when the Player is dead and
        //  we removed it from the scene
        if animation_state.0 == PlayerAnimationState::Death(PlayerProcessAnimation::End) {
            // We should remove useless sprite
            if let Ok(death_sprite_entity) = death_animation_query.get_single() {
                commands.entity(death_sprite_entity).despawn();
            }
        }
    }
}

/// Handle all physical changes and set correct player animation state
#[allow(clippy::type_complexity)]
fn player_animation_processor(
    player_animation_state: Res<CurrentState<PlayerAnimationState>>,
    mut commands: Commands,
    mut player_query: Query<(&OnMove, &Climber, &GroundDetection, &Attacks), With<Player>>,
    mut player_hit_event: EventReader<PlayerIsHitEvent>,
    mut player_death_event: EventReader<PlayerIsDeadEvent>,
) {
    if let Ok((on_move, climber, ground_detection, attacks)) = player_query.get_single_mut() {
        if player_death_event.iter().next().is_some() {
            commands.insert_resource(NextState(PlayerAnimationState::Death(
                PlayerProcessAnimation::Start,
            )));

            return;
        }

        if player_hit_event.iter().next().is_some() {
            commands.insert_resource(NextState(PlayerAnimationState::Hit(
                PlayerProcessAnimation::Start,
            )));

            return;
        }

        // Forbid the next animation until player finish the current one
        if player_animation_state.0 == PlayerAnimationState::Hit(PlayerProcessAnimation::Start)
            || player_animation_state.0
                == PlayerAnimationState::Death(PlayerProcessAnimation::Start)
        {
            return;
        }

        if attacks.0 {
            if player_animation_state.0
                != PlayerAnimationState::Attack(PlayerProcessAnimation::Start)
            {
                commands.insert_resource(NextState(PlayerAnimationState::Attack(
                    PlayerProcessAnimation::Start,
                )));
            }

            return;
        }

        // Climbing has more priority than movement or idle
        if climber.climbing {
            if player_animation_state.0 != PlayerAnimationState::Climb {
                commands.insert_resource(NextState(PlayerAnimationState::Climb));
            }

            return;
        }

        if !ground_detection.on_ground {
            if player_animation_state.0 != PlayerAnimationState::Jump {
                commands.insert_resource(NextState(PlayerAnimationState::Jump));
            }

            return;
        }

        if on_move.0 {
            if player_animation_state.0 != PlayerAnimationState::Run {
                commands.insert_resource(NextState(PlayerAnimationState::Run));
            }

            return;
        }

        if player_animation_state.0 != PlayerAnimationState::Idle {
            commands.insert_resource(NextState(PlayerAnimationState::Idle));
        }
    }
}

fn player_idle_animation(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut IdleAnimation), With<Player>>,
) {
    for (mut sprite, mut idle_animation) in query.iter_mut() {
        // Do nothing if the player is not in idle

        idle_animation.timer.tick(time.delta());

        if idle_animation.timer.finished() {
            sprite.index += 1;

            // 24 - is a maximum amount of textures for idle state
            if sprite.index >= 24 {
                // Loop the animation
                sprite.index = 0;
            }
        }
    }
}

fn player_climb_animation(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut TextureAtlasSprite, &mut ClimbAnimation), With<Player>>,
) {
    for (velocity, mut sprite, mut climb_animation) in query.iter_mut() {
        climb_animation.timer.tick(time.delta());

        if climb_animation.timer.finished() {
            // let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let y_velocity = velocity.linvel.y;

            #[allow(clippy::manual_range_contains)]
            if y_velocity > 20.0 || y_velocity < -20.0 {
                climb_animation.index = (climb_animation.index + 1) % 12;

                sprite.index = climb_animation.index;
            }
        }
    }
}

fn player_run_animation(
    time: Res<Time>,
    materials: Res<GameTextures>,
    mut query: Query<(&mut TextureAtlasSprite, &mut MovementAnimation), With<Player>>,
) {
    let player_materials = &materials.player;

    for (mut sprite, mut movement_animation) in query.iter_mut() {
        movement_animation.timer.tick(time.delta());

        if movement_animation.timer.finished() {
            sprite.index += 1;

            if sprite.index >= player_materials.run.items {
                sprite.index = 0;
            }
        }
    }
}

fn player_jump_animation(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut JumpAnimation), With<Player>>,
) {
    for (mut sprite, mut jump_animation) in query.iter_mut() {
        jump_animation.timer.tick(time.delta());
        if jump_animation.timer.finished() {
            // let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            sprite.index += 1;

            if sprite.index >= 13 {
                sprite.index = 0;
            }
        }
    }
}

fn player_attack_animation(
    mut commands: Commands,
    time: Res<Time>,
    materials: Res<GameTextures>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AttackAnimation), With<Player>>,
) {
    let player_materials = &materials.player;

    for (mut sprite, mut attack_animation) in query.iter_mut() {
        attack_animation.timer.tick(time.delta());
        if attack_animation.timer.finished() {
            sprite.index += 1;

            if sprite.index >= player_materials.attack.items {
                sprite.index = 0;

                commands.insert_resource(NextState(PlayerAnimationState::Attack(
                    PlayerProcessAnimation::End,
                )));
            }
        }
    }
}

fn player_hurt_animation(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut HurtAnimation,
        ),
        With<Player>,
    >,
) {
    for (mut sprite, texture_atlas_handle, mut hurt_animation) in query.iter_mut() {
        hurt_animation.timer.tick(time.delta());

        if hurt_animation.timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            sprite.index += 1;

            if sprite.index >= texture_atlas.textures.len() {
                // We should stop the animation and give back the control
                sprite.index = 0;

                commands.insert_resource(NextState(PlayerAnimationState::Hit(
                    PlayerProcessAnimation::End,
                )));
            }
        }
    }
}

fn player_death_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut DeathAnimation,
        &mut Visibility,
    )>,
) {
    for (mut sprite, mut death_animation, mut visibility) in query.iter_mut() {
        death_animation.timer.tick(time.delta());

        if death_animation.timer.finished() {
            sprite.index += 1;

            // Send death state of 1 frome earlier to be able to remove the user
            //  and avoid the idle state
            if sprite.index >= 36 {
                // We should stop the animation and give back the control
                sprite.index = 0;

                commands.insert_resource(NextState(PlayerAnimationState::Death(
                    PlayerProcessAnimation::End,
                )));

                // Hide the entity until remove it from the scene
                visibility.is_visible = false;
            }
        }
    }
}
