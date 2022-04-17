use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_stage("game_map_plugin", SystemStage::single(spawn_map));
        app.add_startup_system(spawn_map2)
            .add_system(set_texture_filters_to_nearest);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct CustomMap {
    width: usize,
    height: usize,
    tiles: Vec<TileType>,
}

impl CustomMap {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileType::Floor; width * height],
        }
    }

    fn tiles_2d(&self) -> Vec<(TileType, usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                let x = index % self.width;
                let y = index / self.width;

                (*tile, x, y)
            })
            .collect()
    }
}

fn spawn_map2(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("tiles.png");
    // let texture_handle = asset_server.load("full_game_assets.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            // MapSize(10, 10),
            // ChunkSize(64, 64),
            // TileSize(16.0, 16.0),
            // TextureSize(96.0, 16.0),
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(16.0, 16.0),
            TextureSize(96.0, 16.0),
        ),
        0,
        0,
    );

    layer_builder.set_all(TileBundle::default());

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    map.add_layer(&mut commands, 0, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}

fn spawn_map(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let width = window.width() / 5.0;
    let height = window.height() / 5.0;

    println!("width: {}", width);
    println!("height: {}", height);

    let map = CustomMap::new(width as usize, height as usize);

    for (tile, x, y) in map.tiles_2d().iter() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: match tile {
                    TileType::Floor => Color::rgb(0.5, 0.5, 0.5),
                    TileType::Wall => Color::rgb(0.1, 0.1, 0.1),
                },
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(*x as f32, *y as f32, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}
