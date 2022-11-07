mod ascii;

use ascii::{load_ascii, AsciiSheet, SpriteIdices};
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
    sprite::Anchor,
};

const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

const TILE_SIZE: f32 = 40.0;

const COLUMS: u32 = 32;
const ROWS: u32 = 18;

const SCREEN_WIDTH: f32 = TILE_SIZE * COLUMS as f32;
const SCREEN_HEIGHT: f32 = TILE_SIZE * ROWS as f32;

#[derive(Component)]
struct Pacman;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Pacman Bevy".to_string(),
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(SpriteIdices::PacmanOpen.into());
    sprite.custom_size = Some(Vec2::splat(1.));
    sprite.anchor = Anchor::BottomLeft;

    commands
        .spawn()
        .insert(Pacman)
        .insert_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(1.0, 1.0, 1.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..default()
            },
            sprite,
            texture_atlas: ascii.0.clone(),
            ..default()
        });
}

/// spawn 2D camera and align it in the positive-positive quadrant
fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::None;

    camera.projection.top = ROWS as f32;
    camera.projection.bottom = 0.0;

    camera.projection.right = COLUMS as f32;
    camera.projection.left = 0.0;

    commands.spawn_bundle(camera);
}
