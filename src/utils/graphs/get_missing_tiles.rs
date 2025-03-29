use std::collections::HashSet;

use crate::utils::{get_n, DominoError, Puzzle, Tile};

pub fn get_missing_tiles(puzzle: &Puzzle) -> Result<HashSet<Tile>, DominoError> {
    let n = get_n(puzzle)?;
    let tileset: HashSet<Tile> = (0..(n + 1))
        .map(|i| ((0)..(n + 1)).map(|j| Tile(i, j)).collect::<Vec<Tile>>())
        .flatten()
        .filter(|tile| {
            if n % 2 == 0 {
                true
            } else {
                (tile.0 - tile.1).abs() as i32 != ((n as i32 + 1) / 2)
            }
        })
        .collect();
    let used_tiles: HashSet<Tile> = puzzle.0
        .iter()
        .filter_map(|tile| {
            if tile.is_some() {
                Some(vec![tile.unwrap().clone(), tile.unwrap().flip()])
            } else {
                None
            }
        })
        .flatten()
        .collect();
    let missing_tiles: HashSet<Tile> = tileset.difference(&used_tiles).cloned().collect();
    Ok(missing_tiles)
}
