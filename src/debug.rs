use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::player::Player;
use crate::Speed;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Player>()
            .register_inspectable::<Speed>()
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(RapierRenderPlugin);
    }
}
