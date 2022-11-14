use bevy::{prelude::*, sprite::Anchor};

use crate::ascii::{AsciiSheet, SpriteIndices};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

#[derive(Component)]
struct Pacman;

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(SpriteIndices::PacmanOpen.into());
    sprite.custom_size = Some(Vec2::splat(1.));
    sprite.anchor = Anchor::BottomLeft;

    commands.spawn(Pacman).insert(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(1.0, 1.0, 10.0),
            scale: Vec3::new(1.0, 1.0, 0.0),
            ..default()
        },
        sprite,
        texture_atlas: ascii.0.clone(),
        ..default()
    });
}
