//! This module provides functionality for generating valid puzzle instances.
//!
//! It includes a function to generate a puzzle with a valid Eulerian cycle and remove a specified number of tiles.

use std::time::{Duration, Instant};

use crate::{classify_puzzle, utils::find_eulerian_cycle, validate_puzzle, ComplexityClass, Graph, Puzzle, Solution, Tile};
use rand::Rng;

/// Generates a puzzle with a valid Eulerian cycle and removes a specified number of tiles.
///
/// This function constructs a `Graph` representation of the puzzle, finds an Eulerian cycle,
/// and converts the cycle into a `Solution`. Then, it removes a specified number of tiles
/// either sequentially or randomly, based on the `random` flag.
///
/// # Arguments
///
/// * `n` - The size of the puzzle.
/// * `minimum_removals` - The number of tiles to remove from the solution.
/// * `random` - If `true`, removes tiles at random; otherwise, removes them sequentially.
///
/// # Returns
///
/// A `Puzzle` instance with `Some(Tile)` values for placed tiles and `None` for removed tiles.
pub fn generate_puzzle(n: usize, c: usize) -> Puzzle {
    let graph = Graph::regular(n);
    let eulerian_cycle = find_eulerian_cycle(&graph,true);
    let mut solution: Solution = create_solution_from_cycle(&eulerian_cycle);
    let mut puzzle= solution.clone().into_iter().map(Some).collect::<Vec<Option<Tile>>>();
    let mut rng = rand::thread_rng();

    // Complexity checks
    let mut expected_complexity = ComplexityClass::new(c).ok();
    let mut actual_complexity: Option<ComplexityClass> = None;
    let mut is_not_complex_enough = actual_complexity != expected_complexity;
    let mut is_too_complex = false;

    // Timeout
    let mut now = Instant::now();
    let timeout = Duration::from_millis(2000);

    // Validity checks
    let is_not_valid = validate_puzzle(&puzzle.clone().into(), &solution).is_err();

    // Removal history
    let mut index = rng.gen_range(0..puzzle.len());
    let mut removal_history: Vec<(Option<Tile>, usize)> = vec![(puzzle[index].clone(), index)];
    puzzle[index] = None;


    // Remove tiles
    while is_not_valid || is_not_complex_enough {

      while is_too_complex {
        // println!("is_too_complex");
        reinsert_tile(&mut puzzle, &mut removal_history);
        update_complexity(&mut actual_complexity, &mut expected_complexity, &puzzle, &mut is_not_complex_enough, &mut is_too_complex);
      }

      let removed_tile: Option<Tile>;
      let removed_position: Option<usize>;

      // Remove a tile at a true position
      (puzzle, removed_tile, removed_position) = remove_non_empty_tile(puzzle);
      removal_history.push((removed_tile, removed_position.unwrap()));

      // Update complexity checks
      update_complexity(&mut actual_complexity, &mut expected_complexity, &puzzle, &mut is_not_complex_enough, &mut is_too_complex);

      // Update validity checks
      let is_not_valid = validate_puzzle(&puzzle.clone().into(), &solution).is_err();

      // The puzzle becomes invalid rollback
      if is_not_valid {
        reinsert_tile(&mut puzzle, &mut removal_history);
        update_complexity(&mut actual_complexity, &mut expected_complexity, &puzzle, &mut is_not_complex_enough, &mut is_too_complex);
      }

      // If the time spent trying to reach the desired complexity is greater than the timeout then restart with another initial solution
      if now.elapsed().as_millis() > timeout.as_millis() || actual_complexity.is_none(){
        // println!("timeout, puzzle is: {puzzle:?}");
        solution = create_solution_from_cycle(&eulerian_cycle);
        puzzle = solution.clone().into_iter().map(Some).collect::<Vec<Option<Tile>>>().into();
        index = rng.gen_range(0..puzzle.len());
        removal_history = vec![(puzzle[index].clone(), index)];
        puzzle[index] = None;
        update_complexity(&mut actual_complexity, &mut expected_complexity, &puzzle, &mut is_not_complex_enough, &mut is_too_complex);
        now = Instant::now();
      }

    }

    puzzle.into()
}

fn update_complexity(actual_complexity: &mut Option<ComplexityClass>, expected_complexity: &mut Option<ComplexityClass>, puzzle: &Vec<Option<Tile>>, is_not_complex_enough: &mut bool, is_too_complex: &mut bool) {
  let result = classify_puzzle(&puzzle.clone().into());
  *actual_complexity = result.ok();
  *is_not_complex_enough = actual_complexity != expected_complexity;
  *is_too_complex = actual_complexity.is_some() && actual_complexity.unwrap() > expected_complexity.unwrap();

}

fn reinsert_tile(puzzle: &mut Vec<Option<Tile>>, history: &mut Vec<(Option<Tile>, usize)>) {
  // println!("Puzzle is not valid reinserting tile");
  let (removed_tile, removed_position) = history.pop().unwrap();
  puzzle[removed_position] = removed_tile;
  // println!("puzzle: {:?}, removed_tile: {:?}, removed_position: {:?}", puzzle, removed_tile, removed_position);

}

fn remove_non_empty_tile(mut puzzle: Vec<Option<Tile>>) -> (Vec<Option<Tile>>, Option<Tile>, Option<usize>) {
  let mut rng = rand::thread_rng();
  let mut index = rng.gen_range(0..puzzle.len());

  for _ in 0..10 {
    if puzzle[index].is_some() {
      index = rng.gen_range(0..puzzle.len());
    }
  }

  while puzzle[index].is_none() {
      index = (index + 1) % puzzle.len();
  }
  let removed_tile = puzzle[index].clone();
  puzzle[index] = None;
  (puzzle.clone(), removed_tile, Some(index))
}

fn create_solution_from_cycle(eulerian_cycle: &Vec<crate::Node>) -> Solution {
eulerian_cycle
    .windows(2)
    .map(|arc| {
        Tile(
            arc[0].clone().try_into().unwrap(),
            arc[1].clone().try_into().unwrap(),
        )
    })
    .collect()
}
