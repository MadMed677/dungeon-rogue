use bevy::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

/// Describes the sprite assets information
///
/// !!Note!! Works only with TextureAtlas
#[derive(Clone, Debug)]
pub struct SpriteAssetInfo {
    /// The `width` of the sprite cell (not the whole atlas texture)
    pub width: f32,

    /// The `height` of the sprite cell (not the whole atlas texture)
    pub height: f32,

    /// TextureAtlas
    pub texture: Handle<TextureAtlas>,
}

#[derive(Debug)]
pub struct PlayerSprites {
    pub idle: SpriteAssetInfo,
    pub run: SpriteAssetInfo,
    pub climb: SpriteAssetInfo,
    pub hurt: SpriteAssetInfo,
    pub death: SpriteAssetInfo,
    pub jump: SpriteAssetInfo,
}

#[derive(Debug)]
pub struct EnemiesSprites {
    pub gray: SpriteAssetInfo,
    pub long: SpriteAssetInfo,
}

#[derive(Debug)]
pub struct TutorialSprites {
    pub movement: Handle<Image>,
}

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Parse current RON config
    let deserialized_game_textures = DeserializedGameTextures::load();

    let game_textures =
        GameTextures::new(&deserialized_game_textures, asset_server, texture_atlases);

    commands.insert_resource(game_textures);
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum DeserializedPlayerType {
    Idle,
    Run,
    Climb,
    Jump,
    Hurt,
    Death,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum DeserializedEnemyType {
    Gray,
    Long,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum DeserializedTutorialType {
    Movement,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DeserializedPlayerSpriteInfo {
    pub sprite_type: DeserializedPlayerType,
    pub width: f32,
    pub height: f32,
    pub texture_path: String,
    pub items: usize,
    pub column_size: usize,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DeserializedEnemySpriteInfo {
    pub sprite_type: DeserializedEnemyType,
    pub width: f32,
    pub height: f32,
    pub texture_path: String,
    pub items: usize,
    pub column_size: usize,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DeserializedTutorialSpriteInfo {
    pub sprite_type: DeserializedTutorialType,
    pub texture_path: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DeserializedGameTextures {
    pub player: Vec<DeserializedPlayerSpriteInfo>,
    pub enemies: Vec<DeserializedEnemySpriteInfo>,
    pub tutorials: Vec<DeserializedTutorialSpriteInfo>,
}

impl DeserializedGameTextures {
    pub fn load() -> Self {
        let file = File::open("resources/textures.ron").expect("Failed opening file");

        from_reader(file).expect("Unable to parse the textures")
    }
}

#[derive(Debug)]
pub struct GameTextures {
    pub player: PlayerSprites,
    pub enemies: EnemiesSprites,
    pub tutorials: TutorialSprites,
}

impl GameTextures {
    pub fn new(
        deserialized_textures: &DeserializedGameTextures,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        // Player
        let player_textures = &deserialized_textures.player;
        let mut idle = None;
        let mut run = None;
        let mut climb = None;
        let mut jump = None;
        let mut hurt = None;
        let mut death = None;

        for texture in player_textures.iter() {
            let player_texture = asset_server.load(texture.texture_path.as_str());
            let player_atlas = TextureAtlas::from_grid_with_padding(
                player_texture,
                Vec2::new(texture.width, texture.height),
                texture.column_size,
                (texture.items as f32 / texture.column_size as f32).ceil() as usize,
                Vec2::new(64.0 - texture.width, 64.0 - texture.height),
            );

            let sprite_asset_info = SpriteAssetInfo {
                width: texture.width,
                height: texture.height,
                texture: texture_atlases.add(player_atlas),
            };

            match texture.sprite_type {
                DeserializedPlayerType::Idle => {
                    idle = Some(sprite_asset_info);
                }
                DeserializedPlayerType::Run => {
                    run = Some(sprite_asset_info);
                }
                DeserializedPlayerType::Climb => {
                    climb = Some(sprite_asset_info);
                }
                DeserializedPlayerType::Jump => {
                    jump = Some(sprite_asset_info);
                }
                DeserializedPlayerType::Hurt => {
                    hurt = Some(sprite_asset_info);
                }
                DeserializedPlayerType::Death => {
                    death = Some(sprite_asset_info);
                }
            }
        }

        // Enemies
        let enemy_textures = &deserialized_textures.enemies;
        let mut gray = None;
        let mut long = None;

        for texture in enemy_textures.iter() {
            let enemy_texture = asset_server.load(texture.texture_path.as_str());
            let enemy_atlas = TextureAtlas::from_grid_with_padding(
                enemy_texture,
                Vec2::new(texture.width, texture.height),
                texture.column_size,
                texture.items / texture.column_size,
                Vec2::new(0.0, 0.0),
            );

            let sprite_asset_info = SpriteAssetInfo {
                width: texture.width,
                height: texture.height,
                texture: texture_atlases.add(enemy_atlas),
            };

            match texture.sprite_type {
                DeserializedEnemyType::Gray => {
                    gray = Some(sprite_asset_info);
                }
                DeserializedEnemyType::Long => {
                    long = Some(sprite_asset_info);
                }
            }
        }

        let tutorial_textures = &deserialized_textures.tutorials;
        let mut movement = None;

        for texture in tutorial_textures.iter() {
            let tutorial_texture = asset_server.load(texture.texture_path.as_str());

            match texture.sprite_type {
                DeserializedTutorialType::Movement => {
                    movement = Some(tutorial_texture);
                }
            }
        }

        // Check the player
        if idle.is_none()
            || run.is_none()
            || climb.is_none()
            || jump.is_none()
            || hurt.is_none()
            || death.is_none()
            || gray.is_none()
            || long.is_none()
            || movement.is_none()
        {
            panic!("All animations must be mapped");
        }

        Self {
            player: PlayerSprites {
                idle: idle.unwrap(),
                run: run.unwrap(),
                climb: climb.unwrap(),
                jump: jump.unwrap(),
                hurt: hurt.unwrap(),
                death: death.unwrap(),
            },
            enemies: EnemiesSprites {
                gray: gray.unwrap(),
                long: long.unwrap(),
            },
            tutorials: TutorialSprites {
                movement: movement.unwrap(),
            },
        }
    }
}
