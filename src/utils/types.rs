#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Tile(pub i32, pub i32);

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

impl From<(i32, i32)> for Tile {
    fn from(value: (i32, i32)) -> Self {
        Tile(value.0, value.1)
    }
}

impl Tile {
    pub fn flip(self) -> Self {
        Tile(self.1, self.0)
    }
}

pub type Solution = Vec<Tile>;

pub type Puzzle = Vec<Option<Tile>>;
