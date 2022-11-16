use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::{ascii::SpriteIndices, map::Tile};

/// Component representing a specific part of a wall.
#[derive(Component)]
pub enum WallPart {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl WallPart {
    /// Determine the correct sprite for a given wall part.
    /// The orientation is supposed to be the following (here for the top-left part):
    ///
    /// 2 3
    /// 1 X
    ///
    /// This allows you to "rotate" over all 4 corners of a wall part, e.g., for the top-right part:
    ///
    /// 1 2
    /// X 3
    ///
    /// The result is then either `None`, if there should be no sprite displayed in this sub part of the
    /// tile, or `Some((spriteIndex, rotation))`, where `spriteIndex` is the index of the sprite on the
    /// sprite sheet and `rotation` is the _additional_ rotation for this sheet to correctly align it.
    pub fn determine_sprite_for_wall_part(
        one: Option<Tile>,
        two: Option<Tile>,
        three: Option<Tile>,
    ) -> Option<(SpriteIndices, f32)> {
        match one {
            Some(Tile::Wall) => match two {
                Some(Tile::Wall) => match three {
                    Some(Tile::Wall) => None,
                    Some(_) => Some((SpriteIndices::WallStraight, FRAC_PI_2)),
                    _ => {
                        warn!("{one:?} {two:?} {three:?} not yet implemented!");
                        None
                    }
                },
                Some(_) => match three {
                    Some(Tile::Wall) => Some((SpriteIndices::WallCorner, PI)),
                    Some(_) => Some((SpriteIndices::WallStraight, FRAC_PI_2)),
                    _ => {
                        warn!("{one:?} {two:?} {three:?} not yet implemented!");
                        None
                    }
                },
                _ => {
                    warn!("{one:?} {two:?} {three:?} not yet implemented!");
                    None
                }
            },
            Some(_) => match two {
                Some(Tile::Wall) => match three {
                    Some(Tile::Wall) => Some((SpriteIndices::WallStraight, 0.0)),
                    Some(_) => Some((SpriteIndices::WallCorner, 0.0)),
                    _ => {
                        warn!("{one:?} {two:?} {three:?} not yet implemented!");
                        None
                    }
                },
                Some(_) => match three {
                    Some(Tile::Wall) => Some((SpriteIndices::WallStraight, 0.0)),
                    Some(_) => Some((SpriteIndices::WallCorner, 0.0)),
                    _ => {
                        warn!("{one:?} {two:?} {three:?} not yet implemented!");
                        None
                    }
                },
                None => match three {
                    _ => Some((SpriteIndices::WallStraight, 0.0)),
                },
            },
            None => match two {
                None => match three {
                    Some(Tile::Wall) => None,
                    Some(_) => Some((SpriteIndices::WallStraight, FRAC_PI_2)),
                    None => None,
                },
                _ => {
                    warn!("{one:?} {two:?} {three:?} not yet implemented!");
                    None
                }
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_PI_2, PI};

    use crate::{ascii::SpriteIndices, map::WallPart};

    use super::Tile;

    #[test]
    fn test_empty_corner() {
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(
                Some(Tile::Wall),
                Some(Tile::Wall),
                Some(Tile::Wall)
            ),
            None
        );
    }

    #[test]
    fn test_horizontal_wall() {
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(
                Some(Tile::Wall),
                Some(Tile::Wall),
                Some(Tile::Empty)
            ),
            Some((SpriteIndices::WallStraight, FRAC_PI_2))
        );
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(
                Some(Tile::Wall),
                Some(Tile::Empty),
                Some(Tile::Empty)
            ),
            Some((SpriteIndices::WallStraight, FRAC_PI_2))
        );
    }

    #[test]
    fn test_vertical_wall() {
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(
                Some(Tile::Empty),
                Some(Tile::Empty),
                Some(Tile::Wall)
            ),
            Some((SpriteIndices::WallStraight, 0.0))
        );
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(
                Some(Tile::Empty),
                Some(Tile::Wall),
                Some(Tile::Wall)
            ),
            Some((SpriteIndices::WallStraight, 0.0))
        );
    }

    #[test]
    fn test_bottom_to_right_corner_wall() {
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(
                Some(Tile::Empty),
                Some(Tile::Wall),
                Some(Tile::Empty)
            ),
            Some((SpriteIndices::WallCorner, 0.0))
        );
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(
                Some(Tile::Empty),
                Some(Tile::Empty),
                Some(Tile::Empty)
            ),
            Some((SpriteIndices::WallCorner, 0.0))
        );
    }

    #[test]
    fn test_left_to_top_corner_wall() {
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(
                Some(Tile::Wall),
                Some(Tile::Empty),
                Some(Tile::Wall)
            ),
            Some((SpriteIndices::WallCorner, PI))
        );
    }

    #[test]
    fn test_left_empty() {
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(None, None, Some(Tile::Empty)),
            Some((SpriteIndices::WallStraight, FRAC_PI_2))
        );
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(None, None, Some(Tile::Wall)),
            None
        );
    }

    #[test]
    fn test_top_empty() {
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(Some(Tile::Empty), None, None),
            Some((SpriteIndices::WallStraight, 0.0))
        );
        assert_eq!(
            WallPart::determine_sprite_for_wall_part(Some(Tile::Wall), None, None),
            None
        );
    }
}
