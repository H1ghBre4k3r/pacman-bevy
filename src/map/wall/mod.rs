mod wall_part;

pub use self::wall_part::*;

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{prelude::*, sprite::Anchor};

use crate::ascii::{AsciiSheet, SpriteIndices};

use super::TileMap;

const GENERAL_OFFSET: Vec3 = Vec3 {
    x: 0.25,
    y: 0.25,
    z: 0.0,
};

/// Component representing a wall on the map.
/// It has (or at least should have) 4 children (one for each corner).
#[derive(Component)]
pub struct WallTile;

/// Determine the sprites for a wall depending on the sprites around it.
pub fn spawn_sprites_for_wall(
    commands: &mut Commands,
    ascii: &Res<AsciiSheet>,
    tiles: &TileMap,
    x: i32,
    y: i32,
) {
    let layout = ascii.layout.clone();
    let texture = ascii.image.clone();

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
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            },
            texture: texture.clone(),
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: SpriteIndices::Empty.into(),
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
                let sprite = Sprite {
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                };
                let atlas = TextureAtlas {
                    layout: layout.clone(),
                    index: sprite_index.into(),
                };

                parent.spawn(WallPart::TopLeft).insert(SpriteSheetBundle {
                    sprite,
                    atlas,
                    transform: Transform {
                        translation: Vec3 {
                            y: 0.5,
                            z: 1.0,
                            ..default()
                        },
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        rotation: Quat::from_rotation_z(rotation),
                    },
                    texture: texture.clone(),
                    ..default()
                });
            }

            // top right
            if let Some((sprite_index, rotation)) = WallPart::determine_sprite_for_wall_part(
                tiles.at(x, y + 1),
                tiles.at(x + 1, y + 1),
                tiles.at(x + 1, y),
            ) {
                let sprite = Sprite {
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                };
                let atlas = TextureAtlas {
                    layout: layout.clone(),
                    index: sprite_index.into(),
                };

                parent.spawn(WallPart::TopRight).insert(SpriteSheetBundle {
                    sprite,
                    atlas,
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.5,
                            y: 0.5,
                            z: 1.0,
                        },
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        rotation: Quat::from_rotation_z(-FRAC_PI_2 + rotation),
                    },
                    texture: texture.clone(),
                    ..default()
                });
            }

            // bottom right
            if let Some((sprite_index, rotation)) = WallPart::determine_sprite_for_wall_part(
                tiles.at(x + 1, y),
                tiles.at(x + 1, y - 1),
                tiles.at(x, y - 1),
            ) {
                let sprite = Sprite {
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                };

                let atlas = TextureAtlas {
                    layout: layout.clone(),
                    index: sprite_index.into(),
                };

                parent
                    .spawn(WallPart::BottomRight)
                    .insert(SpriteSheetBundle {
                        sprite,
                        atlas,
                        transform: Transform {
                            translation: Vec3 {
                                x: 0.5,
                                z: 1.0,
                                ..default()
                            },
                            scale: Vec3::new(1.0, 1.0, 0.0),
                            rotation: Quat::from_rotation_z(PI + rotation),
                        },
                        texture: texture.clone(),
                        ..default()
                    });
            }

            // bottom left
            if let Some((sprite_index, rotation)) = WallPart::determine_sprite_for_wall_part(
                tiles.at(x, y - 1),
                tiles.at(x - 1, y - 1),
                tiles.at(x - 1, y),
            ) {
                let sprite = Sprite {
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                };

                let atlas = TextureAtlas {
                    layout: layout.clone(),
                    index: sprite_index.into(),
                };
                parent
                    .spawn(WallPart::BottomLeft)
                    .insert(SpriteSheetBundle {
                        sprite,
                        atlas,
                        transform: Transform {
                            translation: Vec3 {
                                z: 1.0,
                                ..default()
                            },
                            scale: Vec3::new(1.0, 1.0, 0.0),
                            rotation: Quat::from_rotation_z(FRAC_PI_2 + rotation),
                        },
                        texture,
                        ..default()
                    });
            }
        });
}
