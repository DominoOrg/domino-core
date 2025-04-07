use std::{collections::vec_deque::{Iter}, ops::{Index, IndexMut}};
use std::vec::IntoIter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Tile(pub i32, pub i32);

impl std::fmt::Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{},{}]", self.0, self.1)
  }
}

impl std::str::FromStr for Tile {
  type Err = std::num::ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let v: Vec<i32> = s
      .trim_matches(|c| c == '[' || c == ']')
      .split(',')
      .map(|s| s.parse().unwrap())
      .collect();
    Ok(Tile(v[0], v[1]))
  }
}

impl Serialize for Tile {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&format!("[{},{}]", self.0, self.1))
  }
}

impl<'de> Deserialize<'de> for Tile {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let v: Vec<i32> = s
      .trim_matches(|c| c == '[' || c == ']')
      .split(',')
      .map(|s| s.parse().unwrap())
      .collect();
    Ok(Tile(v[0], v[1]))
  }
}

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Puzzle(pub Vec<Option<Tile>>);

impl Puzzle {

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn into_iter(self) -> std::vec::IntoIter<Option<Tile>> {
    self.0.into_iter()
  }
}

impl Index<usize> for Puzzle {
    type Output = Option<Tile>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Puzzle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Serialize for Puzzle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = vec![];
        self.0.iter().for_each(|t| {
          if let Some(t) = t {
            s.push(t.to_string());
          } else {
            s.push("null".to_string());
          }
         });
         let result = "[".to_string() + &s.join(",") + "]";
        serializer.serialize_str(&result)
    }
}

impl<'de> Deserialize<'de> for Puzzle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut v: Vec<Option<Tile>> = vec![];
        s
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .for_each(|tile| {
              if tile == "null" {
                v.push(None);
              } else {
                let parsed_tile: Tile = tile.parse().unwrap();
                v.push(Some(parsed_tile));
              }
            });

        Ok(Puzzle(v))
    }
}

impl From<Vec<Option<Tile>>> for Puzzle {
    fn from(value: Vec<Option<Tile>>) -> Self {
        Puzzle(value)
    }
}
