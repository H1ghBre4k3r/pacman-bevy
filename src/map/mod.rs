mod tile;
mod tilemap;

pub use tile::*;
pub use tilemap::*;

use std::f32::consts::{FRAC_PI_2, PI};

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

/// Spawn tiles depending on the loaded map.
fn spawn_tiles(mut commands: Commands, map: Res<TileMap>, ascii: Res<AsciiSheet>) {
    let columns = map.columns();

    for (x, column) in columns.enumerate() {
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

const GENERAL_OFFSET: Vec3 = Vec3 {
    x: 0.25,
    y: 0.25,
    z: 0.0,
};

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

    let mut corner_wall = TextureAtlasSprite::new(SpriteIdices::WallCorner.into());
    corner_wall.custom_size = Some(Vec2::splat(0.5));

    // bottom left
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3 {
                x: x as f32,
                y: y as f32,
                z: 1.0,
            } + GENERAL_OFFSET,
            scale: Vec3::new(1.0, 1.0, 0.0),
            rotation: Quat::from_rotation_z(FRAC_PI_2),
            ..default()
        },
        sprite: corner_wall.clone(),
        texture_atlas: texture_atlas.clone(),
        ..default()
    });

    // bottom right
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3 {
                x: x as f32 + 0.5,
                y: y as f32,
                z: 1.0,
            } + GENERAL_OFFSET,
            scale: Vec3::new(1.0, 1.0, 0.0),
            rotation: Quat::from_rotation_z(PI),
            ..default()
        },
        sprite: corner_wall.clone(),
        texture_atlas: texture_atlas.clone(),
        ..default()
    });

    // top right
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3 {
                x: x as f32 + 0.5,
                y: y as f32 + 0.5,
                z: 1.0,
            } + GENERAL_OFFSET,
            scale: Vec3::new(1.0, 1.0, 0.0),
            rotation: Quat::from_rotation_z(-FRAC_PI_2),
            ..default()
        },
        sprite: corner_wall.clone(),
        texture_atlas: texture_atlas.clone(),
        ..default()
    });

    // top left
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3 {
                x: x as f32,
                y: y as f32 + 0.5,
                z: 1.0,
            } + GENERAL_OFFSET,
            scale: Vec3::new(1.0, 1.0, 0.0),
            ..default()
        },
        sprite: corner_wall.clone(),
        texture_atlas: texture_atlas.clone(),
        ..default()
    });
}
