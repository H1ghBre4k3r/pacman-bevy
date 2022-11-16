use bevy::{prelude::*, render::camera::ScalingMode};

const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

const TILE_SIZE: f32 = 40.0;

pub const COLUMNS: u32 = 19;
pub const ROWS: u32 = 22;

pub const SCREEN_WIDTH: f32 = TILE_SIZE * COLUMNS as f32;
pub const SCREEN_HEIGHT: f32 = TILE_SIZE * ROWS as f32;

/// Struct for configuring the view etc.
pub struct ViewConfigurationPlugin;

impl Plugin for ViewConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .add_startup_system(spawn_camera);
    }
}

/// spawn 2D camera and align it in the positive-positive quadrant
fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::None;

    camera.projection.top = ROWS as f32;
    camera.projection.bottom = 0.0;

    camera.projection.right = COLUMNS as f32;
    camera.projection.left = 0.0;

    commands.spawn(camera);
}
