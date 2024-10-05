use bevy::{prelude::*, sprite::Anchor};

use crate::ascii::{AsciiSheet, SpriteIndices};

#[derive(Component)]
pub struct Coin;

/// Spawn a coin at the given location
pub fn spawn_coin(commands: &mut Commands, ascii: &Res<AsciiSheet>, x: i32, y: i32) {
    let sprite = Sprite {
        custom_size: Some(Vec2::splat(1.0)),
        anchor: Anchor::BottomLeft,
        ..default()
    };

    let atlas = TextureAtlas {
        index: SpriteIndices::SmallCoin.into(),
        layout: ascii.layout.clone(),
    };

    commands.spawn(Coin).insert((
        SpriteBundle {
            sprite,
            transform: Transform {
                translation: Vec3 {
                    x: x as f32,
                    y: y as f32,
                    z: 1.0,
                },
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..default()
            },
            texture: ascii.image.clone(),
            ..default()
        },
        atlas,
    ));
}
