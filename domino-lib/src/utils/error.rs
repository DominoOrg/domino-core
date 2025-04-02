#[derive(Debug, PartialEq)]
pub enum DominoError {
    InvalidLength,
    UnsolvablePuzzle,
    NotValidPuzzle,
    Timeout,
    ModelGenerationError(String),
    ModelError(String),
    GenerationError(String),
    InvalidClass(String),
    EmptyPuzzle,
}

impl std::fmt::Display for DominoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "The puzzle length is not correct"),
            Self::UnsolvablePuzzle => write!(f, "The puzzle has no solutions"),
            Self::NotValidPuzzle => write!(
                f,
                "The puzzle is not valid/unique, it has multiple solutions"
            ),
            Self::Timeout => write!(f, "The puzzle took too long to solve"),
            Self::ModelGenerationError(message) => write!(f, "{}", message),
            Self::ModelError(message) => write!(f, "{}", message),
            Self::GenerationError(message) => write!(
                f,
                "The puzzle could not be generated with the requires parameters: {}",
                message
            ),
            Self::InvalidClass(message) => write!(f, "{}", message),
            Self::EmptyPuzzle => write!(f, "The puzzle is empty"),
        }
    }
}

impl std::error::Error for DominoError {}
