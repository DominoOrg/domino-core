use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

use super::tile::Tile;

pub struct Tileset(HashSet<Tile>);

impl Tileset {
  pub fn new(n: usize) -> Self {
    let mut tileset = HashSet::new();
    for i in 0..n+1 {
      for j in 0..n+1 {
        tileset.insert(Tile(i, j));
      }
    }
    Tileset(tileset)
  }

  pub fn get(&self, position: usize) -> Option<Tile> {
    let ordered_tileset: Vec<Tile> = self.0.clone().into_iter().sorted_by(|t1, t2| {
      if t1.0.cmp(&t2.0) != Ordering::Equal {
        t1.0.cmp(&t2.0)
      } else {
        t1.1.cmp(&t2.1)
      }
    })
    .map(|tile| tile.clone())
    .collect();
    ordered_tileset.get(position).copied()
  }
}

impl Iterator for Tileset {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next().cloned()
    }
}