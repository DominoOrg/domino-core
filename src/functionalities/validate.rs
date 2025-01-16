use crate::types::{error::DominoError, Puzzle};

use super::{common::{get_empty_positions, get_missing_tiles}, solve::solve_puzzle};

pub fn validate_puzzle(puzzle: &Puzzle) -> Result<bool, DominoError> {
    let solved_puzzle = solve_puzzle(puzzle);
    if solved_puzzle.is_ok() {
        let empty_positions: Vec<usize> = get_empty_positions(puzzle)?;
        let missing_tiles = get_missing_tiles(puzzle)?;
        for empty_position in empty_positions {
            for tile in missing_tiles.iter() {
                let mut new_puzzle = puzzle.clone();
                if empty_position > 0 {
                    if let Some(previous_tile) = new_puzzle[empty_position - 1] {
                        if previous_tile.1 != tile.0 {
                            continue;
                        }
                    }                        
                }
                if empty_position < puzzle.len() - 1 {
                    if let Some(next_tile) = new_puzzle[empty_position + 1] {
                        if next_tile.0 != tile.1 {
                            continue;
                        }
                    }                        
                }
                new_puzzle[empty_position] = Some(*tile);
                let new_solved_puzzle = solve_puzzle(&new_puzzle);
                if new_solved_puzzle.is_ok() {
                    return Ok(false);
                }
            }
        }
        return Ok(true);
    } else {
        return Ok(false);
    }
}