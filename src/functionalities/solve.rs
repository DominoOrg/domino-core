use std::collections::HashSet;
use crate::types::error::DominoError;
use crate::types::{Puzzle, Solution, Tile};
use super::common::get_missing_tiles;

pub fn solve_puzzle(puzzle: &Puzzle) -> Result<Solution, DominoError> {
    let mut new_puzzle = puzzle.clone();
    let missing_tiles = get_missing_tiles(puzzle)?;
    if solve_puzzle_r(&mut new_puzzle, &missing_tiles, 0) {
        let solution = new_puzzle.into_iter().map(|tile| tile.unwrap()).collect();
        return Ok(solution);
    } else {
        return Err(DominoError::UnsolvablePuzzle);
    }
}

fn solve_puzzle_r(puzzle: &mut Puzzle, missing_tiles: &HashSet<Tile>, current_position: usize) -> bool {
    // Base case: if we've gone past the last element, we've found a valid solution
    if current_position == puzzle.len() {
        return true;
    }

    // If the current slot is already filled, move to the next
    if puzzle[current_position].is_some() {
        return solve_puzzle_r(puzzle, missing_tiles, current_position + 1);
    }

    // Try each element in the current empty slot
    for &element in missing_tiles {
        // Check if this element can be used (not already in the puzzle and adjacent to its neighbors)
        if puzzle.iter().all(|&slot| slot != Some(element)) &&
            (
                puzzle[(puzzle.len() + current_position - 1) % puzzle.len()].is_none() ||
                puzzle[(puzzle.len() + current_position - 1) % puzzle.len()].unwrap().1 == element.0
            ) &&
            (
                puzzle[(current_position + 1) % puzzle.len()].is_none() ||
                puzzle[(current_position + 1) % puzzle.len()].unwrap().0 == element.1
            ) {
            // Place the element
            puzzle[current_position] = Some(element);
            
            // Recurse with the next index
            if solve_puzzle_r(puzzle, missing_tiles, current_position + 1) {
                return true;
            }
            
            // Backtrack: if the recursion didn't lead to a solution, remove this element
            puzzle[current_position] = None;
        }

        element.flip();
        // Check if the rotated version of the same tile can be used (not already in the puzzle and adjacent to its neighbors)
        if puzzle.iter().all(|&slot| slot != Some(element)) &&
        (
            puzzle[(puzzle.len() + current_position - 1) % puzzle.len()].is_none() ||
            puzzle[(puzzle.len() + current_position - 1) % puzzle.len()].unwrap().1 == element.0
        ) &&
        (
            puzzle[(current_position + 1) % puzzle.len()].is_none() ||
            puzzle[(current_position + 1) % puzzle.len()].unwrap().0 == element.1
        ) {
            // Place the element
            puzzle[current_position] = Some(element);
            
            // Recurse with the next index
            if solve_puzzle_r(puzzle, missing_tiles, current_position + 1) {
                return true;
            }
            
            // Backtrack: if the recursion didn't lead to a solution, remove this element
            puzzle[current_position] = None;
        }
    }
    false
}