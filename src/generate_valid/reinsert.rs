use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::classify::tiles_to_remove_range;
use crate::utils::get_n;
use crate::{classify_puzzle, ComplexityClass, Puzzle, Solution, Tile};
use crate::{validate_puzzle, DominoError};

/// Attempts to reinsert tiles into the puzzle and checks if the puzzle meets the desired complexity.
///
/// If the puzzle is already valid, has the given complexity class, and the number of removed tiles (as computed
/// by `tiles_to_remove_range`) is within the valid range, it is returned as-is.
/// If there are no more tiles to reinsert and the puzzle does not match the requirements, an error is produced.
/// Otherwise, a tile is selected and reinserted, and the function recurses until the conditions are met.
///
/// Special case: If `c` is `3`, the function stops after the first successful insertion.
///
/// # Arguments
/// * `puzzle` - The current puzzle state.
/// * `solution` - The complete solution for reference.
/// * `removed_tiles` - A list of removed tiles to be reinserted.
/// * `c` - The desired complexity class.
/// * `anchor` - The last inserted tile's position to ensure adjacency.
///
/// # Returns
/// * `Ok(Puzzle)` if a valid puzzle with the required complexity is produced.
/// * `Err(DominoError)` if no reinsertion attempt produces a puzzle that is both valid and has the required complexity.
pub(super) fn reinsert_tile_and_check(
    puzzle: Puzzle,
    solution: Solution,
    removed_tiles: Vec<Tile>,
    c: ComplexityClass,
    anchor: usize,
) -> Result<Puzzle, DominoError> {
    // Retrieve the puzzle dimension 'n'
    let n: usize = get_n(&puzzle)? as usize;
    // Compute the valid range of removed tiles for the desired complexity class using the inverse classification formula.
    let (min_removed, max_removed) = tiles_to_remove_range(c, n);

    println!(
        "Reinsertion attempt: current_removed_count = {}, expected range = [{}, {}]",
        removed_tiles.len(),
        min_removed,
        max_removed
    );

    let current_removed_count = removed_tiles.len();

    // If too few tiles remain removed, then we've reinserted too many already.
    if current_removed_count < min_removed {
        return Err(DominoError::GenerationError(
        "Too many tiles have been reinserted. Insufficient removed tiles remain to achieve the desired complexity.".to_owned()
    ));
    }

    // If the puzzle is valid, its complexity matches, and the removed count is within the valid bounds, we're done.
    let validation_result = validate_puzzle(&puzzle, &solution);
    let classification_result = classify_puzzle(&puzzle);
    let classification_option = classification_result.ok();

    if validation_result.is_ok() && classification_option.is_some() {
        if classification_option == Some(c)
            && current_removed_count >= min_removed
            && current_removed_count <= max_removed
        {
            return Ok(puzzle);
        }
    }
    // If there are no more tiles to reinsert and the conditions are not met, return an error.
    if removed_tiles.is_empty() {
        return Err(DominoError::GenerationError(
        "No more tiles available for reinsertion; failed to produce a puzzle with the required complexity.".to_owned()
    ));
    }

    // Select a tile and a position for reinsertion.
    let (tile, position) = select_tile_and_position(&puzzle, &solution, anchor);
    // Attempt to reinsert the tile. Propagate an error if reinsertion fails.
    let (new_puzzle, new_removed_tiles) =
        reinsert_tile(puzzle.clone(), removed_tiles, tile, position).or_else(|_| {
            Err(DominoError::GenerationError(
                "Tile reinsertion failed during generation of the desired puzzle complexity."
                    .to_owned(),
            ))
        })?;

    println!(
          "After reinsertion: puzzle validity = {:?}, classification = {:?} for puzzle \n{:?}\n for solution: \n{solution:?}--------------\n",
          validation_result,
          classification_option,
          &puzzle
      );

    // Special case: for the highest complexity class, stop after the first successful insertion.
    if Some(c) == classification_option && validate_puzzle(&new_puzzle, &solution).is_ok() {
        return Ok(new_puzzle);
    }

    // Update the anchor to the last inserted position.
    let new_anchor = position;

    // Recursively try reinsertion until a valid puzzle is produced or no more reinsertion attempts remain.
    reinsert_tile_and_check(new_puzzle, solution, new_removed_tiles, c, new_anchor)
}

/// Selects an empty position in `puzzle` and retrieves the corresponding tile from `solution` for reinsertion.
///
/// For the first reinsertion (i.e. when `anchor` is `None`), a random empty position is selected and used as
/// the anchor. For subsequent reinsertions (when `anchor` is `Some(pos)`), an empty position adjacent (cyclically)
/// to the anchor is chosen. If no adjacent empty position is available, the function falls back to selecting a
/// random empty position.
///
/// # Arguments
///
/// * `puzzle` - A slice representing the current puzzle state.
/// * `solution` - A slice representing the reference solution for the puzzle.
/// * `anchor` - An optional anchor position. If `None`, this is the first reinsertion; if `Some(pos)`, subsequent
///   reinsertions are attempted adjacent to `pos`.
///
/// # Returns
///
/// A tuple `(Tile, usize, Option<usize>)` where:
/// - The first element is the tile from the solution corresponding to the chosen empty position.
/// - The second element is the index of the chosen empty position.
///
/// # Panics
///
/// Panics if no empty position is found in the puzzle.
fn select_tile_and_position(puzzle: &Puzzle, solution: &Solution, anchor: usize) -> (Tile, usize) {
    let mut rng = thread_rng();
    let len = puzzle.len();

    // Collect all empty positions in the puzzle.
    let empty_positions: Vec<usize> = puzzle
        .iter()
        .enumerate()
        .filter(|(_idx, tile)| tile.is_none())
        .map(|(idx, _)| idx)
        .collect();

    if empty_positions.is_empty() {
        panic!("No empty position available for reinsertion");
    }

    // If an anchor is provided, attempt to select an empty position adjacent to the anchor.
    let a = anchor;
    let left = if a == 0 { len - 1 } else { a - 1 };
    let right = (a + 1) % len;

    let mut adjacent_positions = vec![];
    if puzzle[left].is_none() {
        adjacent_positions.push(left);
    }
    if puzzle[right].is_none() {
        adjacent_positions.push(right);
    }

    let pos = if !adjacent_positions.is_empty() {
        adjacent_positions.into_iter().choose(&mut rng).unwrap()
    } else {
        // Fallback: choose any random empty position if no adjacent position is available.
        empty_positions.into_iter().choose(&mut rng).unwrap()
    };

    let candidate = solution[pos].clone();
    (candidate, pos)
}

/// Reinserts a tile into the puzzle at a specified position.
///
/// If the tile is not in the list of removed tiles or the position is invalid, an error is returned.
/// Otherwise, the tile is inserted, and it is removed from `removed_tiles`.
///
/// # Arguments
/// * `puzzle` - The current puzzle state.
/// * `removed_tiles` - The tiles available for reinsertion.
/// * `tile` - The tile to be reinserted.
/// * `position` - The position where the tile should be placed.
///
/// # Returns
/// A `Result` containing the updated puzzle and remaining removed tiles, or an error message.
fn reinsert_tile(
    puzzle: Puzzle,
    removed_tiles: Vec<Tile>,
    tile: Tile,
    position: usize,
) -> Result<(Puzzle, Vec<Tile>), String> {
    if !removed_tiles.contains(&tile) {
        return Err("Tile not in removed tiles".to_string());
    }
    if position >= puzzle.len() {
        return Err("Invalid position".to_string());
    }
    let mut removed_tiles = removed_tiles.clone();
    let mut puzzle = puzzle.clone();
    puzzle[position] = Some(tile);
    if let Some(removed_position) = removed_tiles.iter().position(|t| t == &tile) {
        removed_tiles.remove(removed_position);
    }
    Ok((puzzle, removed_tiles))
}

#[cfg(test)]
mod tests {
    use crate::{solve_puzzle, Tile};

    #[test]
    fn test_reinsert_tile_and_check() {
        let puzzle = vec![Some(Tile(0, 0)), None, None, None, None, None, None, None];
        let solution = solve_puzzle(&puzzle);
        assert!(solution.is_ok());
    }

    #[test]
    fn test_reinsert_tile_with_valid_tile_and_valid_position() {
        let puzzle: Vec<Option<Tile>> = vec![None, None, None, None, None, None, None, None];
        let removed_tiles: Vec<Tile> = vec![Tile(1, 1), Tile(2, 2), Tile(3, 3)];
        let tile: Tile = Tile(3, 3);
        let position: usize = 0;
        let (puzzle, removed_tiles_after_removal) =
            super::reinsert_tile(puzzle, removed_tiles.clone(), tile, position).unwrap();

        // Test reinsertion on valid position with valid tile
        assert_eq!(
            puzzle,
            vec![Some(tile), None, None, None, None, None, None, None]
        );
        let mut expected_after_removal = removed_tiles.clone();
        let tile_position = expected_after_removal
            .iter()
            .position(|&x| x == tile)
            .unwrap();
        expected_after_removal.splice(tile_position..tile_position + 1, vec![]);
        assert_eq!(removed_tiles_after_removal, expected_after_removal);
    }

    #[test]
    fn test_reinsert_tile_with_invalid_tile() {
        let puzzle: Vec<Option<Tile>> = vec![None, None, None, None, None, None, None, None];
        let removed_tiles: Vec<Tile> = vec![Tile(1, 1), Tile(2, 2), Tile(3, 3)];
        let tile = Tile(4, 4);
        let position: usize = 0;

        let result = super::reinsert_tile(puzzle, removed_tiles.clone(), tile, position);
        assert!(result.is_err());
    }

    #[test]
    fn test_reinsert_tile_with_invalid_position() {
        let puzzle: Vec<Option<Tile>> = vec![None, None, None, None, None, None, None, None];
        let removed_tiles: Vec<Tile> = vec![Tile(1, 1), Tile(2, 2), Tile(3, 3)];
        let tile: Tile = Tile(3, 3);
        let position: usize = 8;

        let result = super::reinsert_tile(puzzle, removed_tiles.clone(), tile, position);
        assert!(result.is_err());
    }
}
