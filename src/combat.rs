use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{player::Player, ApplicationState, Health};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(combat)
                .into(),
        );
    }
}

fn combat(keyboard: Res<Input<KeyCode>>, mut health_query: Query<&mut Health, With<Player>>) {
    for mut health in health_query.iter_mut() {
        if keyboard.just_pressed(KeyCode::H) {
            if health.current != 0 {
                health.current -= 1;
            }
        }
    }
}
