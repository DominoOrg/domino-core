use crate::graphs::{find_eulerian_cycle, ComplexityClass};
use crate::{types::Graph, Puzzle, Solution, Tile};
use rand::seq::SliceRandom;
use rand::thread_rng;
use reinsert::reinsert_tile_and_check;
use std::vec;

mod reinsert;

/// Generates a valid puzzle of size `n` that can later be customized by complexity class and randomness.
///
/// This function adopts a functional style by returning a series of closures, each progressively refining the puzzle generation process.
/// The first closure takes a `ComplexityClass`, and the second closure takes a boolean (`random`).
/// These parameters are intended to be used within the function, guiding the puzzle generation based on the chosen complexity and randomness.
///
/// # Arguments
/// * `n` - The number of tiles in the puzzle.
///
/// # Returns
/// A function that, given a `ComplexityClass`, returns another function that generates a `Puzzle`.
pub fn generate_valid_puzzle(
  n: usize,
) -> Box<dyn Fn(ComplexityClass) -> Box<dyn Fn(bool) -> Puzzle>> {
  Box::new(move |c: ComplexityClass| {
      Box::new(move |random| {
          let graph = Graph::regular(n);

          // The Eulerian path is represented as a sequence of nodes traversed.
          // The solution is built by grouping nodes in pairs and constructing tiles from each pair.
          let solution: Solution = find_eulerian_cycle(&graph)(random)
              .windows(2)
              .map(|arc| {
                  Tile(
                      arc[0].clone().try_into().unwrap(),
                      arc[1].clone().try_into().unwrap(),
                  )
              })
              .collect();

          let len = solution.len();

          // If `random` is false, adjust the solution to ensure `c == 3` for `n == 3`.
          // This is done by cyclically rotating the sequence to start with a single tile.
          let rotated_left_solution = solution.into_iter().cycle().skip(1).take(len).collect();
          let puzzle = generate_puzzle(rotated_left_solution)(c)(random);
          puzzle
      })
  })
}

/// Generates a puzzle based on a given solution and complexity class.
///
/// This function returns a closure that takes a `ComplexityClass` and returns another closure,
/// which, when called with a boolean (`random`), generates a puzzle.
///
/// # Arguments
/// * `solution` - The reference solution from which to generate the puzzle.
///
/// # Returns
/// A function that, given a `ComplexityClass`, returns another function that generates a `Puzzle`.
fn generate_puzzle(
  solution: Solution,
) -> Box<dyn Fn(ComplexityClass) -> Box<dyn Fn(bool) -> Puzzle>> {
  Box::new(move |c: ComplexityClass| {
      Box::new({
          let value = solution.clone();
          move |random: bool| -> Puzzle {
              // Create an empty puzzle structure with all positions set to `None`.
              let starting_puzzle: Vec<Option<Tile>> = vec![None; value.clone().len()];

              // Determine which tiles should be removed initially.
              let removed_tiles: Vec<Tile> = if random {
                  let mut removed = value.clone();
                  removed.shuffle(&mut thread_rng());
                  removed
              } else {
                  value.clone()
              };

              // Attempt to reinsert removed tiles while checking puzzle validity.
              let puzzle: Puzzle = reinsert_tile_and_check(
                  starting_puzzle,
                  value.clone(),
                  removed_tiles,
                  c,
                  random,
              );
              puzzle
          }
      })
  })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::ComplexityClass;
    use crate::Puzzle;

    /// Helper function to check if a puzzle is valid.
    fn is_valid_puzzle(puzzle: &Puzzle) -> bool {
        // Ensure the puzzle contains at least one tile
        !puzzle.is_empty() && puzzle.iter().any(|tile| tile.is_some())
    }

    /// Test puzzle generation for different values of `n`.
    #[test]
    fn test_generate_valid_puzzle_n3() {
        let generator = generate_valid_puzzle(3);
        let generate_puzzle = generator(ComplexityClass(1));
        let puzzle = generate_puzzle(false);
        assert!(is_valid_puzzle(&puzzle), "Generated puzzle should be valid for n=3");
    }

    #[test]
    fn test_generate_valid_puzzle_n4() {
        let generator = generate_valid_puzzle(4);
        let generate_puzzle = generator(ComplexityClass(2));
        let puzzle = generate_puzzle(true);
        assert!(is_valid_puzzle(&puzzle), "Generated puzzle should be valid for n=4");
    }

    #[test]
    fn test_generate_valid_puzzle_n5() {
        let generator = generate_valid_puzzle(5);
        let generate_puzzle = generator(ComplexityClass(3));
        let puzzle = generate_puzzle(false);
        assert!(is_valid_puzzle(&puzzle), "Generated puzzle should be valid for n=5");
    }

    /// Test puzzle generation for different complexity classes.
    #[test]
    fn test_generate_puzzle_complexity_class1() {
        let generator = generate_valid_puzzle(4);
        let generate_puzzle = generator(ComplexityClass(1));
        let puzzle = generate_puzzle(false);
        assert!(is_valid_puzzle(&puzzle), "Generated puzzle should be valid for ComplexityClass(1)");
    }

    #[test]
    fn test_generate_puzzle_complexity_class2() {
        let generator = generate_valid_puzzle(4);
        let generate_puzzle = generator(ComplexityClass(2));
        let puzzle = generate_puzzle(true);
        assert!(is_valid_puzzle(&puzzle), "Generated puzzle should be valid for ComplexityClass(2)");
    }

    #[test]
    fn test_generate_puzzle_complexity_class3() {
        let generator = generate_valid_puzzle(4);
        let generate_puzzle = generator(ComplexityClass(3));
        let puzzle = generate_puzzle(false);
        assert!(is_valid_puzzle(&puzzle), "Generated puzzle should be valid for ComplexityClass(3)");
    }

    /// Test random vs non-random puzzle generation.
    #[test]
    fn test_generate_puzzle_random_vs_nonrandom() {
        let generator = generate_valid_puzzle(3);
        let generate_nonrandom = generator(ComplexityClass(1));
        let generate_random = generator(ComplexityClass(1));
        let puzzle_nonrandom = generate_nonrandom(false);
        let puzzle_random = generate_random(true);
        assert!(is_valid_puzzle(&puzzle_nonrandom), "Non-random puzzle should be valid");
        assert!(is_valid_puzzle(&puzzle_random), "Random puzzle should be valid");
        assert_ne!(puzzle_nonrandom, puzzle_random, "Random puzzle should differ from non-random");
    }
}
