mod tile;
mod tilemap;
mod wall;

pub use tile::*;
pub use tilemap::*;
pub use wall::*;

use bevy::{prelude::*, sprite::Anchor};

use crate::ascii::{AsciiSheet, SpriteIndices};

pub struct MapPlugin;

/// Plugin for managing the map load and instantiation of tiles.
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap::from_string("assets/map.txt"))
            .add_startup_system(spawn_tiles);
    }
}

/// Spawn tiles depending on the loaded map.
fn spawn_tiles(mut commands: Commands, map: Res<TileMap>, ascii: Res<AsciiSheet>) {
    let columns = map.columns();

    for (x, column) in columns.enumerate() {
        for (y, tile) in column.iter().enumerate() {
            let sprite = match *tile {
                Tile::Wall => {
                    spawn_sprites_for_wall(
                        &mut commands,
                        ascii.0.clone(),
                        &map,
                        x as i32,
                        y as i32,
                    );
                    continue;
                }
                Tile::Coin => {
                    let mut sprite = TextureAtlasSprite::new(SpriteIndices::SmallCoin.into());
                    sprite.custom_size = Some(Vec2::splat(1.0));
                    sprite.anchor = Anchor::BottomLeft;
                    sprite
                }
                _ => {
                    continue;
                }
            };

            commands.spawn(*tile).insert(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: 1.0,
                    },
                    scale: Vec3::new(1.0, 1.0, 0.0),
                    ..default()
                },
                sprite,
                texture_atlas: ascii.0.clone(),
                ..default()
            });
        }
    }
}
