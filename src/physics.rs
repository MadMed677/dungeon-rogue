use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// This means that 50px in graphics is the same as 1 in physics
/// Read more information there:
///  https://rapier.rs/docs/user_guides/bevy_plugin/common_mistakes#why-is-everything-moving-in-slow-motion
pub const GRAPHICS_TO_PHYSICS: f32 = 50.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                scale: GRAPHICS_TO_PHYSICS,
                ..Default::default()
            });
    }
}
