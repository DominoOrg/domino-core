use std::collections::HashSet;

use crate::types::domino_types::{DominoError, Puzzle, SequenceScraper, Tile};

use super::solve::solve_puzzle;

pub fn validate_puzzle(puzzle: &Puzzle) -> Result<bool, DominoError> {
    let solved_puzzle = solve_puzzle(puzzle);
    if solved_puzzle.is_some() {
        let empty_positions: Vec<usize> = puzzle
        .iter()
        .enumerate()
        .filter_map(|(index, tile)| if tile.is_none() { Some(index) } else { None })
        .collect();
        let n = SequenceScraper::get_n(&puzzle)?;
        let tileset: HashSet<Tile> = (0..(n + 1)).map(|i| {
           ((i + 1)..(n + 1)).map(|j| Tile(i, j)).collect::<Vec<Tile>>()
        })
        .flatten()
        .collect();
        let used_tiles: HashSet<Tile> = puzzle.iter().filter_map(|tile| if tile.is_some() { Some(tile.unwrap()) } else { None } ).collect();
        let missing_tiles: HashSet<Tile> = tileset.difference(&used_tiles).cloned().collect();
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
                if new_solved_puzzle.is_none() {
                    return Ok(false);
                }
            }
        }
        return Ok(true);
    } else {
        return Ok(false);
    }
}