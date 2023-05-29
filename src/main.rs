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
        .add_startup_system(load_ascii.in_base_set(StartupSet::PreStartup))
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(ViewConfigurationPlugin)
        .add_plugin(GameLoop)
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
        .add_system(bevy::window::close_on_esc)
        .run();
}
