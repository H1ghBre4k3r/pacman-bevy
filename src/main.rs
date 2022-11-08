mod ascii;
mod map;
mod player;

use std::fs;

use ascii::load_ascii;
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
use map::MapPlugin;
use player::PlayerPlugin;

const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

const TILE_SIZE: f32 = 40.0;

const COLUMS: u32 = 32;
const ROWS: u32 = 18;

const SCREEN_WIDTH: f32 = TILE_SIZE * COLUMS as f32;
const SCREEN_HEIGHT: f32 = TILE_SIZE * ROWS as f32;

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
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .run();
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
