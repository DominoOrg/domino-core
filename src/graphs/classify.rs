use crate::Puzzle;

use super::common::get_n;

pub fn classify_puzzle(puzzle: &Puzzle) -> usize {
    let n = get_n(puzzle).expect("Puzzle does not have a valid length");
    let l = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
    let max_hole = (l as f32 - (n as f32 / 2.0).floor()) as usize;
    let max_contigous_hole = max_hole_length(puzzle) as usize;
    let class = if max_contigous_hole < (max_hole * 4) / 7 {
        1
    } else if max_contigous_hole < (max_hole * 6) / 7 {
        2
    } else {
        3
    };
    class
}

fn max_hole_length(puzzle: &Puzzle) -> usize {
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
    use crate::{graphs::classify::max_hole_length, Tile};


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
                assert_eq!(max_hole_length(&p), i);
            }
        }
    }
}
