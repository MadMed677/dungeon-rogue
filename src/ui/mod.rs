use bevy::prelude::*;

mod components;
mod dead_menu_ui;
mod main_menu_ui;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(main_menu_ui::MainMenuUIPlugin)
            .add_plugin(dead_menu_ui::DeadMenuUIPlugin);
    }
}
