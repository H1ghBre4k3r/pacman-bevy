mod direction;

pub use self::direction::*;

use bevy::{prelude::*, sprite::Anchor};

use crate::ascii::{AsciiSheet, SpriteIndices};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(check_for_input);
    }
}

#[derive(Component)]
struct Pacman;

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(SpriteIndices::PacmanOpen.into());
    sprite.custom_size = Some(Vec2::splat(1.));
    sprite.anchor = Anchor::BottomLeft;

    commands
        .spawn(Pacman)
        .insert(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(1.0, 1.0, 10.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..default()
            },
            sprite,
            texture_atlas: ascii.0.clone(),
            ..default()
        })
        .insert(DirectionWrapper::default());
}

fn check_for_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut direction_query: Query<&mut DirectionWrapper, With<Pacman>>,
) {
    let mut direction = direction_query.get_single_mut().unwrap();

    // TODO: should this be exclusive?
    if keyboard_input.just_pressed(KeyCode::W) {
        direction.set(Some(MovementDirection::Up));
    }
    if keyboard_input.just_pressed(KeyCode::A) {
        direction.set(Some(MovementDirection::Left));
    }
    if keyboard_input.just_pressed(KeyCode::S) {
        direction.set(Some(MovementDirection::Down));
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        direction.set(Some(MovementDirection::Right));
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        direction.set(None);
    }
}
