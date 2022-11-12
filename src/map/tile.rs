use bevy::prelude::*;

/// A tile within the game.
#[derive(Component, Clone, Copy)]
pub enum Tile {
    Empty,
    Wall,
    Coin,
}

impl Tile {
    pub fn from_char(character: char) -> Self {
        match character {
            ' ' => Self::Empty,
            '#' => Self::Wall,
            '.' => Self::Coin,
            _ => unreachable!(),
        }
    }
}
