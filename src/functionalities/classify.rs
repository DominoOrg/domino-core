use crate::types::Puzzle;

use super::common::get_n;

pub fn classify_puzzle(puzzle: &Puzzle) -> usize {
    let n = get_n(puzzle).expect("Puzzle does not have a valid length");
    let max_hole = 2 * n + 1;
    let max_contigous_hole = max_hole_length(puzzle);
    let ratio = max_contigous_hole as f32 / max_hole as f32;
    let threshold1 = 4.0 * (1.0 / 7.0);
    let threshold2 = threshold1 + 2.0 * (1.0 / 7.0);
    let class = if ratio < threshold1 {
        1
    } else if ratio >= threshold1 && ratio < threshold2 {
        2
    } else {
        3
    };
    class
}

fn max_hole_length(puzzle: &Puzzle) -> i32 {
    let mut max_contigous_holes = 0;
    let mut current_hole = 0;
    puzzle
        .iter()
        .enumerate()
        .for_each(|(i, tile)| {
            if (
                i > 0 && tile.is_none() && puzzle[i - 1].is_some()
            ) || (
                i == 0 && tile.is_none() && puzzle[puzzle.len() - 1].is_some()
            ) {
                current_hole = 1;
            }

            if tile.is_none() && puzzle[(i + 1) % puzzle.len()].is_none() {
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
    max_contigous_holes
}

#[cfg(test)]
mod tests {
    use crate::{functionalities::classify::max_hole_length, types::Tile};

    #[test]
    fn test_hole_length() {
        for n in [3,6] {
            let l = if n%2==0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
            let puzzle = vec![Some(Tile(0,0));l];
            for i in 1..(l - (n + 1)) {
                let mut p = puzzle.clone();
                for j in 1..i+1 {
                    p[j] = None;                
                }
                assert_eq!(max_hole_length(&p), i as i32);
            }
        }
    }
}
