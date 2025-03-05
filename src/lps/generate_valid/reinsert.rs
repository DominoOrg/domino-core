use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;

use crate::graphs::Classification;
use crate::validate_puzzle;
use crate::{classify_puzzle, Puzzle, Solution, Tile};

pub(super) fn reinsert_tile_and_check(
    puzzle: Puzzle,
    solution: Solution,
    removed_tiles: Vec<Tile>,
    c: Classification,
    random: bool,
) -> Puzzle {
    // If puzzle is valid and matches desired complexity, return it
    if validate_puzzle(&puzzle, &solution).is_ok() && classify_puzzle(&puzzle).ok() == Some(c) {
        return puzzle;
    }

    // If no tiles to insert and not random, return current puzzle
    if !random && removed_tiles.is_empty() {
        return puzzle;
    }

    // Select tile and position
    let (tile, position) = select_tile_and_position(&puzzle, &solution, &removed_tiles, random);
    let (new_puzzle, new_removed_tiles) =
        reinsert_tile(puzzle, removed_tiles, tile, position).unwrap();

    // Special case for c == 3: stop after one insertion if puzzle becomes valid
    if c == Classification::new(3) && validate_puzzle(&new_puzzle, &solution).is_ok() {
        return new_puzzle;
    }
    // Otherwise, recurse
    reinsert_tile_and_check(new_puzzle, solution, new_removed_tiles, c, random)
}

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
                    tile.is_some() && puzzle[index + 1 % puzzle.len()].is_none()
                })
                .next()
                .unwrap()
                .0
                + 1,
        ),
    }
}

// Rest of the supporting functions remain unchanged
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
    let removed_position = removed_tiles.iter().position(|t| t == &tile).unwrap();
    removed_tiles.splice(removed_position..removed_position + 1, vec![]);
    Ok((puzzle, removed_tiles))
}

fn random_tile(solution: Solution) -> Tile {
    let mut seed = rand::thread_rng();
    solution.choose(&mut seed).unwrap().clone()
}

fn random_empty_position_next_to(puzzle: &Puzzle) -> usize {
    puzzle
        .iter()
        .enumerate()
        .filter(|(index, tile)| tile.is_some() && puzzle[index + 1 % puzzle.len()].is_none())
        .choose(&mut thread_rng())
        .unwrap()
        .0
        + 1
}

fn random_empty_position(puzzle: &Puzzle) -> usize {
    let mut seed = rand::thread_rng();
    puzzle
        .iter()
        .enumerate()
        .filter(|(_index, tile)| tile.is_none())
        .choose(&mut seed)
        .unwrap()
        .0
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
