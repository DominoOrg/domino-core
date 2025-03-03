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

      let len = solution.len();
      // When building with random set to false the eulerian path starts with a double tile
      // So to have a puzzle that has c == 3 for n == 3 the only way to make it in a NON RANDOM
      // way is to make it start with a single tile, so it's done
      // rotating cyclically the sequence by one to the left
      let rotated_left_solution = solution
        .into_iter()
        .cycle()
        .skip(1)
        .take(len)
        .collect();
      println!("Solution: {rotated_left_solution:?}");
      let puzzle = generate_puzzle(rotated_left_solution)(c)(random);
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
  use crate::graphs::{invert_logaritmic_scale, relative_complexity_from_class};

  fn one_hole_relative_complexity_to_absolute(max_hole: usize, c: usize) -> f32 {
    println!("max_hole: {max_hole}, c: {c}");
    let relative_complexity = relative_complexity_from_class(c);
    let normalized_complexity = invert_logaritmic_scale(relative_complexity, max_hole);

    println!("relative_complexity: {normalized_complexity}");
    let absolute_complexity = normalized_complexity * max_hole as f32;
    absolute_complexity
  }

  #[test]
  fn test_generate_valid_puzzle() {
    (3..=6).into_iter().for_each(|n| {
      println!("n: {n}");
      (1..=3).into_iter().for_each(|c| {
        println!("c: {c}");
        let puzzle = super::generate_valid_puzzle(n)(c)(false);
        println!("puzzle: {puzzle:?}");
        let puzzle_len = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
        let max_hole = puzzle_len as f32 - (n as f32 / 2.0).floor();
        let hole_size = puzzle.iter().filter(|tile| tile.is_none()).count();
        let expected_hole_size = one_hole_relative_complexity_to_absolute(max_hole as usize, c) as usize;
        assert_eq!(puzzle.len(), puzzle_len);
        let tolerance = ((n as f32 / 2.0).floor() * 2.0) as usize;
        assert!(hole_size >= expected_hole_size - tolerance && hole_size <= expected_hole_size + tolerance);
        println!("-------------------------");
      })
    });
  }
}
