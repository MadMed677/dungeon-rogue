use bevy::prelude::*;
use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};
use bevy_rapier2d::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::{Inspectable, InspectorPlugin, RegisterInspectable};

use crate::map::WallCollision;
use crate::player::Player;
use crate::Speed;

pub struct DebugPlugin;

#[derive(Inspectable, Default)]
struct Inspector {
    player: InspectorQuerySingle<Entity, With<Player>>,
    collisions: InspectorQuery<Entity, With<WallCollision>>,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(InspectorPlugin::<Inspector>::new())
                // app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>()
                .register_inspectable::<Speed>()
                .add_system(debug_collisions)
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
    wall_colliders: Query<(Entity, &Collider, &RigidBody, &Transform), Added<WallCollision>>,
    player_collider: Query<(Entity, &Collider, &Transform), With<Player>>,
) {
    // Show debug layer for the walls
    for (entity, collider, _, transform) in wall_colliders.iter() {
        let half_sizes = collider.as_cuboid().unwrap().half_extents();
        let full_sizes = half_sizes * 2.0;

        commands.entity(entity).insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.5, 0.5, 0.5, 0.5),
                custom_size: Some(full_sizes),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, 20.0),
                rotation: transform.rotation,
                scale: transform.scale,
            },
            ..Default::default()
        });
    }

    // Show the debug layer for the player
    //
    // !!Note!! This code will add SpriteBundle on each tick
    //  it happenes because Added<Player> don't use properly
    //  because the Player is a composite entity
    //
    // We will create the Player via `register_ldtk_entity` and
    //  create an empty Player entity with Worldly coords
    //  and after that we use `spawn_player` system which will add
    //  an extra components to the entity to make it work properly
    //
    // I don't know how to deal it better and use `Added<Player>` but
    //  only when `spawn_player` added all required components
    //  because of that I made just `With<Player>` and call this system
    //  on each tick. It's definitely bad for performance but I think
    //  it's okay for the debug mode. I'll handle it later (I hope so)
    if let Ok((player_entity, player_collider, player_transform)) = player_collider.get_single() {
        let half_sizes = player_collider.as_cuboid().unwrap().half_extents();
        let full_sizes = half_sizes * 2.0;

        commands.entity(player_entity).insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.5, 0.5, 0.5, 0.5),
                custom_size: Some(full_sizes),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    20.0,
                ),
                rotation: player_transform.rotation,
                scale: player_transform.scale,
            },
            ..Default::default()
        });
    }
}
