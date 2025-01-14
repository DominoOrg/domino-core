#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Tile(pub i32, pub i32);

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 ||
        self.0 == other.1 && self.1 == other.0
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
        Err(DominoError::InvalidPuzzle("Invalid puzzle".to_string()))
    }
}

#[derive(Debug)]
pub enum DominoError {
    InvalidPuzzle(String),
}

impl std::fmt::Display for DominoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidPuzzle(msg) => write!(f, "Invalid puzzle: {}", msg),
        }
    }
}

impl std::error::Error for DominoError {}

pub type Result<T, E = DominoError> = std::result::Result<T, E>;