#[cfg(test)]
use bevy::prelude::*;

#[cfg(test)]
use crate::ron_parsers::{
    EnemiesSprites, GameTextures, PlayerSprites, SpriteAssetInfo, TutorialSprites,
};

#[cfg(test)]
pub fn prepare_sprites() -> GameTextures {
    GameTextures {
        player: PlayerSprites {
            idle: SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
            run: SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
            climb: SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
            hurt: SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
            death: SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
            jump: SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
        },
        enemies: EnemiesSprites {
            gray: SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
            long: SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
        },
        tutorials: TutorialSprites {
            movement: Handle::default(),
        },
    }
}
