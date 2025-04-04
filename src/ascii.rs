use bevy::prelude::*;

#[derive(Resource)]
pub struct AsciiSheet {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

/// A struct representing the indices of the sprites on the sprite sheet.
#[derive(Debug, PartialEq, Eq)]
pub enum SpriteIndices {
    PacmanClosed = 0,
    PacmanOpen = 1,
    Blinky1 = 2,
    Blinky2 = 3,
    Inky1 = 4,
    Inky2 = 5,
    Pinky1 = 6,
    Pinky2 = 7,
    Clyde1 = 8,
    Clyde2 = 9,
    WallCorner = 10,
    WallStraight = 11,
    SmallCoin = 12,
    LargeCoin = 13,
    Empty = 14,
}

impl From<SpriteIndices> for usize {
    fn from(val: SpriteIndices) -> Self {
        val as usize
    }
}

/// Load the sprite sheet (aka, TextureAtlas) from the assets.
pub fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = assets.load("ascii.png");
    let atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32), // 32x32px per sprite
        4,
        4,
        Some(UVec2::splat(0)),
        Some(UVec2::default()),
    );

    let layout_handle = texture_atlases.add(atlas_layout);

    commands.insert_resource(AsciiSheet {
        image,
        layout: layout_handle,
    });
}
