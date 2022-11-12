use std::{
    f32::consts::{FRAC_PI_2, PI},
    fs,
};

use bevy::{prelude::*, sprite::Anchor};

use crate::ascii::{AsciiSheet, SpriteIdices};

pub struct MapPlugin;

/// Plugin for managing the map load and instantiation of tiles.
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap::from_string("assets/map.txt"))
            .add_startup_system(spawn_tiles);
    }
}

#[derive(Clone)]
pub struct TileMap {
    _tiles: Vec<Vec<Tile>>,
}

impl From<Vec<Vec<Tile>>> for TileMap {
    fn from(tiles: Vec<Vec<Tile>>) -> Self {
        Self { _tiles: tiles }
    }
}

impl TileMap {
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

        TileMap::from(columns)
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self._tiles
    }

    /// Get the tile at the specified position. If there is no tile or the indices are negative,
    /// this function returns None.
    pub fn at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        match self._tiles.get(x as usize) {
            Some(column) => {
                return column.get(y as usize).map(|tile| tile.clone());
            }
            None => None,
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
fn spawn_tiles(mut commands: Commands, map: Res<TileMap>, ascii: Res<AsciiSheet>) {
    let tiles = map.tiles();

    for (x, column) in tiles.iter().enumerate() {
        for (y, tile) in column.iter().enumerate() {
            let sprite = match *tile {
                Tile::Wall => {
                    determine_sprites_for_wall(&mut commands, ascii.0.clone(), *tile, &map, x, y);
                    continue;
                }
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
fn determine_sprites_for_wall(
    commands: &mut Commands,
    texture_atlas: Handle<TextureAtlas>,
    tile: Tile,
    tiles: &TileMap,
    x: usize,
    y: usize,
) {
    let mut straight_wall = TextureAtlasSprite::new(SpriteIdices::WallStraight.into());
    straight_wall.custom_size = Some(Vec2::splat(0.5));
    straight_wall.anchor = Anchor::BottomLeft;

    let mut corner_wall = TextureAtlasSprite::new(SpriteIdices::WallCorner.into());
    corner_wall.custom_size = Some(Vec2::splat(0.5));
    corner_wall.anchor = Anchor::BottomLeft;

    commands.spawn().insert_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3 {
                x: x as f32 + 0.5,
                y: y as f32,
                z: 1.0,
            },
            scale: Vec3::new(1.0, 1.0, 0.0),
            rotation: Quat::from_rotation_z(FRAC_PI_2),
            ..default()
        },
        sprite: corner_wall.clone(),
        texture_atlas: texture_atlas.clone(),
        ..default()
    });

    commands.spawn().insert_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3 {
                x: x as f32 + 1.0,
                y: y as f32 + 0.5,
                z: 1.0,
            },
            scale: Vec3::new(1.0, 1.0, 0.0),
            rotation: Quat::from_rotation_z(PI),
            ..default()
        },
        sprite: corner_wall.clone(),
        texture_atlas: texture_atlas.clone(),
        ..default()
    });

    commands.spawn().insert_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3 {
                x: x as f32 + 0.5,
                y: y as f32 + 1.0,
                z: 1.0,
            },
            scale: Vec3::new(1.0, 1.0, 0.0),
            rotation: Quat::from_rotation_z(-FRAC_PI_2),
            ..default()
        },
        sprite: corner_wall.clone(),
        texture_atlas: texture_atlas.clone(),
        ..default()
    });

    commands.spawn().insert_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3 {
                x: x as f32,
                y: y as f32 + 0.5,
                z: 1.0,
            },
            scale: Vec3::new(1.0, 1.0, 0.0),
            ..default()
        },
        sprite: corner_wall.clone(),
        texture_atlas: texture_atlas.clone(),
        ..default()
    });
}
