mod ascii;
mod gameloop;
mod map;
mod player;
mod view;

use ascii::load_ascii;
use bevy::prelude::*;
use gameloop::GameLoop;
use map::MapPlugin;
use player::PlayerPlugin;
use view::{ViewConfigurationPlugin, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    App::new()
        .add_systems(PreStartup, load_ascii)
        .add_plugins(PlayerPlugin)
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
