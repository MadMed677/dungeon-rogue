use bevy::prelude::*;
use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};
use bevy_rapier2d::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::{Inspectable, InspectorPlugin, RegisterInspectable};

use crate::map::WallCollision;
use crate::player::Player;
use crate::{Climbable, Speed};

pub struct DebugPlugin;

#[derive(Inspectable, Default)]
struct Inspector {
    player: InspectorQuerySingle<Entity, With<Player>>,
    collisions: InspectorQuery<Entity, With<WallCollision>>,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // label for our debug stage
        static DEBUG: &str = "debug";

        if cfg!(debug_assertions) {
            app.add_plugin(InspectorPlugin::<Inspector>::new())
                .register_inspectable::<Player>()
                .register_inspectable::<Speed>()
                .register_inspectable::<Name>()
                .add_stage_after(CoreStage::Update, DEBUG, SystemStage::single_threaded())
                .add_system_to_stage(DEBUG, debug_collisions)
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
    // wall_colliders: Query<(Entity, &Collider, &GlobalTransform), With<WallCollision>>,
    player_collider: Query<(Entity, &Collider, &GlobalTransform), Added<Player>>,
    climbables_collider: Query<(Entity, &Collider, &GlobalTransform), Added<Climbable>>,
) {
    // Show debug layer for the walls
    // for (entity, collider, transform) in wall_colliders.iter() {
    //     let half_sizes = collider.as_cuboid().unwrap().half_extents();
    //     let full_sizes = half_sizes * 2.0;

    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite {
    //                 color: Color::rgba(0.5, 0.5, 0.5, 0.5),
    //                 custom_size: Some(full_sizes),
    //                 ..Default::default()
    //             },
    //             transform: Transform {
    //                 translation: Vec3::new(transform.translation.x, transform.translation.y, 20.0),
    //                 rotation: transform.rotation,
    //                 scale: transform.scale,
    //             },
    //             ..Default::default()
    //         })
    //         .insert(Parent(entity));
    // }

    // Show the debug layer for the player
    if let Ok((player_entity, player_collider, player_transform)) = player_collider.get_single() {
        let half_sizes = player_collider.as_cuboid().unwrap().half_extents();
        let full_sizes = half_sizes * 2.0;

        let debug_player_layer = commands.spawn_bundle(SpriteBundle {
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

        let debug_player_layer_entity = debug_player_layer.id();

        // Add Debug Player layer as a children of the player itself
        commands
            .entity(player_entity)
            .add_child(debug_player_layer_entity);
    }

    for (climbable_entity, climbable_collider, climbable_transform) in climbables_collider.iter() {
        let half_sizes = climbable_collider.as_cuboid().unwrap().half_extents();
        let full_sizes = half_sizes * 2.0;

        let debug_climbable_layer = commands.spawn_bundle(SpriteBundle {
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

        let debug_climbable_layer_entity = debug_climbable_layer.id();

        commands
            .entity(climbable_entity)
            .add_child(debug_climbable_layer_entity);
    }
}
