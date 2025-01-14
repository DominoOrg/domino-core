use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Tile(pub i32, pub i32);

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 ||
        self.0 == other.1 && self.1 == other.0
    }
}

impl From<(i32, i32)> for Tile {
    fn from(value: (i32, i32)) -> Self {
        Tile(value.0, value.1)
    }
}

pub type Solution = Vec<Tile>;

pub type Puzzle = Vec<Option<Tile>>;

pub(crate) struct SequenceScraper;

impl SequenceScraper {
    pub fn get_n(puzzle: &Puzzle) -> Result<i32, DominoError> {
        let mut tmp = (-3.0 + (1.0 + 8.0 * puzzle.len() as f32).sqrt()) / 2.0;
        if tmp - tmp.floor() == 0.0 {
            return Ok(tmp as i32);
        }
        tmp = -1.0 + (2.0 * puzzle.len() as f32).sqrt();
        if tmp - tmp.floor() == 0.0 {
            return Ok(tmp as i32);
        }
        tmp = (-1.0 + (1.0 + 4.0 * puzzle.len() as f32).sqrt()) / 2.0;
        if tmp - tmp.floor() == 0.0 {
            return Ok(tmp as i32);
        }
        tmp = (puzzle.len() as f32).sqrt();
        if tmp - tmp.floor() == 0.0 {
            return Ok(tmp as i32);
        }
        Err(DominoError::InvalidPuzzle("Puzzle lenght is invalid".to_string()))
    }

    pub fn get_missing_tiles(puzzle: &Puzzle) -> Result<HashSet<Tile>, DominoError> {
        let n = SequenceScraper::get_n(puzzle)?;
        let tileset: HashSet<Tile> = (0..(n + 1)).map(|i| {
            ((i + 1)..(n + 1)).map(|j| Tile(i, j)).collect::<Vec<Tile>>()
        })
        .flatten()
        .collect();
        let used_tiles: HashSet<Tile> = puzzle.iter().filter_map(|tile| if tile.is_some() { Some(tile.unwrap()) } else { None } ).collect();
        let missing_tiles: HashSet<Tile> = tileset.difference(&used_tiles).cloned().collect();
        Ok(missing_tiles)
    }

    pub fn get_empty_positions(puzzle: &Puzzle) -> Result<Vec<usize>, DominoError> {
        let empty_positions: Vec<usize> = puzzle
        .iter()
        .enumerate()
        .filter_map(|(index, tile)| if tile.is_none() { Some(index) } else { None })
        .collect();
        Ok(empty_positions)
    }
}

#[derive(Debug)]
pub enum DominoError {
    InvalidPuzzle(String),
    UnsolvableGraph(String)
}

impl std::fmt::Display for DominoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidPuzzle(msg) => write!(f, "Invalid puzzle: {}", msg),
            Self::UnsolvableGraph(msg) => write!(f, "The puzzle is represented by an unsolvable graph: {}", msg),
        }
    }
}

impl std::error::Error for DominoError {}

pub type Result<T, E = DominoError> = std::result::Result<T, E>;