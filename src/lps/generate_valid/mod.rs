use crate::graphs::{find_eulerian_cycle, Classification};
use crate::{types::Graph, Puzzle, Solution, Tile};
use rand::seq::SliceRandom;
use rand::thread_rng;
use reinsert::reinsert_tile_and_check;
use std::vec;

mod reinsert;

pub fn generate_valid_puzzle(
    n: usize,
) -> Box<dyn Fn(Classification) -> Box<dyn Fn(bool) -> Puzzle>> {
    Box::new(move |c: Classification| {
        Box::new(move |random| {
            let graph = Graph::regular(n);
            // Since the eulerian path is represented as the sequence of the nodes traversed
            // The solution is built groupping 2 by 2 each node traversed and
            // building a tile from each couple
            let solution: Solution = find_eulerian_cycle(&graph)(random)
                .windows(2)
                .map(|arc| {
                    Tile(
                        arc[0].clone().try_into().unwrap(),
                        arc[1].clone().try_into().unwrap(),
                    )
                })
                .collect();

            let len = solution.len();
            // When building with random set to false the eulerian path starts with a double tile
            // So to have a puzzle that has c == 3 for n == 3 the only way to make it in a NON RANDOM
            // way is to make it start with a single tile, so it's done
            // rotating cyclically the sequence by one to the left
            let rotated_left_solution = solution.into_iter().cycle().skip(1).take(len).collect();
            let puzzle = generate_puzzle(rotated_left_solution)(c)(random);
            puzzle
        })
    })
}

fn generate_puzzle(
    solution: Solution,
) -> Box<dyn Fn(Classification) -> Box<dyn Fn(bool) -> Puzzle>> {
    Box::new(move |c: Classification| {
        Box::new({
            let value = solution.clone();
            move |random: bool| -> Puzzle {
                let starting_puzzle: Vec<Option<Tile>> = vec![None; value.clone().len()];
                let removed_tiles: Vec<Tile> = if random {
                    let mut removed = value.clone();
                    removed.shuffle(&mut thread_rng());
                    removed
                } else {
                    value.clone()
                };
                let puzzle: Puzzle = reinsert_tile_and_check(
                    starting_puzzle,
                    value.clone(),
                    removed_tiles,
                    c,
                    random,
                );
                puzzle
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::graphs::Classification;

    fn max_hole_size_by_class(max_hole: usize, c: Classification) -> usize {
        let relative_complexity: f32 = Into::<f32>::into(c) / 2.0;
        let hole_size = (relative_complexity.sqrt() * max_hole as f32).ceil();
        (hole_size as usize).saturating_sub(1)
    }

    #[test]
    fn test_generate_valid_puzzle() {
      (3..=6).into_iter().for_each(|n| {
        (1..=3).into_iter().rev().for_each(|c| {
          let c = Classification::new(c);
          let puzzle = super::generate_valid_puzzle(n)(c)(false);
          let puzzle_len = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
          assert_eq!(puzzle.len(), puzzle_len);
          // let is_planar = n <= 3;
          // let max_hole = if is_planar {
          //   puzzle_len as f32 - (n as f32 / 2.0).floor()
          // } else {
          //   puzzle_len as f32 - (n as f32 + 2.0)
          // };
          // let min_hole_size = if c == Classification::new(1) { 0 } else { max_hole_size_by_class(max_hole as usize, Classification::new(c.0 - 1)) as usize };
          // let max_hole_size = max_hole_size_by_class(max_hole as usize, c) as usize;
          // let hole_size = puzzle.iter().filter(|tile| tile.is_none()).count();
          // assert!(hole_size >= min_hole_size && hole_size <= max_hole_size);
        });
      });
    }
}
