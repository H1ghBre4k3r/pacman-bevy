mod direction;

use std::f32::consts::{FRAC_PI_2, PI};

pub use self::direction::*;

use bevy::{prelude::*, sprite::Anchor, time::FixedTimestep};

use crate::ascii::{AsciiSheet, SpriteIndices};

pub struct PlayerPlugin;

const TICK_TIME: f64 = 1.0 / 4.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(check_for_input)
            .add_system(rotate_pacman_head.after(check_for_input))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TICK_TIME))
                    .with_system(change_pacman_mouth),
            );
    }
}

/// Component for representing Pacman
#[derive(Component)]
pub struct Pacman;

/// Spawn a new player entity and all its components
fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(SpriteIndices::PacmanClosed.into());
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

/// Check, if there are any important keys pressed by the user.
fn check_for_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut direction_query: Query<&mut DirectionWrapper, With<Pacman>>,
) {
    let mut direction = direction_query.single_mut();

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

/// Rotate the head of pacman according to the current direction
fn rotate_pacman_head(
    mut pacman_query: Query<
        (&mut Transform, &DirectionWrapper, &mut TextureAtlasSprite),
        With<Pacman>,
    >,
) {
    let (mut transformation, direction_wrapper, mut sprite) = pacman_query.single_mut();
    let direction = direction_wrapper.direction;
    let Some(direction) = direction else {
        return;
    };

    let (rotation, anchor) = match direction {
        MovementDirection::Right => (Quat::default(), Anchor::BottomLeft),
        MovementDirection::Up => (Quat::from_rotation_z(FRAC_PI_2), Anchor::TopLeft),
        MovementDirection::Down => (Quat::from_rotation_z(-FRAC_PI_2), Anchor::BottomRight),
        MovementDirection::Left => (Quat::from_rotation_z(PI), Anchor::TopRight),
    };
    transformation.rotation = rotation;
    sprite.anchor = anchor;
}

fn change_pacman_mouth(mut pacman_query: Query<&mut TextureAtlasSprite, With<Pacman>>) {
    let mut sprite = pacman_query.single_mut();
    let open_mouth: usize = SpriteIndices::PacmanOpen.into();
    let closed_mouth: usize = SpriteIndices::PacmanClosed.into();

    if sprite.index == open_mouth {
        sprite.index = closed_mouth;
    } else {
        sprite.index = open_mouth;
    }
}
