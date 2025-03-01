use std::vec;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reinsert::reinsert_tile_and_check;
use crate::{types::Graph, Puzzle, Solution, Tile};
use crate::graphs::find_eulerian_cycle;

mod reinsert;

pub fn generate_valid_puzzle(n: usize) ->  Box<dyn Fn(usize) -> Box<dyn Fn(bool) -> Puzzle>> {
  Box::new(move |c| {
    Box::new(move |random| {
      let graph = Graph::regular(n);
      // Since the eulerian path is represented as the sequence of the nodes traversed
      // The solution is built groupping 2 by 2 each node traversed and
      // building a tile from each couple
      let solution: Solution = find_eulerian_cycle(&graph)(random)
        .windows(2)
        .map(|arc| Tile(arc[0].clone().try_into().unwrap(), arc[1].clone().try_into().unwrap()))
        .collect();
      let puzzle = generate_puzzle(solution)(c)(random);
      puzzle
    })
  })
}

fn generate_puzzle(solution: Solution) -> Box<dyn Fn(usize) -> Box<dyn Fn(bool) -> Puzzle>> {
  Box::new(move |c: usize| {
    Box::new({
    let value = solution.clone();
    move |random: bool| -> Puzzle {
      let starting_puzzle: Vec<Option<Tile>> = vec![None; value.clone().len()];
      let removed_tiles: Vec<Tile> = if random {
        let mut removed =  value.clone();
        removed.shuffle(&mut thread_rng());
        removed
      } else { value.clone() };
      let puzzle: Puzzle = reinsert_tile_and_check(starting_puzzle, value.clone(), removed_tiles, c, random);
      puzzle
    }
    })
  })
}

#[cfg(test)]
mod tests {

  #[test]
  fn test_generate_valid_puzzle() {
    (3..=6).into_iter().for_each(|n| {
      println!("n: {n}");
      (1..=3).into_iter().for_each(|c| {
        println!("c: {c}");
        let puzzle = super::generate_valid_puzzle(n)(c)(false);
        println!("puzzle: {puzzle:?}");
        let expected_len = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
        let max_hole = expected_len as f32 - (n as f32 / 2.0).floor();
        let log_factor: f64 = match c {
          1 => 3.0 / 7.0,
          2 => 5.0 / 7.0,
          3 => 1.0,
          _ => 0.0
        };
        let expected_hole_size = (max_hole as f64 * log_factor.sqrt()).ceil() as usize;
        assert_eq!(puzzle.len(), expected_len);
        assert_eq!(puzzle.iter().filter(|tile| tile.is_none()).count(), expected_hole_size);
      })
    });

    (3..=6).into_iter().for_each(|n| {
      println!("n: {n}");
      (1..=3).into_iter().for_each(|c| {
        println!("c: {c}");
        let puzzle = super::generate_valid_puzzle(n)(c)(true);
        println!("puzzle: {puzzle:?}");
        let expected_len = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
        let max_hole = expected_len as f32 - (n as f32 / 2.0).floor();
        let log_factor: f64 = match c {
          1 => 3.0 / 7.0,
          2 => 5.0 / 7.0,
          3 => 1.0,
          _ => 0.0
        };
        let expected_hole_size = (max_hole as f64 * log_factor.sqrt()).ceil() as usize;
        assert_eq!(puzzle.len(), expected_len);
        assert_eq!(puzzle.iter().filter(|tile| tile.is_none()).count(), expected_hole_size);
      })
    });
  }
}
