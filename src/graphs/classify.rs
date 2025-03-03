use std::f32;

use crate::Puzzle;

use super::common::get_n;

const NUMBER_OF_CLASSES: usize = 3;

pub fn classify_puzzle(puzzle: &Puzzle) -> usize {
    let n: usize = get_n(puzzle).expect("Puzzle does not have a valid length").try_into().unwrap();
    let l: usize = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
    let max_hole = l as f32 - (n as f32 / 2.0).floor();
    let holes: Vec<(usize, usize)> = detect_holes(puzzle);
    if holes.len() == 0 {
      return 0;
    }
    let absolute_complexity = compute_complexity(holes, max_hole);
    absolute_complexity
}

fn compute_complexity(holes: Vec<(usize, usize)>, max_hole: f32) -> usize {
    let relative_complexity = compute_relative_logarithmic_complexity(holes, max_hole);
    println!("relative_complexity: {relative_complexity}");
    let class = map_to_class(relative_complexity);
    println!("class: {class}");
    class
}

// Computes relative complexity on a logarithmic scale between 0.0 and 1.0
fn compute_relative_logarithmic_complexity(holes: Vec<(usize, usize)>, max_hole: f32) -> f32 {
    let puzzle_absolute_complexity = absolute_complexity(holes, max_hole);
    // Maximum possible complexity with single hole of max size
    let max_value = absolute_complexity(vec![(0, max_hole as usize)], max_hole);

    // Ensure we don't get negative or zero before ln()
    let adjusted_complexity = puzzle_absolute_complexity + 1.0;
    let adjusted_max = max_value + 1.0;

    // Calculate logarithmic ratio between 0 and 1
    let log_base = 100.0;
    let ratio = (adjusted_complexity.log(log_base) / adjusted_max.log(log_base)).clamp(0.0, 1.0);
    ratio
}

// Maps 0.0-1.0 relative complexity to classes 1-3
fn map_to_class(relative_complexity: f32) -> usize {
    if relative_complexity.is_nan() {
        return 0;
    }

    // Scale 0.0-1.0 to 1-3
    let scaled = (relative_complexity * 2.0) + 1.0;  // Maps to 1.0-3.0
    let class = scaled.round() as usize;
    class.clamp(1, 3)  // Ensure output stays between 1 and 3
}

fn absolute_complexity(
  holes: Vec<(usize, usize)>,
  max_hole: f32
) -> f32 {
  let number_of_holes_factor = 1.0 / ((holes.len() as f32).powf(0.1));
  let length_factor = holes.into_iter().map(|hole| (
    hole.1.saturating_sub(hole.0) as f32 /
    max_hole as f32)
    .powf(2.0)).sum::<f32>();
  let absolute_complexity = number_of_holes_factor * length_factor;
  absolute_complexity
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

        Some(result)
    })
    .filter_map(|x| x)
    .collect();

  holes
}

pub fn invert_logaritmic_scale(value: f32, max_hole: usize) -> f32 {
  let log_base: f32 = 100.0;
  let relative_max = absolute_complexity(vec![(0, max_hole)], max_hole as f32);

  log_base.powf(value * (relative_max + 1.0).log(log_base)) - 1.0
}

pub fn relative_complexity_from_class(class: usize) -> f32 {
  class as f32 / (NUMBER_OF_CLASSES as f32 - 1.0)
}

