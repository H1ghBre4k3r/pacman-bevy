use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_fixed_timer};

use crate::{
    map::{Coin, WallTile},
    player::{DirectionWrapper, Pacman},
    view::{COLUMNS, ROWS},
};

const TICK_TIME: f64 = 1.0 / 2.0;

/// Plugin for managing the game loop of the game
pub struct GameLoop;

impl Plugin for GameLoop {
    fn build(&self, app: &mut App) {
        app.add_system(move_player.run_if(on_fixed_timer(Duration::from_millis(
            (TICK_TIME * 1000.0) as u64,
        ))))
        .add_system(eat_coin.after(move_player));
    }
}

/// Move the player according to its current position
fn move_player(
    mut transform_direction_query: Query<(&mut Transform, &DirectionWrapper), With<Pacman>>,
    wall_query: Query<&Transform, (With<WallTile>, Without<Pacman>)>,
) {
    let (mut transform, direction_wrapper) = transform_direction_query.single_mut();
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

type PacmanQuery<'world, 'state, 'a> =
    Query<'world, 'state, &'a Transform, (With<Pacman>, Without<Coin>, Without<WallTile>)>;

type CoinQuery<'world, 'state, 'a> = Query<
    'world,
    'state,
    (&'a Transform, Entity),
    (With<Coin>, Without<Pacman>, Without<WallTile>),
>;

/// Eat the coin at the current location of pacman
fn eat_coin(mut commands: Commands, pacman_query: PacmanQuery, coins: CoinQuery) {
    // convert coordinate to u32 to avoid floating point errors
    let transform = pacman_query.single();
    let x = transform.translation.x as u32;
    let y = transform.translation.y as u32;

    // check if there is a coin at the current position
    for (position, coin) in &coins {
        if x == position.translation.x as u32 && y == position.translation.y as u32 {
            commands.entity(coin).despawn();
        }
    }
}
