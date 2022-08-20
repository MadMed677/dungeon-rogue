use bevy::prelude::*;

mod settings;
mod textures;

pub use settings::Settings;
pub use textures::{EnemiesSprites, GameTextures, PlayerSprites, SpriteAssetInfo, TutorialSprites};

pub struct RonParsersPlugin;

impl Plugin for RonParsersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(settings::SettingsPlugin)
            .add_plugin(textures::TexturesPlugin);
    }
}
