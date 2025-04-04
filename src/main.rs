mod ascii;
mod entities;
mod gameloop;
mod lighthouse;
mod map;
mod view;

use ascii::load_ascii;
use bevy::prelude::*;
use dotenv::dotenv;
use entities::*;
use gameloop::GameLoop;
use lighthouse::LighthousePlugin;
use map::MapPlugin;
use view::{ViewConfigurationPlugin, SCREEN_HEIGHT, SCREEN_WIDTH};

macro_rules! get_env {
    ($name:expr) => {
        std::env::var($name).expect(&format!("{} should be given", $name))
    };
    ($name:expr, $default:expr) => {
        std::env::var($name).unwrap_or($default.into())
    };
}

fn main() {
    _ = dotenv().ok();

    App::new()
        .add_systems(PreStartup, load_ascii)
        .add_plugins(EntityPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(ViewConfigurationPlugin)
        .add_plugins(GameLoop)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        title: "Pacman Bevy".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LighthousePlugin {
            token: get_env!("LH_TOKEN"),
            user: get_env!("LH_USER"),
        })
        .add_systems(Update, close_on_esc)
        .run();
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
