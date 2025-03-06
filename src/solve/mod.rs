//! This module provides a recursive backtracking algorithm for solving a Domino puzzle.
//!
//! It includes functions to determine missing tiles, check valid placements, and recursively
//! search for a valid solution.

use std::collections::HashSet;
use std::time::Instant;

use crate::{utils::get_n, DominoError, Puzzle, Solution, Tile};

/// Attempts to solve a given puzzle using a recursive backtracking approach.
///
/// This function clones the puzzle, determines the missing tiles, and then attempts to solve
/// the puzzle by filling in the missing tiles while ensuring valid adjacency constraints.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` structure representing the current puzzle state.
///
/// # Returns
///
/// * `Ok(Solution)` - If a valid solution is found.
/// * `Err(DominoError::UnsolvablePuzzle)` - If no solution exists.
/// * `Err(DominoError::InvalidPuzzle)` - If the puzzle input is invalid.
///
/// # Example
///
/// ```rust
/// let puzzle = Puzzle::new();
/// let solution = solve_puzzle(&puzzle);
/// ```
pub fn solve_puzzle(puzzle: &Puzzle) -> Result<Solution, DominoError> {
    let mut new_puzzle = puzzle.clone();
    let missing_tiles = get_missing_tiles(puzzle)?;
    let start_instant = Instant::now();

    if solve_puzzle_r(&mut new_puzzle, &missing_tiles, 0, &start_instant) {
        let solution = new_puzzle.into_iter().map(|tile| tile.unwrap()).collect();
        return Ok(solution);
    } else {
        return Err(DominoError::UnsolvablePuzzle);
    }
}

/// Recursive backtracking function to solve the puzzle.
///
/// This function iterates over the missing tiles, attempting to place each tile in the puzzle.
/// It ensures that the placement is valid according to adjacency constraints and backtracks
/// if necessary.
///
/// # Arguments
///
/// * `puzzle` - A mutable reference to the puzzle being solved.
/// * `missing_tiles` - A reference to a `HashSet` of available tiles.
/// * `current_position` - The index in the puzzle currently being filled.
/// * `start_instant` - A reference to the `Instant` tracking execution time.
///
/// # Returns
///
/// * `true` - If a valid solution is found.
/// * `false` - If no valid placement is possible.
fn solve_puzzle_r(
    puzzle: &mut Puzzle,
    missing_tiles: &HashSet<Tile>,
    current_position: usize,
    start_instant: &Instant,
) -> bool {
    // Base case: all positions are filled successfully
    if current_position == puzzle.len() {
        return true;
    }

    // Skip already filled positions
    if puzzle[current_position].is_some() {
        return solve_puzzle_r(puzzle, missing_tiles, current_position + 1, start_instant);
    }

    // Try each missing tile
    for &element in missing_tiles {
        if is_valid_placement(puzzle, element, current_position) {
            puzzle[current_position] = Some(element);

            if solve_puzzle_r(puzzle, missing_tiles, current_position + 1, start_instant) {
                return true;
            }

            // Backtrack if no solution is found
            puzzle[current_position] = None;
        }

        let flipped_element = element.flip();
        if is_valid_placement(puzzle, flipped_element, current_position) {
            puzzle[current_position] = Some(flipped_element);

            if solve_puzzle_r(puzzle, missing_tiles, current_position + 1, start_instant) {
                return true;
            }

            // Backtrack
            puzzle[current_position] = None;
        }
    }
    false
}

/// Checks whether a tile can be placed at a given position in the puzzle.
///
/// Ensures that:
/// - The tile is not already used elsewhere in the puzzle.
/// - The left neighbor (if any) matches the left side of the tile.
/// - The right neighbor (if any) matches the right side of the tile.
///
/// # Arguments
///
/// * `puzzle` - A reference to the puzzle grid.
/// * `tile` - The tile being placed.
/// * `position` - The index where the tile is being placed.
///
/// # Returns
///
/// * `true` - If the placement is valid.
/// * `false` - If the placement violates constraints.
fn is_valid_placement(puzzle: &Puzzle, tile: Tile, position: usize) -> bool {
    let puzzle_length = puzzle.len();

    puzzle.iter().all(|&slot| slot != Some(tile))
        && (puzzle[(puzzle_length + position - 1) % puzzle_length].is_none()
            || puzzle[(puzzle_length + position - 1) % puzzle_length].unwrap().1 == tile.0)
        && (puzzle[(position + 1) % puzzle_length].is_none()
            || puzzle[(position + 1) % puzzle_length].unwrap().0 == tile.1)
}

/// Determines the missing tiles in the puzzle by comparing with a full tileset.
///
/// The function generates the full set of valid tiles and removes the tiles already present in the puzzle.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` structure.
///
/// # Returns
///
/// * `Ok(HashSet<Tile>)` - The set of missing tiles that need to be placed.
/// * `Err(DominoError::InvalidPuzzle)` - If the puzzle input is invalid.
///
/// # Example
///
/// ```rust
/// let puzzle = Puzzle::new();
/// let missing_tiles = get_missing_tiles(&puzzle);
/// ```
pub fn get_missing_tiles(puzzle: &Puzzle) -> Result<HashSet<Tile>, DominoError> {
    let n = get_n(puzzle)?;

    // Generate the complete set of valid tiles
    let tileset: HashSet<Tile> = (0..=n)
        .flat_map(|i| (0..=n).map(move |j| Tile(i, j)))
        .filter(|tile| {
            if n % 2 == 0 {
                true
            } else {
                (tile.0 as i32 - tile.1 as i32).abs() != ((n as i32 + 1) / 2)
            }
        })
        .collect();

    // Collect all used tiles (including flipped versions)
    let used_tiles: HashSet<Tile> = puzzle
        .iter()
        .filter_map(|&tile| tile.map(|t| vec![t, t.flip()]))
        .flatten()
        .collect();

    // Compute the set of missing tiles
    let missing_tiles: HashSet<Tile> = tileset.difference(&used_tiles).cloned().collect();

    Ok(missing_tiles)
}
