mod coin;
mod tile;
mod tilemap;
mod wall;

pub use coin::*;
pub use tile::*;
pub use tilemap::*;
pub use wall::*;

use bevy::prelude::*;

use crate::ascii::AsciiSheet;

pub struct MapPlugin;

/// Plugin for managing the map load and instantiation of tiles.
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap::from_string("assets/map.txt"))
            .add_systems(Startup, spawn_tiles);
    }
}

/// Spawn tiles depending on the loaded map.
fn spawn_tiles(mut commands: Commands, map: Res<TileMap>, ascii: Res<AsciiSheet>) {
    let columns = map.columns();

    for (x, column) in columns.enumerate() {
        for (y, tile) in column.iter().enumerate() {
            match *tile {
                Tile::Wall => {
                    spawn_sprites_for_wall(&mut commands, &ascii, &map, x as i32, y as i32);
                }
                Tile::Coin => spawn_coin(&mut commands, &ascii, x as i32, y as i32),
                _ => {
                    continue;
                }
            };
        }
    }
}
