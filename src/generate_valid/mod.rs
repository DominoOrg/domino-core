use crate::utils::find_eulerian_cycle;
use crate::{ComplexityClass, DominoError, Node};
use crate::{Graph, Puzzle, Solution, Tile};
use rand::Rng;
use std::vec;

/// Represents the puzzle parameters and solution
#[derive(Clone)]
struct PuzzleData {
    n: usize,
    c: ComplexityClass,
    graph: Graph,
    solution: Solution,
}

/// Orchestrates the puzzle generation flow using currying (one input function)
fn generate_valid_puzzle(n: usize) -> Box<dyn Fn(usize) -> Result<Puzzle, DominoError>> {
    Box::new(move |c| ComplexityClass::new(c)
    .and_then(|valid_c|
      validate_input(n)(valid_c)
        .map(generate_solution)
        .map(generate_puzzle)
    ))
}

/// Validates input parameters using currying (one input function)
fn validate_input(n: usize) -> Box<dyn Fn(ComplexityClass)->Result<PuzzleData, DominoError>> {
  Box::new(move |c| {
    if n < 1 {
      Err(DominoError::InvalidLength)
    } else {
        Ok(PuzzleData {
            n,
            c,
            graph: Graph::regular(n),
            solution: vec![], // Empty solution initially
        })
    }
  })
}

/// Reinserts Hamiltonian paths and returns updated PuzzleData
fn generate_puzzle(puzzle: PuzzleData) -> Puzzle {
  let mut rng = rand::thread_rng();
  let mut hamiltonian_cycles = compute_hamiltonian_cycles(puzzle.n);
  let mut modified_solution = puzzle.solution.clone();

  for _ in 0..(puzzle.n - 1) / 2 {
      if let Some(cycle) = hamiltonian_cycles.pop() {
          let index = rng.gen_range(0..modified_solution.len());
          // modified_solution.splice(index..index, cycle);
      }
  }

  PuzzleData { solution: modified_solution, ..puzzle };
  vec![]
}

fn compute_hamiltonian_cycles(n: usize) -> Vec<Vec<Node>> {
    todo!()
}

/// Generates a solution using Hierholzer's algorithm
fn generate_solution(puzzle: PuzzleData) -> PuzzleData {
  let cycle = find_eulerian_cycle(&puzzle.graph)(true);
  let sequence = cycle.windows(2)
  .map(|arc| {
      Tile(
          arc[0].clone().try_into().unwrap(),
          arc[1].clone().try_into().unwrap(),
      )
  })
  .collect();

  PuzzleData { solution: sequence, ..puzzle }
}

mod tests {

    use crate::NUMBER_OF_CLASSES;

    use super::generate_valid_puzzle;

    #[test]
    fn it_works() {
        const RETRIALS: usize = 10;
        (3..=4).into_iter().for_each(|n| {
            (1..=NUMBER_OF_CLASSES)
                .into_iter()
                .rev()
                .for_each(|c| {
                    (0..=RETRIALS).into_iter().for_each(|_| {
                        println!("Generating puzzle for n = {n} and c = {c}");
                        let puzzle = generate_valid_puzzle(n)(c);
                        assert_eq!(puzzle.is_ok(), true, "puzzle should be valid");
                        println!("*********SUCCESS*********\n\n---------------\n\n");
                    });
                });
        });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::graphs::ComplexityClass;
//     use crate::Puzzle;

//     /// Helper function to check if a puzzle is valid.
//     fn is_valid_puzzle(puzzle: &Puzzle) -> bool {
//         // Ensure the puzzle contains at least one tile.
//         !puzzle.is_empty() && puzzle.iter().any(|tile| tile.is_some())
//     }

//     /// Test puzzle generation for different values of `n` (n=3).
//     #[test]
//     fn test_generate_valid_puzzle_n3() {
//         let generator = generate_valid_puzzle(3);
//         let result = generator(ComplexityClass(1));
//         assert!(result.is_ok(), "Puzzle generation should succeed for n=3");
//         let puzzle = result.unwrap();
//         assert!(
//             is_valid_puzzle(&puzzle),
//             "Generated puzzle should be valid for n=3"
//         );
//     }

//     /// Test puzzle generation for different values of `n` (n=5).
//     #[test]
//     fn test_generate_valid_puzzle_n5() {
//         let generator = generate_valid_puzzle(5);
//         let result = generator(ComplexityClass(3));
//         assert!(result.is_ok(), "Puzzle generation should succeed for n=5");
//         let puzzle = result.unwrap();
//         assert!(
//             is_valid_puzzle(&puzzle),
//             "Generated puzzle should be valid for n=5"
//         );
//     }

//     /// Test puzzle generation for a given complexity class (ComplexityClass(1)).
//     #[test]
//     fn test_puzzle_complexity_class1() {
//         let generator = generate_valid_puzzle(4);
//         let result = generator(ComplexityClass(1));
//         assert!(
//             result.is_ok(),
//             "Puzzle generation should succeed for ComplexityClass(1)"
//         );
//         let puzzle = result.unwrap();
//         assert!(
//             is_valid_puzzle(&puzzle),
//             "Generated puzzle should be valid for ComplexityClass(1)"
//         );
//     }

//     /// Test puzzle generation for a given complexity class (ComplexityClass(3)).
//     #[test]
//     fn test_generate_puzzle_complexity_class3() {
//         let generator = generate_valid_puzzle(4);
//         let result = generator(ComplexityClass(3));
//         assert!(
//             result.is_ok(),
//             "Puzzle generation should succeed for ComplexityClass(3)"
//         );
//         let puzzle = result.unwrap();
//         assert!(
//             is_valid_puzzle(&puzzle),
//             "Generated puzzle should be valid for ComplexityClass(3)"
//         );
//     }
// }
