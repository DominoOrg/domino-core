use std::{collections::HashSet, fmt::Display};

use super::tile::Tile;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Puzzle {
  pub tiles: HashSet<Tile>,
  pub vec: Vec<Option<Tile>>
}

impl Puzzle {

    pub fn push(&mut self, tile: Option<Tile>) {
      if let Some(tile) = tile {
        self.tiles.insert(tile);
      }
      self.vec.push(tile);

    }
  
    pub fn insert(&mut self, position: usize, tile: Option<Tile>) {
        if let Some(tile) = tile {
          self.tiles.insert(tile);
        }
        self.vec.insert(position, tile);
    }

    pub fn remove(&mut self, position: usize) -> Option<Tile> {
        if let Some(Some(tile)) = self.vec.get(position) {
          self.tiles.remove(tile);
        }
        self.vec.remove(position)
    }

    pub fn at(&self, position: usize) -> Option<Tile> {
        self.vec.get(position).cloned().expect("Index out of bounds")
    }

    pub fn contains(&self, tile: Tile) -> Option<usize> {
        self.tiles.iter().position(|t| *t == tile)  
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl From<Vec<Option<(usize, usize)>>> for Puzzle {
    fn from(value: Vec<Option<(usize, usize)>>) -> Self {
        let mut puzzle = Puzzle::default();
        for tile in value.iter() {
          puzzle.push((*tile).map(|tile| tile.into()));
        }
        puzzle
    }
}

impl Into<Vec<Option<(usize, usize)>>> for Puzzle {
    fn into(self) -> Vec<Option<(usize, usize)>> {
        self.vec.into_iter().map(|tile| if let Some(tile) = tile {
          Some((tile.0, tile.1))  
        } else {
          None
        }).collect()
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      for tile in self.vec.iter() {
        if let Some(tile) = tile {
          write!(f, "({},{})", tile.0, tile.1)?;
        } else {
          write!(f, "(?,?)")?;
        }
      }
      std::fmt::Result::Ok(())
    }
}
pub struct FitPuzzleChecker;

impl FitPuzzleChecker {
    fn check_left(&self, puzzle: &Puzzle, position: usize, tile: Tile) -> bool {
      let left_tile = puzzle.at(position - 1);
      if let Some(left_tile) = left_tile {
        if left_tile.1 == tile.0 {
          true
        } else {
          false
        }
      } else {
        true
      } 
    }

    fn check_right(&self, puzzle: &Puzzle, position: usize, tile: Tile) -> bool {
      let right_tile = puzzle.at(position + 1);
      if let Some(right_tile) = right_tile {
        if right_tile.0 == tile.1 {
          true
        } else {
          false
        }
      } else {
        true
      }
    }

    pub fn check(&self, puzzle: &Puzzle, position: usize, tile: Tile) -> bool {
        match position {
            0 => {
              self.check_right(puzzle, position, tile)
            }, 
            position if position == puzzle.len() - 1 => {
              self.check_left(puzzle, position, tile)
            },
            position => {
              self.check_left(puzzle, position, tile) &&
              self.check_right(puzzle, position, tile)
            }
        }
    }

}