use std::collections::HashSet;
use crate::types::domino_types::error::DominoError;
use crate::types::domino_types::{Puzzle, Solution, Tile};
use crate::types::graph_types::graph::Graph;
use super::common::{get_missing_tiles, get_n};
use super::graph_common::{lexicographic_2coloring, perfect_elimination_order};

pub fn solve_puzzle(puzzle: &Puzzle) -> Result<Solution, DominoError> {
    let n = get_n(puzzle)?;
    if n >= 4 {
        let pog_graph = Graph::partially_oriented(&puzzle)?;
        let underlying_graph = Graph::underlying(&pog_graph);
        let auxiliary_graph = Graph::auxiliary(&underlying_graph);
        let mut new_puzzle = puzzle.clone();
        let perf_elim_ordering = perfect_elimination_order(&underlying_graph);
        let coloring= lexicographic_2coloring(&auxiliary_graph, &perf_elim_ordering)?;
        coloring.into_iter().filter(|(_, color)| *color == false).for_each(|(node, _)| {
            new_puzzle[node.try_position().unwrap()] = Some(node.try_tile().unwrap().into());
        });
        let solution = new_puzzle.into_iter().map(|tile| tile.unwrap()).collect();
        return Ok(solution);
    } else {
        let mut new_puzzle = puzzle.clone();
        let missing_tiles = get_missing_tiles(puzzle)?;
        if solve_planar_r(&mut new_puzzle, &missing_tiles, 0) {
            let solution = new_puzzle.into_iter().map(|tile| tile.unwrap()).collect();
            return Ok(solution);
        } else {
            return Err(DominoError::InvalidPuzzle("Puzzle is not solvable".to_string()));
        }
    }
}

fn solve_planar_r(puzzle: &mut Puzzle, missing_tiles: &HashSet<Tile>, current_position: usize) -> bool {
    // Base case: if we've gone past the last element, we've found a valid solution
    if current_position == puzzle.len() {
        return true;
    }

    // If the current slot is already filled, move to the next
    if puzzle[current_position].is_some() {
        return solve_planar_r(puzzle, missing_tiles, current_position + 1);
    }

    // Try each element in the current empty slot
    for &element in missing_tiles {
        // Check if this element can be used (not already in the puzzle)
        if puzzle.iter().all(|&slot| slot != Some(element)) {
            // Place the element
            puzzle[current_position] = Some(element);
            
            // Recurse with the next index
            if solve_planar_r(puzzle, missing_tiles, current_position + 1) {
                return true;
            }
            
            // Backtrack: if the recursion didn't lead to a solution, remove this element
            puzzle[current_position] = None;
        }
    }
    false
}