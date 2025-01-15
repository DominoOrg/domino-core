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