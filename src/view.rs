use bevy::{prelude::*, render::camera::ScalingMode};

const CLEAR_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

const TILE_SIZE: f32 = 40.0;

// pub const COLUMNS: u32 = 19;
// pub const ROWS: u32 = 22;

pub const COLUMNS: u32 = 14;
pub const ROWS: u32 = 14;

pub const SCREEN_WIDTH: f32 = TILE_SIZE * COLUMNS as f32;
pub const SCREEN_HEIGHT: f32 = TILE_SIZE * ROWS as f32;

/// Struct for configuring the view etc.
pub struct ViewConfigurationPlugin;

impl Plugin for ViewConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .add_systems(Startup, spawn_camera);
    }
}

/// spawn 2D camera and align it in the positive-positive quadrant
fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // set scaling (i.e., viewport) to given columns and rows
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: COLUMNS as f32,
        height: ROWS as f32,
    };

    // move camera to upper right
    camera.transform.translation = Vec3 {
        x: (COLUMNS as f32) / 2.,
        y: (ROWS as f32) / 2.,
        z: 1000.,
    };

    commands.spawn(camera);
}
