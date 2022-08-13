use bevy::prelude::*;
use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};
use bevy_rapier2d::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::{Inspectable, InspectorPlugin, RegisterInspectable};

use crate::enemy::{Enemy, EnemyType, Patrol};
use crate::player::Player;
use crate::tutorial::{Tutorial, TutorialPassed, TutorialType};
use crate::{Climbable, Health, MovementDirection, OnMove, Speed};

pub struct DebugPlugin;

#[derive(Inspectable, Default)]
struct Inspector {
    player: InspectorQuerySingle<Entity, With<Player>>,
    tutorials: InspectorQuery<Entity, With<Tutorial>>,
    enemies: InspectorQuery<Entity, With<Enemy>>,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // label for our debug stage
        static DEBUG: &str = "debug";

        if cfg!(feature = "debug") {
            app.add_plugin(InspectorPlugin::<Inspector>::new())
                .register_inspectable::<Player>()
                .register_inspectable::<Speed>()
                .register_inspectable::<Name>()
                .register_inspectable::<TutorialPassed>()
                .register_inspectable::<TutorialType>()
                .register_inspectable::<MovementDirection>()
                .register_inspectable::<OnMove>()
                .register_inspectable::<EnemyType>()
                .register_inspectable::<Patrol>()
                .register_inspectable::<Health>()
                // .register_inspectable::<PlayerAnimationState>()
                .add_stage_after(CoreStage::Update, DEBUG, SystemStage::single_threaded())
                .add_system_to_stage(DEBUG, debug_collisions)
                .add_system_to_stage(DEBUG, update_debug_collisions)
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(RapierDebugRenderPlugin::default());
        }
    }
}

/// Shows an sprites on top of the map to show
///  the collision to be able to debug it
fn debug_collisions(
    mut commands: Commands,
    player_collider: Query<(Entity, &Collider, &GlobalTransform), Added<Player>>,
    enemies_collider: Query<(Entity, &Collider), Added<Enemy>>,
    climbables_collider: Query<(Entity, &Collider, &GlobalTransform), Added<Climbable>>,
) {
    // Show the debug layer for the player
    if let Ok((player_entity, player_collider, player_transform)) = player_collider.get_single() {
        let half_sizes = player_collider.as_cuboid().unwrap().half_extents();
        let full_sizes = half_sizes * 2.0;

        // Add Debug Player layer as a children of the player itself
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.5, 0.5, 0.5, 0.5),
                    custom_size: Some(full_sizes),
                    ..Default::default()
                },
                transform: Transform {
                    // Create relative coordinates for the player
                    translation: Vec3::new(0.0, 0.0, 20.0),
                    rotation: player_transform.rotation,
                    scale: player_transform.scale,
                },
                ..Default::default()
            });
        });
    }

    for (enemy_entity, enemy_collider) in enemies_collider.iter() {
        let half_sizes = enemy_collider.as_cuboid().unwrap().half_extents();
        let full_sizes = half_sizes * 2.0;

        commands.entity(enemy_entity).with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                    custom_size: Some(full_sizes),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 20.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }

    for (climbable_entity, climbable_collider, climbable_transform) in climbables_collider.iter() {
        let half_sizes = climbable_collider.as_cuboid().unwrap().half_extents();
        let full_sizes = half_sizes * 2.0;

        commands.entity(climbable_entity).with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.7, 0.7, 0.7, 0.5),
                    custom_size: Some(full_sizes),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 20.0),
                    rotation: climbable_transform.rotation,
                    scale: climbable_transform.scale,
                },
                ..Default::default()
            });
        });
    }
}

fn update_debug_collisions(
    parent_query: Query<(&Collider, &Children), Changed<Collider>>,
    mut children_query: Query<&mut Sprite>,
) {
    for (collider, children) in parent_query.iter() {
        for child in children.iter() {
            let half_sizes = collider.as_cuboid().unwrap().half_extents();
            let full_sizes = half_sizes * 2.0;

            // We may use `unwrap()` here because we definitely know that
            //  we iterates via all children for specific entity and we sure
            //  that we may get this entity directly
            if let Ok(mut sprite) = children_query.get_mut(*child) {
                sprite.custom_size = Some(full_sizes);
            };
        }
    }
}
