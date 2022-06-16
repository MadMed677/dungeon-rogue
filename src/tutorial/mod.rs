use bevy::prelude::*;

mod tutorial_physics;
mod tutorial_ui;

pub use tutorial_physics::*;
pub use tutorial_ui::*;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TutorialPhysicsPlugin)
            .add_plugin(TutorialUiPlugin);
    }
}
