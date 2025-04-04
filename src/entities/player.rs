use std::{
    f32::consts::{FRAC_PI_2, PI},
    time::Duration,
};

use bevy::{prelude::*, sprite::Anchor, time::common_conditions::on_timer};

use crate::{
    ascii::{AsciiSheet, SpriteIndices},
    lighthouse::{LighthouseBundle, LighthouseColor, LighthousePosition},
    DirectionWrapper, MovementDirection,
};

pub struct PlayerPlugin;

const TICK_TIME: f64 = 1.0 / 4.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (check_for_input, rotate_pacman_head).chain())
            .add_systems(
                Update,
                change_pacman_mouth.run_if(on_timer(Duration::from_secs_f64(TICK_TIME))),
            )
            .add_systems(Update, update_lighthouse_position);
    }
}

/// Component for representing Pacman
#[derive(Component)]
pub struct Pacman;

/// Spawn a new player entity and all its components
fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let layout = ascii.layout.clone();
    let texture = ascii.image.clone();

    let sprite = Sprite {
        custom_size: Some(Vec2::splat(1.)),
        anchor: Anchor::BottomLeft,
        ..default()
    };

    let atlas = TextureAtlas {
        layout,
        index: SpriteIndices::PacmanClosed.into(),
    };

    commands
        .spawn(Pacman)
        .insert((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(1.0, 1.0, 10.0),
                    scale: Vec3::new(1.0, 1.0, 0.0),
                    ..default()
                },
                sprite,
                texture,
                ..default()
            },
            atlas,
        ))
        .insert(DirectionWrapper::default())
        .insert(LighthouseBundle {
            position: LighthousePosition { x: 1, y: 1, z: 10 },
            color: LighthouseColor::Inline(255, 255, 0),
        });
}

fn update_lighthouse_position(
    mut query: Query<(&Transform, &mut LighthousePosition), With<Pacman>>,
) {
    let (Transform { translation, .. }, mut position) = query.single_mut();
    position.x = translation.x.max(0.0) as usize;
    position.y = translation.y.max(0.0) as usize;
    position.z = translation.z.max(0.0) as usize;
}

/// Check, if there are any important keys pressed by the user.
fn check_for_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction_query: Query<&mut DirectionWrapper, With<Pacman>>,
) {
    let mut direction = direction_query.single_mut();

    // TODO: should this be exclusive?
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        direction.set(Some(MovementDirection::Up));
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        direction.set(Some(MovementDirection::Left));
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        direction.set(Some(MovementDirection::Down));
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        direction.set(Some(MovementDirection::Right));
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        direction.set(None);
    }
}

/// Rotate the head of pacman according to the current direction
fn rotate_pacman_head(
    mut pacman_query: Query<(&mut Transform, &DirectionWrapper, &mut Sprite), With<Pacman>>,
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

fn change_pacman_mouth(mut pacman_query: Query<&mut TextureAtlas, With<Pacman>>) {
    let mut sprite = pacman_query.single_mut();
    let open_mouth: usize = SpriteIndices::PacmanOpen.into();
    let closed_mouth: usize = SpriteIndices::PacmanClosed.into();

    if sprite.index == open_mouth {
        sprite.index = closed_mouth;
    } else {
        sprite.index = open_mouth;
    }
}
