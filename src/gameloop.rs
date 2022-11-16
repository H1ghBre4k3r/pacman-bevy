use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    map::{WallPart, WallTile},
    player::{DirectionWrapper, Pacman},
    view::{COLUMNS, ROWS},
};

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
    wall_query: Query<&Transform, (With<WallTile>, Without<Pacman>)>,
) {
    let (mut transform, direction_wrapper) = transform_direction_query.get_single_mut().unwrap();
    let direction = direction_wrapper.direction;
    let Some(direction) = direction else {
        return;
    };
    // calculate the new position according to the current direction
    let mut new_position = transform.translation + direction;

    // we convert x and y to integers so we avoid floating point errors
    let x = new_position.x as i32;
    let y = new_position.y as i32;

    // check for collisions with each wall
    for wall in &wall_query {
        let wall_position = wall.translation;
        if wall_position.x.floor() as i32 == x && wall_position.y.floor() as i32 == y {
            return;
        }
    }

    // check for "going out of bounds"
    if x < 0 {
        new_position.x = (COLUMNS - 1) as f32;
    } else if x >= COLUMNS as i32 {
        new_position.x = 0.0;
    }

    if y < 0 {
        new_position.y = (ROWS - 1) as f32;
    } else if y >= ROWS as i32 {
        new_position.y = 0.0;
    }

    transform.translation = new_position;
}
