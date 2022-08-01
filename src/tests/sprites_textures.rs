#[cfg(test)]
use bevy::prelude::*;

#[cfg(test)]
use crate::{PlayerSprites, Sprites};

#[cfg(test)]
pub fn prepare_sprites() -> Sprites {
    Sprites {
        player: PlayerSprites {
            pumpkin: crate::SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
            dragon: crate::SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
        },
        monsters: crate::MonstersSprites {
            gray: crate::SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
            long: crate::SpriteAssetInfo {
                width: 10.0,
                height: 10.0,
                texture: Handle::default(),
            },
        },
        tutorial: crate::TutorialSprites {
            movement: Handle::default(),
        },
    }
}
