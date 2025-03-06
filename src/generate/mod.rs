//! This module provides functionality for generating valid puzzle instances.
//!
//! It includes a function to generate a puzzle with a valid Eulerian cycle and remove a specified number of tiles.

use rand::Rng;
use crate::{utils::find_eulerian_cycle, Graph, Puzzle, Solution, Tile};

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
///
/// # Example
///
/// ```rust
/// let puzzle = generate_puzzle(3, 5, true);
/// ```
pub fn generate_puzzle(n: usize, minimum_removals: usize, random: bool) -> Puzzle {
    let graph = Graph::regular(n);
    let eulerian_cycle = find_eulerian_cycle(&graph)(random);

    // Convert Eulerian cycle to a solution of tiles
    let solution: Solution = eulerian_cycle
        .windows(2)
        .map(|arc| {
            Tile(
                arc[0].clone().try_into().unwrap(),
                arc[1].clone().try_into().unwrap(),
            )
        })
        .collect();

    // Convert solution into a puzzle with all tiles initially placed
    let mut puzzle: Puzzle = solution.into_iter().map(Some).collect();

    // Remove the specified number of tiles
    if puzzle.len() > minimum_removals {
        if random {
            let mut rng = rand::thread_rng();
            for _ in 0..minimum_removals {
                let mut index = rng.gen_range(0..puzzle.len());
                while puzzle[index].is_none() {
                    index = rng.gen_range(0..puzzle.len());
                }
                puzzle[index] = None;
            }
        } else {
            for index in 0..minimum_removals {
                puzzle[index] = None;
            }
        }
    }

    puzzle
}
