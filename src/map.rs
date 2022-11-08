use std::fs;

use bevy::{prelude::*, sprite::Anchor};

use crate::ascii::{AsciiSheet, SpriteIdices};

pub struct MapPlugin;

/// Plugin for managing the map load and instantiation of tiles.
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::from_string("assets/map.txt"))
            .add_startup_system(spawn_tiles);
    }
}

#[derive(Clone)]
pub struct Map {
    // TODO: Do we need these two fields?
    pub columns: usize,
    pub rows: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn from_string(data: &str) -> Self {
        let content = fs::read_to_string(data).expect("Should have been able to read the map");

        let lines = content.lines().rev();

        let mut columns = vec![];

        for line in lines {
            let letters = line.chars();
            for (x, letter) in letters.enumerate() {
                if columns.len() <= x {
                    columns.push(vec![]);
                }
                columns[x].push(Tile::from_char(letter));
            }
        }

        // TODO: evaluate (or think about) whether this is needed
        assert!(
            columns.len() > 0,
            "Expected map to have at least one column"
        );

        let rows = columns[0].len();
        for (x, column) in columns.iter().enumerate() {
            assert!(
                column.len() == rows,
                "Expected all columns to have the same amount of rows! (failed at column {}; {} instead of {})",
                x, column.len(), rows
            );
        }

        Map {
            columns: columns.len(),
            tiles: columns,
            rows,
        }
    }
}

/// A tile within the game.
#[derive(Component, Clone, Copy)]
pub enum Tile {
    Empty,
    Wall,
    Coin,
}

impl Tile {
    pub fn from_char(character: char) -> Self {
        match character {
            ' ' => Self::Empty,
            '#' => Self::Wall,
            '.' => Self::Coin,
            _ => unreachable!(),
        }
    }
}

/// Spawn tiles depending on the loaded map.
fn spawn_tiles(mut commands: Commands, map: Res<Map>, ascii: Res<AsciiSheet>) {
    let tiles = map.tiles.clone();

    for (x, column) in tiles.iter().enumerate() {
        for (y, tile) in column.iter().enumerate() {
            let sprite = match *tile {
                Tile::Wall => determine_sprite_for_wall(&tiles, x, y),
                Tile::Coin => {
                    let mut sprite = TextureAtlasSprite::new(SpriteIdices::SmallCoin.into());
                    sprite.custom_size = Some(Vec2::splat(1.0));
                    sprite.anchor = Anchor::BottomLeft;
                    sprite
                }
                _ => {
                    continue;
                }
            };

            commands
                .spawn()
                .insert(*tile)
                .insert_bundle(SpriteSheetBundle {
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

/// Determine the sprites for a wall depending on the sprites around it.
fn determine_sprite_for_wall(tiles: &Vec<Vec<Tile>>, x: usize, y: usize) -> TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite::new(SpriteIdices::WallStraight.into());
    sprite.custom_size = Some(Vec2::splat(1.0));
    sprite.anchor = Anchor::BottomLeft;
    sprite
}
