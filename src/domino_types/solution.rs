use std::collections::HashMap;

use super::{puzzle::Puzzle, tile::Tile, tileset::Tileset};

pub type Solution = Puzzle;
struct PlacedTile {
  position: usize,
  tile: Tile
}

struct Variable {
  key: PlacedTile,
  value: bool
}

struct VariableParser {
  n: usize
}

impl VariableParser {
  fn tileset_len(&self) -> usize {
    if self.n % 2 == 0 {
      (self.n + 1) * (self.n + 2)
    } else {
      (self.n + 1).pow(2)
    }
  }

  fn sequence_len(&self) -> usize {
    self.tileset_len() / 2
  }

  fn tileset_digits(&self) -> usize {
    ((self.tileset_len() - 1) as f64).log10().ceil() as usize
  } 

  fn sequence_digits(&self) -> usize {
    ((self.sequence_len() - 1) as f64).log10().ceil() as usize
  }

  fn parse_variable(&self, variable: (String, f64)) -> Variable {
    let key = variable.0;
    let value = variable.1;
    let tileset = Tileset::new(self.n);
    let tile_index = key[1..self.tileset_digits()].parse::<usize>().unwrap();
    let tile = tileset.get(tile_index).unwrap();
    let position = key[self.tileset_digits()..self.tileset_digits() + self.sequence_digits()].parse::<usize>().unwrap();
    let placed_tile = PlacedTile{
      tile,
      position,
    };
    let variable = Variable{
      key: placed_tile,
      value: value == 1.0
    };
    variable
  }

  pub fn parse(self, variables_map: HashMap<String, f64>) -> Vec<Variable> {
      let mut variables: Vec<Variable> = Vec::new();

      for variable in variables_map {
        let parsed_variable = self.parse_variable(variable);
        variables.push(parsed_variable);
      }
      variables
  }
}

pub struct SolutionBuilder {}

impl SolutionBuilder {

  pub fn build(self, puzzle: &Puzzle, variables_map: HashMap<String, f64>, n: usize) -> Solution {
    let mut solution: Solution = puzzle.clone();
    let variable_parser = VariableParser{
      n
    };
    let variables = variable_parser.parse(variables_map);
    for (_position, tile) in variables.into_iter().filter(|variable| variable.value).map(|variable| (variable.key.position, variable.key.tile)) {
      solution.push(Some(tile.clone()));
    }
    solution
  }
}