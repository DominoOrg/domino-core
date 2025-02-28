
use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;

use crate::{classify_puzzle, Puzzle, Solution, Tile};
use crate::validate_puzzle;

pub(super) fn reinsert_tile_and_check(puzzle: Puzzle, solution: Solution, removed_tiles: Vec<Tile>, c: usize, random: bool) -> Puzzle {
  if validate_puzzle(&puzzle, &solution).is_ok() &&
  classify_puzzle(&puzzle) == c {
    return puzzle;
  }

  match (random, removed_tiles.len() == solution.len()) {
    (true, true) => {
      let tile = random_tile(solution.clone());
      let position = random_empty_position(&puzzle);
      let (puzzle, removed_tiles) = reinsert_tile(
        puzzle,
        removed_tiles,
        tile,
        position);
      return reinsert_tile_and_check(puzzle, solution, removed_tiles, c, random);
    },
    (true, false) => {
      let tile = random_tile(removed_tiles.clone());
      let position = random_empty_position_next_to(&puzzle);
      let (puzzle, removed_tiles) = reinsert_tile(
        puzzle,
        removed_tiles,
        tile,
        position);
      return reinsert_tile_and_check(puzzle, solution, removed_tiles, c, random);
    },
    (false, true) => {
      let tile: Tile = removed_tiles[0];
      let position: usize = 0;
      let (puzzle, removed_tiles) = reinsert_tile(
        puzzle,
        removed_tiles,
        tile,
        position);
      return reinsert_tile_and_check(puzzle, solution, removed_tiles, c, random);
    },
    (false, false) => {
      let tile: Tile = removed_tiles[0];
      let position: usize = puzzle.iter().enumerate()
        .filter(|(index, tile)| tile.is_some() &&
          puzzle[index + 1 % puzzle.len()].is_none())
        .next().unwrap().0 + 1;
      let (puzzle, removed_tiles) = reinsert_tile(
        puzzle,
        removed_tiles,
        tile,
        position);
      return reinsert_tile_and_check(puzzle, solution, removed_tiles, c, random);
    },
  }
}

fn reinsert_tile(puzzle: Puzzle, removed_tiles: Vec<Tile>, tile: Tile, position: usize) -> (Puzzle, Vec<Tile>) {
  let mut removed_tiles = removed_tiles.clone();
  let mut puzzle = puzzle.clone();
  puzzle[position] = Some(tile);
  removed_tiles.splice(position..position+1, vec![]);
  (puzzle, removed_tiles)
}


fn random_tile(solution: Solution) -> Tile {
  let mut seed = rand::thread_rng();
  solution.choose(&mut seed).unwrap().clone()
}

fn random_empty_position_next_to(puzzle: &Puzzle) -> usize {
  puzzle.iter().enumerate()
      .filter(|(index, tile)| tile.is_some() &&
        puzzle[index + 1 % puzzle.len()].is_none())
      .choose(&mut thread_rng()).unwrap().0 + 1
}

fn random_empty_position(puzzle: &Puzzle) -> usize {
  let mut seed = rand::thread_rng();
  puzzle.iter().enumerate()
    .filter(|(_index, tile)| tile.is_none())
    .choose(&mut seed).unwrap().0
}
