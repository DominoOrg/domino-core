use crate::{DominoError, Puzzle, Solution, Tile};
use std::collections::HashSet;
use std::time::Instant;

use super::common::get_missing_tiles;

pub fn solve_puzzle(puzzle: &Puzzle) -> Result<Solution, DominoError> {
    let mut new_puzzle = puzzle.clone();
    let missing_tiles = get_missing_tiles(puzzle)?;
    let start_instant = Instant::now();
    if solve_puzzle_r(&mut new_puzzle, &missing_tiles, 0, &start_instant) {
        let solution = new_puzzle.into_iter().map(|tile| tile.unwrap()).collect();
        return Ok(solution);
    } else {
        // if start_instant.elapsed().as_millis() > 100000 {
        //     return Err(DominoError::Timeout);
        // } else {
        return Err(DominoError::UnsolvablePuzzle);
        // }
    }
}

fn solve_puzzle_r(
    puzzle: &mut Puzzle,
    missing_tiles: &HashSet<Tile>,
    current_position: usize,
    start_instant: &Instant,
) -> bool {
    // if start_instant.elapsed().as_millis() > 100000 {
    //     return false;
    // }

    // Base case: if we've gone past the last element, we've found a valid solution
    if current_position == puzzle.len() {
        return true;
    }

    // If the current slot is already filled, move to the next
    if puzzle[current_position].is_some() {
        return solve_puzzle_r(puzzle, missing_tiles, current_position + 1, start_instant);
    }

    // Try each element in the current empty slot
    for &element in missing_tiles {
        // Check if this element can be used (not already in the puzzle and adjacent to its neighbors)
        if puzzle.iter().all(|&slot| slot != Some(element))
            && (puzzle[(puzzle.len() + current_position - 1) % puzzle.len()].is_none()
                || puzzle[(puzzle.len() + current_position - 1) % puzzle.len()]
                    .unwrap()
                    .1
                    == element.0)
            && (puzzle[(current_position + 1) % puzzle.len()].is_none()
                || puzzle[(current_position + 1) % puzzle.len()].unwrap().0 == element.1)
        {
            // Place the element
            puzzle[current_position] = Some(element);

            // Recurse with the next index
            if solve_puzzle_r(puzzle, missing_tiles, current_position + 1, start_instant) {
                return true;
            }

            // Backtrack: if the recursion didn't lead to a solution, remove this element
            puzzle[current_position] = None;
        }

        element.flip();
        // Check if the rotated version of the same tile can be used (not already in the puzzle and adjacent to its neighbors)
        if puzzle.iter().all(|&slot| slot != Some(element))
            && (puzzle[(puzzle.len() + current_position - 1) % puzzle.len()].is_none()
                || puzzle[(puzzle.len() + current_position - 1) % puzzle.len()]
                    .unwrap()
                    .1
                    == element.0)
            && (puzzle[(current_position + 1) % puzzle.len()].is_none()
                || puzzle[(current_position + 1) % puzzle.len()].unwrap().0 == element.1)
        {
            // Place the element
            puzzle[current_position] = Some(element);

            // Recurse with the next index
            if solve_puzzle_r(puzzle, missing_tiles, current_position + 1, start_instant) {
                return true;
            }

            // Backtrack: if the recursion didn't lead to a solution, remove this element
            puzzle[current_position] = None;
        }
    }
    false
}

// #[cfg(test)]
// mod tests {
//   // use crate::Tile;
//   // use super::solve_puzzle;

//   #[test]
//   fn test_solve() {
//     // let puzzle: Vec<Option<Tile>> = vec![
//     //   Some((1,2).into()), None, None, None, None,
//     //   None, None, None, None, None,
//     //   Some((1,4).into()), None, None, None, None
//     // ];
//     // let solution = solve_puzzle(&puzzle).unwrap();
//     // println!("Solution: {solution:?}");
//   }
// }
