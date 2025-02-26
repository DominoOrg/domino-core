use crate::Puzzle;

use super::common::get_n;

#[cfg(test)]
mod tests {
    use std::vec;


    use crate::{Puzzle, Tile};

    use super::classify_puzzle;

    fn mock_puzzle(n: usize, c: usize) -> Puzzle {
      let l = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
      let mut puzzle: Puzzle = vec![Some(Tile::from((0,0))); l];
      let max_hole = (l as f32 - (n as f32 / 2.0).floor()) as usize;
      let log_factor: f32 = match c {
        1 => 1.0 / puzzle.len() as f32,
        2 => 4.0 / 7.0,
        3 => 6.0 / 7.0,
        _ => 0.0
      };
      for i in 0..(max_hole as f32  * log_factor.sqrt()).ceil() as usize {
        puzzle[i] = None;
      }
      puzzle
    }

    #[test]
    fn classify_unit() {
      for mock_c in 1..=3 {
        let puzzle = mock_puzzle(6, mock_c);
        let c = classify_puzzle(&puzzle);
        assert_eq!(c, mock_c);
      }
    }

}

pub fn classify_puzzle(puzzle: &Puzzle) -> usize {
    let n = get_n(puzzle).expect("Puzzle does not have a valid length");
    let l = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
    let max_hole = l as f32 - (n as f32 / 2.0).floor();
    let holes: Vec<(usize, usize)> = detect_holes(puzzle);
    let absolute_complexity = compute_complexity(holes, max_hole);
    absolute_complexity
}

fn compute_complexity(holes: Vec<(usize, usize)>, max_hole: f32) -> usize {
  let number_of_holes_factor = 1.0 / ((holes.len() as f32).sqrt().sqrt().sqrt().sqrt().sqrt());
  let length_factor = holes.into_iter().map(|hole| (hole.1.saturating_sub(hole.0) as f32 /max_hole as f32)
  .powf(2.0)).sum::<f32>();
  // println!("number_of_holes_factor: {number_of_holes_factor}, length_factor: {length_factor}");
  let complexity = number_of_holes_factor * length_factor;
  let c = match complexity {
    c if c < 4.0 / 7.0 => 1,
    c if c < 6.0 / 7.0 => 2,
    _ => 3
  };
  c
}

fn detect_holes(puzzle: &Puzzle) -> Vec<(usize, usize)> {
  let invalid_index = puzzle.len()+1;
  let holes: Vec<(usize, usize)> = puzzle
    .iter()
    .enumerate()
    .scan((invalid_index, invalid_index), |current_hole, (i, &item)| {
        let is_hole_start = item.is_none() &&
            ((i > 0 && puzzle[i-1].is_some()) ||
             (i == 0 && puzzle[puzzle.len() - 1].is_some()));

        if is_hole_start {
            current_hole.0 = i;
        }

        let is_hole_end = item.is_none() &&
            ((i < puzzle.len() - 1 && puzzle[i+1].is_some()) ||
             (i == puzzle.len() - 1 && puzzle[0].is_some()));

        let mut result = None;
        if is_hole_end {
            current_hole.1 = i + 1;
            if current_hole.0 != invalid_index {
                result = Some(*current_hole);
            }
            *current_hole = (invalid_index, invalid_index);
        }

        // println!("{i} {is_hole_start} {is_hole_end} {current_hole:?}");

        Some(result)
    })
    .filter_map(|x| x)
    .collect();

  holes
}
