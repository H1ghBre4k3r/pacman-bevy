use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor, time::common_conditions::on_timer};

use crate::{
    ascii::{AsciiSheet, SpriteIndices},
    lighthouse::{LighthouseBundle, LighthouseColor, LighthousePosition},
    DirectionWrapper,
};

pub struct GhostPlugin;

const GHOST_TICK_TIME: f64 = 1.0 / 5.0;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ghosts)
            .add_systems(
                Update,
                animate_ghost_sprite.run_if(on_timer(Duration::from_secs_f64(GHOST_TICK_TIME))),
            )
            .add_systems(Update, update_lighthouse_position);
    }
}

#[derive(Component)]
pub struct Ghost;

#[derive(Clone, Copy, Component)]
pub enum GhostType {
    Blinky,
    Inky,
    Pinky,
    Clyde,
}

impl From<GhostType> for SpriteIndices {
    fn from(value: GhostType) -> Self {
        match value {
            GhostType::Blinky => SpriteIndices::Blinky1,
            GhostType::Inky => SpriteIndices::Inky1,
            GhostType::Pinky => SpriteIndices::Pinky1,
            GhostType::Clyde => SpriteIndices::Clyde1,
        }
    }
}

impl From<GhostType> for LighthouseColor {
    fn from(value: GhostType) -> Self {
        match value {
            GhostType::Blinky => LighthouseColor::Inline(214, 62, 31),
            GhostType::Inky => LighthouseColor::Inline(109, 209, 253),
            GhostType::Pinky => LighthouseColor::Inline(227, 126, 249),
            GhostType::Clyde => LighthouseColor::Inline(219, 168, 12),
        }
    }
}

fn spawn_ghosts(mut commands: Commands, ascii: Res<AsciiSheet>) {
    spawn_specific_ghost(&mut commands, &ascii, GhostType::Blinky, 5, 6);
    spawn_specific_ghost(&mut commands, &ascii, GhostType::Inky, 5, 8);
    spawn_specific_ghost(&mut commands, &ascii, GhostType::Pinky, 8, 8);
    spawn_specific_ghost(&mut commands, &ascii, GhostType::Clyde, 8, 6);
}

fn spawn_specific_ghost(
    commands: &mut Commands,
    ascii: &Res<AsciiSheet>,
    ghost: GhostType,
    x: usize,
    y: usize,
) {
    let layout = ascii.layout.clone();
    let texture = ascii.image.clone();

    let sprite = Sprite {
        custom_size: Some(Vec2::splat(1.)),
        anchor: Anchor::BottomLeft,
        ..default()
    };

    let atlas = TextureAtlas {
        layout,
        index: SpriteIndices::from(ghost).into(),
    };

    commands
        .spawn(Ghost)
        .insert(ghost)
        .insert((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x as f32, y as f32, 10.0),
                    scale: Vec3::new(1.0, 1.0, 0.0),
                    ..default()
                },
                sprite: sprite.clone(),
                texture: texture.clone(),
                ..default()
            },
            atlas,
        ))
        .insert(LighthouseBundle {
            position: LighthousePosition { x, y, z: 2 },
            color: ghost.into(),
        })
        .insert(DirectionWrapper::default());
}

fn update_lighthouse_position(
    mut query: Query<(&Transform, &mut LighthousePosition), With<Ghost>>,
) {
    for (Transform { translation, .. }, mut position) in query.iter_mut() {
        position.x = translation.x.max(0.0) as usize;
        position.y = translation.y.max(0.0) as usize;
        position.z = translation.z.max(0.0) as usize;
    }
}

fn animate_ghost_sprite(mut ghost_query: Query<&mut TextureAtlas, With<Ghost>>) {
    for mut sprite in ghost_query.iter_mut() {
        if sprite.index % 2 == 0 {
            sprite.index += 1;
        } else {
            sprite.index -= 1;
        }
    }
}
