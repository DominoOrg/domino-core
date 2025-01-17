use crate::types::{error::DominoError, Puzzle};

use super::{common::{get_empty_positions, get_missing_tiles}, solve::solve_puzzle};

pub fn validate_puzzle(puzzle: &Puzzle) -> Result<(), DominoError> {
    if let Ok(solved_puzzle) = solve_puzzle(puzzle) {
        let empty_positions: Vec<usize> = get_empty_positions(&puzzle)?;
        let missing_tiles = get_missing_tiles(&puzzle)?;
        
        // If the puzzle had a single tile missing and the tile remaining fits the only hole in the puzzle
        // it is already valid
        if missing_tiles.len() == 1 && empty_positions.len() == 1
        {
            let missing_tile = missing_tiles.iter().next().unwrap();
            let empty_position = empty_positions[0];
            if (
                    (empty_position == 0 || puzzle[empty_position-1].unwrap().1 == missing_tile.0) &&
                    (empty_position == puzzle.len() - 1 || puzzle[empty_position+1].unwrap().0 == missing_tile.1)
                ) ||
                (
                    (empty_position == 0 || puzzle[empty_position-1].unwrap().0 == missing_tile.1) &&
                    (empty_position == puzzle.len() - 1 || puzzle[empty_position+1].unwrap().1 == missing_tile.0)
                )
            {
                return Ok(());
            }
            
        } 

        // Otherwise setup a new version of the puzzle to solve with an additional filled tile
        let mut new_puzzle = puzzle.clone();
        for empty_position in empty_positions {
            for tile in missing_tiles.iter() {
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

                // If the new version of the puzzle with an additional filled space is not solvable
                // then skip to the next variation trying new combinations
                if new_solved_puzzle.is_err() {
                    new_puzzle[empty_position] = None;
                    continue;
                }
                // If the new solved version of the puzzle is not the same solution to the original puzzle
                // Then it means the puzzle has more than 1 solution and its considered not valid
                if let Ok(new_solved_puzzle) = new_solved_puzzle {
                    let mut is_same = true;
                    for i in 0..solved_puzzle.len() {
                        if puzzle[i].is_none() && new_solved_puzzle[i] != solved_puzzle[i] {
                            is_same = false;
                            break;
                        }
                    }
                    if !is_same {
                        return Err(DominoError::NotValidPuzzle);
                    };
                }
                // Otherwise if the new version of the puzzle has the same solution of the original puzzle
                // then the puzzle for now is considered valid and to compute other versions of the puzzle
                // the inserted tile is removed
                new_puzzle[empty_position] = None;
            }
        }
        return Ok(());
    } else {
        return Err(DominoError::UnsolvablePuzzle);
    }
}