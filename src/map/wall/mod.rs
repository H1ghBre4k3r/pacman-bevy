mod wall_part;

pub use self::wall_part::*;

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::ascii::SpriteIndices;

use super::TileMap;

const GENERAL_OFFSET: Vec3 = Vec3 {
    x: 0.25,
    y: 0.25,
    z: 0.0,
};

/// Component representing a wall tile on the map.
/// It has (or at least should have) 4 children (one for each corner).
#[derive(Component)]
pub struct WallTile;

/// Determine the sprites for a wall depending on the sprites around it.
pub fn spawn_sprites_for_wall(
    commands: &mut Commands,
    texture_atlas: Handle<TextureAtlas>,
    tiles: &TileMap,
    x: i32,
    y: i32,
) {
    let mut straight_wall = TextureAtlasSprite::new(SpriteIndices::WallStraight.into());
    straight_wall.custom_size = Some(Vec2::splat(0.5));

    let mut corner_wall = TextureAtlasSprite::new(SpriteIndices::WallCorner.into());
    corner_wall.custom_size = Some(Vec2::splat(0.5));

    commands
        .spawn(WallTile)
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
            if let Some((sprite_index, rotation)) = WallPart::determine_sprite_for_wall_part(
                tiles.at(x - 1, y),
                tiles.at(x - 1, y + 1),
                tiles.at(x, y + 1),
            ) {
                let sprite = TextureAtlasSprite {
                    index: sprite_index.into(),
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                };
                parent.spawn(WallPart::TopLeft).insert(SpriteSheetBundle {
                    sprite,
                    texture_atlas: texture_atlas.clone(),
                    transform: Transform {
                        translation: Vec3 {
                            y: 0.5,
                            z: 1.0,
                            ..default()
                        },
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        rotation: Quat::from_rotation_z(rotation),
                        ..default()
                    },
                    ..default()
                });
            }

            // top right
            if let Some((sprite_index, rotation)) = WallPart::determine_sprite_for_wall_part(
                tiles.at(x, y + 1),
                tiles.at(x + 1, y + 1),
                tiles.at(x + 1, y),
            ) {
                let sprite = TextureAtlasSprite {
                    index: sprite_index.into(),
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                };
                parent.spawn(WallPart::TopRight).insert(SpriteSheetBundle {
                    sprite,
                    texture_atlas: texture_atlas.clone(),
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.5,
                            y: 0.5,
                            z: 1.0,
                        },
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        rotation: Quat::from_rotation_z(-FRAC_PI_2 + rotation),
                        ..default()
                    },
                    ..default()
                });
            }

            // bottom right
            if let Some((sprite_index, rotation)) = WallPart::determine_sprite_for_wall_part(
                tiles.at(x + 1, y),
                tiles.at(x + 1, y - 1),
                tiles.at(x, y - 1),
            ) {
                let sprite = TextureAtlasSprite {
                    index: sprite_index.into(),
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                };
                parent
                    .spawn(WallPart::BottomRight)
                    .insert(SpriteSheetBundle {
                        sprite,
                        texture_atlas: texture_atlas.clone(),
                        transform: Transform {
                            translation: Vec3 {
                                x: 0.5,
                                z: 1.0,
                                ..default()
                            },
                            scale: Vec3::new(1.0, 1.0, 0.0),
                            rotation: Quat::from_rotation_z(PI + rotation),
                            ..default()
                        },
                        ..default()
                    });
            }

            // bottom left
            if let Some((sprite_index, rotation)) = WallPart::determine_sprite_for_wall_part(
                tiles.at(x, y - 1),
                tiles.at(x - 1, y - 1),
                tiles.at(x - 1, y),
            ) {
                let sprite = TextureAtlasSprite {
                    index: sprite_index.into(),
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                };
                parent
                    .spawn(WallPart::BottomLeft)
                    .insert(SpriteSheetBundle {
                        sprite,
                        texture_atlas: texture_atlas.clone(),
                        transform: Transform {
                            translation: Vec3 {
                                z: 1.0,
                                ..default()
                            },
                            scale: Vec3::new(1.0, 1.0, 0.0),
                            rotation: Quat::from_rotation_z(FRAC_PI_2 + rotation),
                            ..default()
                        },
                        ..default()
                    });
            }
        });
}
