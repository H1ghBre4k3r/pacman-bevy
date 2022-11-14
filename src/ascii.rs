use bevy::prelude::*;

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

/// A struct representing the indices of the sprites on the sprite sheet.
#[derive(Debug, PartialEq, Eq)]
pub enum SpriteIndices {
    PacmanClosed = 0,
    PacmanOpen = 1,
    WallCorner = 10,
    WallStraight = 11,
    SmallCoin = 12,
    LargeCoin = 13,
}

impl Into<usize> for SpriteIndices {
    fn into(self) -> usize {
        self as usize
    }
}

/// Load the sprite sheet (aka, TextureAtlas) from the assets.
pub fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("ascii.png");
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(32.), // 32x32px per sprite
        4,
        4,
        Some(Vec2::splat(0.0)),
        Some(Vec2::default()),
    );

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}
