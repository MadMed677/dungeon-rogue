use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

const PIXELS_PER_METER: f32 = 50.0;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // This means that 50px in graphics is the same as 1 in physics
        // Read more information there:
        //  https://rapier.rs/docs/user_guides/bevy_plugin/common_mistakes#why-is-everything-moving-in-slow-motion
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        // Change gravity from -98.0 to -300.0
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(1.0, -300.0),
            ..Default::default()
        });
    }
}
