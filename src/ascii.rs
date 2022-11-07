use bevy::prelude::*;

pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub enum SpriteIdices {
    PacmanClosed = 0,
    PacmanOpen = 1,
    WallCorner = 8,
    WallStraight = 9,
}

impl Into<usize> for SpriteIdices {
    fn into(self) -> usize {
        self as usize
    }
}

pub fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(32.),
        3,
        4,
        Vec2::splat(1.),
        Vec2::default(),
    );

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}
