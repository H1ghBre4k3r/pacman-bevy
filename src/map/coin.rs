use bevy::{prelude::*, sprite::Anchor};

use crate::ascii::SpriteIndices;

#[derive(Component)]
struct Coin;

/// Spawn a coin at the given location
pub fn spawn_coin(commands: &mut Commands, texture_atlas: Handle<TextureAtlas>, x: i32, y: i32) {
    let mut sprite = TextureAtlasSprite::new(SpriteIndices::SmallCoin.into());
    sprite.custom_size = Some(Vec2::splat(1.0));
    sprite.anchor = Anchor::BottomLeft;
    commands.spawn(Coin).insert(SpriteSheetBundle {
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
        texture_atlas,
        ..default()
    });
}
