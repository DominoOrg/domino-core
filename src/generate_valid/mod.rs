use crate::utils::find_eulerian_cycle;
use crate::{ComplexityClass, DominoError};
use crate::{Graph, Puzzle, Solution, Tile};
use rand::seq::{IteratorRandom, SliceRandom};
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
) -> Box<dyn Fn(ComplexityClass) -> Result<Puzzle, DominoError>> {
    Box::new(move |c: ComplexityClass| -> Result<Puzzle, DominoError> {
        let graph = Graph::regular(n);

        // The Eulerian path is represented as a sequence of nodes traversed.
        // The solution is built by grouping nodes in pairs and constructing tiles from each pair.
        let solution: Solution = find_eulerian_cycle(&graph)(true)
            .windows(2)
            .map(|arc| {
                Tile(
                    arc[0].clone().try_into().unwrap(),
                    arc[1].clone().try_into().unwrap(),
                )
            })
            .collect();

        // Removes recursively tiles and ensures the puzzle is still valid after removal
        // Repeats this until the classification class is not matched then returns the puzzle
        let puzzle = generate_puzzle(solution)(c);
        puzzle
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
) -> Box<dyn Fn(ComplexityClass) -> Result<Puzzle, DominoError>> {
    Box::new(move |c: ComplexityClass| {
        // Create an empty puzzle structure with all positions set to `None`.
        let starting_puzzle: Vec<Option<Tile>> = vec![None; solution.clone().len()];

        // Set the initial removed tiles to the totality of the tiles in the sequence.
        let mut removed_tiles = solution.clone();
        removed_tiles.shuffle(&mut thread_rng());

        // The first tile to be reinserted
        // We avoid double tiles for the first tile since their are always neutral
        // to the orientation of whole puzzle and so they produce an invalid puzzle
        let anchor: usize = (0..solution.len())
            .into_iter()
            .enumerate()
            .filter(|(_, pos)| solution[*pos].0 == solution[*pos].1)
            .map(|(i, _)| i)
            .choose(&mut thread_rng())
            .unwrap();

        // Attempt to reinsert removed tiles while checking puzzle validity.
        let puzzle: Result<Puzzle, DominoError> =
            reinsert_tile_and_check(starting_puzzle, solution.clone(), removed_tiles, c, anchor);
        puzzle
    })
}

#[cfg(test)]
mod tests {
    use crate::{classify::NUMBER_OF_CLASSES, ComplexityClass};

    use super::generate_valid_puzzle;

    #[test]
    fn it_works() {
        const RETRIALS: usize = 10;
        (3..=4).into_iter().for_each(|n| {
            (1..=NUMBER_OF_CLASSES)
                .into_iter()
                .rev()
                .map(|c| ComplexityClass::new(c).unwrap())
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
