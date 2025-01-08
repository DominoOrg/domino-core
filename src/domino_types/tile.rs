use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Tile(pub usize, pub usize);

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        // Convert both tuples into a HashSet to disregard order
        let self_set = HashSet::from([self.0, self.1]);
        let other_set = HashSet::from([other.0, other.1]);
        self_set == other_set
    }
}

impl Eq for Tile {
    
}

impl TryFrom<(String, String)> for Tile {
    type Error = String;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        let (a, b) = value;
        let a = a.parse().map_err(|_| "Invalid tile".to_string())?;
        let b = b.parse().map_err(|_| "Invalid tile".to_string())?;

        Ok(Tile(a, b))
    }
}

impl From<(usize, usize)> for Tile {
    fn from(value: (usize, usize)) -> Self {
        Tile(value.0, value.1)
    }
}