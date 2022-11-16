use bevy::{prelude::*, time::FixedTimestep};

use crate::player::{DirectionWrapper, Pacman};

const TICK_TIME: f64 = 1.0 / 2.0;

/// Plugin for managing the game loop of the game
pub struct GameLoop;

impl Plugin for GameLoop {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TICK_TIME))
                .with_system(move_player),
        );
    }
}

/// Move the player according to its current position
fn move_player(
    mut transform_direction_query: Query<(&mut Transform, &DirectionWrapper), With<Pacman>>,
) {
    let (mut transform, direction_wrapper) = transform_direction_query.get_single_mut().unwrap();
    let direction = direction_wrapper.direction;
    if let Some(direction) = direction {
        transform.translation += direction;
    }
}
