#[derive(Debug)]
pub enum DominoError {
    InvalidLength,
    UnsolvablePuzzle,
    NotValidPuzzle,
    Timeout
}

impl std::fmt::Display for DominoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "The puzzle length is not correct"),
            Self::UnsolvablePuzzle => write!(f, "The puzzle has no solutions"),
            Self::NotValidPuzzle => write!(f, "The puzzle is not valid/unique, it has multiple solutions"),
            Self::Timeout => write!(f, "The puzzle took too long to solve")
        }
    }
}

impl std::error::Error for DominoError {}

pub type Result<T, E = DominoError> = std::result::Result<T, E>;