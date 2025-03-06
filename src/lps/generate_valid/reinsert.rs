use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;

use crate::graphs::ComplexityClass;
use crate::validate_puzzle;
use crate::{classify_puzzle, Puzzle, Solution, Tile};

/// Attempts to reinsert tiles into the puzzle and checks if the puzzle meets the desired complexity.
///
/// If the puzzle is already valid and matches the given complexity class, it is returned as-is.
/// If there are no more tiles to insert and `random` is false, the puzzle is returned as well.
/// Otherwise, a tile is selected and reinserted, and the function recurses until conditions are met.
///
/// Special case: If `c` is `3`, the function stops after the first successful insertion.
///
/// # Arguments
/// * `puzzle` - The current puzzle state.
/// * `solution` - The complete solution for reference.
/// * `removed_tiles` - A list of removed tiles to be reinserted.
/// * `c` - The desired complexity class.
/// * `random` - Whether tile selection should be random.
///
/// # Returns
/// The modified puzzle after reinsertion attempts.
pub(super) fn reinsert_tile_and_check(
  puzzle: Puzzle,
  solution: Solution,
  removed_tiles: Vec<Tile>,
  c: ComplexityClass,
  random: bool,
) -> Puzzle {
  if validate_puzzle(&puzzle, &solution).is_ok() && classify_puzzle(&puzzle).ok() == Some(c) {
      return puzzle;
  }
  if !random && removed_tiles.is_empty() {
      return puzzle;
  }

  let (tile, position) = select_tile_and_position(&puzzle, &solution, &removed_tiles, random);
  let (new_puzzle, new_removed_tiles) =
      reinsert_tile(puzzle, removed_tiles, tile, position).unwrap();

  if c == ComplexityClass::new(3) && validate_puzzle(&new_puzzle, &solution).is_ok() {
      return new_puzzle;
  }
  reinsert_tile_and_check(new_puzzle, solution, new_removed_tiles, c, random)
}

/// Selects a tile and its reinsertion position based on strategy.
///
/// If `random` is true, a tile is randomly chosen from either the solution or removed tiles,
/// and a random valid position is selected. Otherwise, the first available tile and position
/// are chosen deterministically.
///
/// # Arguments
/// * `puzzle` - The current puzzle state.
/// * `solution` - The reference solution.
/// * `removed_tiles` - The tiles available for reinsertion.
/// * `random` - Whether selection should be random.
///
/// # Returns
/// A tuple containing the selected tile and its position.
fn select_tile_and_position(
  puzzle: &Puzzle,
  solution: &Solution,
  removed_tiles: &Vec<Tile>,
  random: bool,
) -> (Tile, usize) {
  match (random, removed_tiles.len() == solution.len()) {
      (true, true) => (random_tile(solution.clone()), random_empty_position(puzzle)),
      (true, false) => (
          random_tile(removed_tiles.clone()),
          random_empty_position_next_to(puzzle),
      ),
      (false, true) => (removed_tiles[0], 0),
      (false, false) => (
          removed_tiles[0].clone(),
          puzzle
              .iter()
              .enumerate()
              .filter(|(index, tile)| {
                  tile.is_some() && puzzle[(index + 1) % puzzle.len()].is_none()
              })
              .next()
              .map(|(index, _)| (index + 1) % puzzle.len())
              .unwrap_or(0),
      ),
  }
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

/// Selects a random tile from the given solution.
///
/// # Arguments
/// * `solution` - The reference solution.
///
/// # Returns
/// A randomly chosen tile from the solution.
fn random_tile(solution: Solution) -> Tile {
  let mut seed = rand::thread_rng();
  solution.choose(&mut seed).unwrap_or(&solution[0]).clone()
}

/// Selects a random empty position next to an existing tile.
///
/// # Arguments
/// * `puzzle` - The current puzzle state.
///
/// # Returns
/// A valid empty position adjacent to an existing tile.
fn random_empty_position_next_to(puzzle: &Puzzle) -> usize {
  puzzle
      .iter()
      .enumerate()
      .filter(|(index, tile)| tile.is_some() && puzzle[(index + 1) % puzzle.len()].is_none())
      .choose(&mut thread_rng())
      .map(|(index, _)| (index + 1) % puzzle.len())
      .unwrap_or(0)
}

/// Selects a random empty position in the puzzle.
///
/// # Arguments
/// * `puzzle` - The current puzzle state.
///
/// # Returns
/// A randomly chosen empty position.
fn random_empty_position(puzzle: &Puzzle) -> usize {
  let mut seed = rand::thread_rng();
  puzzle
      .iter()
      .enumerate()
      .filter(|(_index, tile)| tile.is_none())
      .choose(&mut seed)
      .map(|(index, _)| index)
      .unwrap_or(0)
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
