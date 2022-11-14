mod tile;
mod tilemap;

pub use tile::*;
pub use tilemap::*;

use std::f32::consts::{FRAC_PI_2, PI};

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
                    determine_sprites_for_wall(&mut commands, ascii.0.clone(), *tile, &map, x, y);
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

const GENERAL_OFFSET: Vec3 = Vec3 {
    x: 0.25,
    y: 0.25,
    z: 0.0,
};

#[derive(Component)]
struct WallBundle;

/// Determine the sprites for a wall depending on the sprites around it.
fn determine_sprites_for_wall(
    commands: &mut Commands,
    texture_atlas: Handle<TextureAtlas>,
    tile: Tile,
    tiles: &TileMap,
    x: usize,
    y: usize,
) {
    let mut straight_wall = TextureAtlasSprite::new(SpriteIndices::WallStraight.into());
    straight_wall.custom_size = Some(Vec2::splat(0.5));

    let mut corner_wall = TextureAtlasSprite::new(SpriteIndices::WallCorner.into());
    corner_wall.custom_size = Some(Vec2::splat(0.5));

    commands
        .spawn(WallBundle)
        .insert(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3 {
                    x: x as f32,
                    y: y as f32,
                    z: 1.0,
                } + GENERAL_OFFSET,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // top left
            parent.spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3 {
                        y: 0.5,
                        z: 1.0,
                        ..default()
                    },
                    scale: Vec3::new(1.0, 1.0, 0.0),
                    ..default()
                },
                sprite: corner_wall.clone(),
                texture_atlas: texture_atlas.clone(),
                ..default()
            });

            // top right
            parent.spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: 0.5,
                        y: 0.5,
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

            // bottom right
            parent.spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: 0.5,
                        z: 1.0,
                        ..default()
                    },
                    scale: Vec3::new(1.0, 1.0, 0.0),
                    rotation: Quat::from_rotation_z(PI),
                    ..default()
                },
                sprite: corner_wall.clone(),
                texture_atlas: texture_atlas.clone(),
                ..default()
            });

            // bottom left
            parent.spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3 {
                        z: 1.0,
                        ..default()
                    },
                    scale: Vec3::new(1.0, 1.0, 0.0),
                    rotation: Quat::from_rotation_z(FRAC_PI_2) + Quat::from_rotation_z(FRAC_PI_2),
                    ..default()
                },
                sprite: corner_wall.clone(),
                texture_atlas: texture_atlas.clone(),
                ..default()
            });
        });
}

fn determine_sprite_for_wall_part(
    one: Option<Tile>,
    two: Option<Tile>,
    three: Option<Tile>,
) -> Option<(SpriteIndices, f32)> {
    match one {
        Some(Tile::Wall) => match two {
            Some(Tile::Wall) => match three {
                Some(Tile::Wall) => None,
                Some(_) => Some((SpriteIndices::WallStraight, FRAC_PI_2)),
                _ => todo!(),
            },
            Some(_) => match three {
                Some(Tile::Wall) => Some((SpriteIndices::WallCorner, PI)),
                Some(_) => Some((SpriteIndices::WallStraight, FRAC_PI_2)),
                _ => todo!(),
            },
            _ => todo!(),
        },
        Some(_) => match two {
            Some(Tile::Wall) => match three {
                Some(Tile::Wall) => Some((SpriteIndices::WallStraight, 0.0)),
                Some(_) => Some((SpriteIndices::WallCorner, 0.0)),
                _ => todo!(),
            },
            Some(_) => match three {
                Some(Tile::Wall) => Some((SpriteIndices::WallStraight, 0.0)),
                Some(_) => Some((SpriteIndices::WallCorner, 0.0)),
                _ => todo!(),
            },
            _ => todo!(),
        },
        _ => todo!(),
    }
}

mod tests {
    use std::f32::consts::{FRAC_PI_2, PI};

    use crate::ascii::SpriteIndices;

    use super::{determine_sprite_for_wall_part, Tile};

    #[test]
    fn test_empty_corner() {
        assert_eq!(
            determine_sprite_for_wall_part(Some(Tile::Wall), Some(Tile::Wall), Some(Tile::Wall)),
            None
        );
    }

    #[test]
    fn test_horizontal_wall() {
        assert_eq!(
            determine_sprite_for_wall_part(Some(Tile::Wall), Some(Tile::Wall), Some(Tile::Empty)),
            Some((SpriteIndices::WallStraight, FRAC_PI_2))
        );
        assert_eq!(
            determine_sprite_for_wall_part(Some(Tile::Wall), Some(Tile::Empty), Some(Tile::Empty)),
            Some((SpriteIndices::WallStraight, FRAC_PI_2))
        );
    }

    #[test]
    fn test_vertical_wall() {
        assert_eq!(
            determine_sprite_for_wall_part(Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Wall)),
            Some((SpriteIndices::WallStraight, 0.0))
        );
        assert_eq!(
            determine_sprite_for_wall_part(Some(Tile::Empty), Some(Tile::Wall), Some(Tile::Wall)),
            Some((SpriteIndices::WallStraight, 0.0))
        );
    }

    #[test]
    fn test_bottom_to_right_corner_wall() {
        assert_eq!(
            determine_sprite_for_wall_part(Some(Tile::Empty), Some(Tile::Wall), Some(Tile::Empty)),
            Some((SpriteIndices::WallCorner, 0.0))
        );
        assert_eq!(
            determine_sprite_for_wall_part(Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Empty)),
            Some((SpriteIndices::WallCorner, 0.0))
        );
    }

    #[test]
    fn test_left_to_top_corner_wall() {
        assert_eq!(
            determine_sprite_for_wall_part(Some(Tile::Wall), Some(Tile::Empty), Some(Tile::Wall)),
            Some((SpriteIndices::WallCorner, PI))
        );
    }
}
