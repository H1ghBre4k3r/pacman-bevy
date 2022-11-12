use std::fs;

use super::Tile;

/// A map representing all tiles present on the map.
#[derive(Clone)]
pub struct TileMap {
    _tiles: Vec<Vec<Tile>>,
}

impl From<Vec<Vec<Tile>>> for TileMap {
    fn from(tiles: Vec<Vec<Tile>>) -> Self {
        Self { _tiles: tiles }
    }
}

impl TileMap {
    pub fn from_string(data: &str) -> Self {
        let content = fs::read_to_string(data).expect("Should have been able to read the map");

        let lines = content.lines().rev();

        let mut columns = vec![];

        for line in lines {
            let letters = line.chars();
            for (x, letter) in letters.enumerate() {
                if columns.len() <= x {
                    columns.push(vec![]);
                }
                columns[x].push(Tile::from_char(letter));
            }
        }

        // TODO: evaluate (or think about) whether this is needed
        assert!(
            columns.len() > 0,
            "Expected map to have at least one column"
        );

        let rows = columns[0].len();
        for (x, column) in columns.iter().enumerate() {
            assert!(
                column.len() == rows,
                "Expected all columns to have the same amount of rows! (failed at column {}; {} instead of {})",
                x, column.len(), rows
            );
        }

        TileMap::from(columns)
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self._tiles
    }

    /// Get the tile at the specified position. If there is no tile or the indices are negative,
    /// this function returns None.
    pub fn at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        match self._tiles.get(x as usize) {
            Some(column) => {
                return column.get(y as usize).map(|tile| tile.clone());
            }
            None => None,
        }
    }
}
