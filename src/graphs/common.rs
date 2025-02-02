use std::collections::HashSet;

use crate::{DominoError, Puzzle, Tile};

pub fn get_n(puzzle: &Puzzle) -> Result<i32, DominoError> {
    if puzzle.len() == 0 {
        return Err(DominoError::InvalidLength);
    }
    let mut tmp: f32 = (-3.0 + (1.0 as f32 + 8.0 as f32 * puzzle.len() as f32).sqrt()) / 2.0;
    if tmp - tmp.floor() == 0.0 {
        return Ok(tmp as i32);
    }
    tmp = -1.0 + (2.0 as f32 * puzzle.len() as f32).sqrt();
    if tmp - tmp.floor() == 0.0 {
        return Ok(tmp as i32);
    }
    tmp = (-1.0 + (1.0 as f32 + 4.0 * puzzle.len() as f32).sqrt()) / 2.0;
    if tmp - tmp.floor() == 0.0 {
        return Ok(tmp as i32);
    }
    tmp = (puzzle.len() as f32).sqrt();
    if tmp - tmp.floor() == 0.0 {
        return Ok(tmp as i32);
    }
    Err(DominoError::InvalidLength)
}

pub fn get_missing_tiles(puzzle: &Puzzle) -> Result<HashSet<Tile>, DominoError> {
    let n = get_n(puzzle)?;
    let tileset: HashSet<Tile> = (0..(n + 1)).map(|i| {
        ((0)..(n + 1)).map(|j| Tile(i, j)).collect::<Vec<Tile>>()
    })
    .flatten()
    .filter(|tile| if n % 2 == 0 {
        true
    } else {
        (tile.0 - tile.1).abs() as i32 != ((n as i32 + 1)/ 2)
    })
    .collect();
    let used_tiles: HashSet<Tile> = puzzle.iter()
    .filter_map(|tile| if tile.is_some() {
        Some(vec![tile.unwrap().clone(), tile.unwrap().flip()])
    } else {
        None
    } )
    .flatten()
    .collect();
    let missing_tiles: HashSet<Tile> = tileset.difference(&used_tiles).cloned().collect();
    Ok(missing_tiles)
}