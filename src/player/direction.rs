use std::ops::Add;

use bevy::prelude::*;

/// Enum representing a direction an entity is currently moving in.
#[derive(Clone, Copy, Debug)]
pub enum MovementDirection {
    Up,
    Right,
    Left,
    Down,
}

/// Wrapper around the direction, which allows to dynamically change the direction.
#[derive(Component, Debug, Default)]
pub struct DirectionWrapper {
    /// Direction the entity is currently moving in.
    /// `None` means that the entity is not moving
    pub direction: Option<MovementDirection>,
}

impl DirectionWrapper {
    /// Update the direction. Passing `None` should be equivalent to "stopping" the entity.
    pub fn set(&mut self, direction: Option<MovementDirection>) {
        debug!("DirectionWrapper::set({:?})", direction);
        self.direction = direction;
    }
}

impl Add<MovementDirection> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: MovementDirection) -> Self::Output {
        let rhs: Vec3 = rhs.into();
        self + rhs
    }
}

impl Into<Vec3> for MovementDirection {
    /// Convert a MovementDirection into a Vec3
    fn into(self) -> Vec3 {
        match self {
            Self::Up => Vec3 {
                y: 1.0,
                ..default()
            },
            Self::Right => Vec3 {
                x: 1.0,
                ..default()
            },
            Self::Down => Vec3 {
                y: -1.0,
                ..default()
            },
            Self::Left => Vec3 {
                x: -1.0,
                ..default()
            },
        }
    }
}
