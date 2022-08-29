use bevy::prelude::*;

mod player_animations;
mod player_physics;

pub use player_animations::{PlayerAnimationState, PlayerProcessAnimation};
pub use player_physics::{GroundDetection, JumpState, Player, SideDetector, SideSensor};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player_physics::PlayerPhysicsPlugin)
            .add_plugin(player_animations::PlayerAnimationPlugin);
    }
}
