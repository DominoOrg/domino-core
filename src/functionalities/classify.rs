use crate::types::Puzzle;

use super::common::get_n;

pub fn classify_puzzle(puzzle: &Puzzle) -> usize {
    let n = get_n(puzzle).expect("Puzzle does not have a valid length");
    let l = puzzle.len() as i32;
    let max_holes = l - (n + 1);
    let max_complexity = max_holes - 1;
    let complexity_classes_each = max_complexity / 3;
    let mut max_contigous_holes = 0;
    let mut current_hole = 0;
    puzzle
        .iter()
        .enumerate()
        .for_each(|(i, tile)| {
            if (
                i > 0 && tile.is_none() && puzzle[0].is_some()
            ) || (
                i == 0 && tile.is_none() && puzzle[i - 1].is_some()
            ) {
                current_hole += 1;
            }
            if (
                i > 0 && tile.is_some() && puzzle[i - 1].is_none()
            ) || (
                i == 0 && tile.is_some() && puzzle[puzzle.len() - 1].is_none()
            ) {
                if current_hole > max_contigous_holes {
                    max_contigous_holes = current_hole;
                }
                current_hole = 0;
            }
        });
    let complexity = max_contigous_holes - 1;
    ((complexity / complexity_classes_each) + 1) as usize
}