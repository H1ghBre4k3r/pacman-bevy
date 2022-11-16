mod ascii;
mod gameloop;
mod map;
mod player;
mod view;

use ascii::load_ascii;
use bevy::prelude::*;
use map::MapPlugin;
use player::PlayerPlugin;
use view::{ViewConfigurationPlugin, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    App::new()
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(ViewConfigurationPlugin)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: SCREEN_WIDTH,
                        height: SCREEN_HEIGHT,
                        title: "Pacman Bevy".to_string(),
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}
