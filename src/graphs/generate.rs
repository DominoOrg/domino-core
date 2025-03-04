use rand::Rng;

use crate::{types::Graph, Puzzle, Solution, Tile};

use super::graph_common::find_eulerian_cycle;

// #[deprecated(since = "0.9.8", note = "Use `generate_valid_puzzle` instead")]
pub fn generate_puzzle(n: usize, minimum_removals: usize, random: bool) -> Puzzle {
    let graph = Graph::regular(n);
    let eulerian_cycle = find_eulerian_cycle(&graph)(random);
    let solution: Solution = eulerian_cycle
    .windows(2).map(|arc| {
        Tile(arc[0].clone().try_into().unwrap(), arc[1].clone().try_into().unwrap())
    })
    .collect();
    let mut puzzle: Puzzle = solution.into_iter()
        .map(|tile| Some(tile))
        .collect();
    if !random {
        if puzzle.len() > minimum_removals {
            for index in 0..minimum_removals {
                puzzle[index] = None;
            }
        }
    } else {
        if puzzle.len() > minimum_removals {
            let mut seed = rand::thread_rng();
            for _ in 0..minimum_removals {
                let mut index = seed.gen_range(0..puzzle.len());
                while puzzle[index].is_none() {
                    index = seed.gen_range(0..puzzle.len());
                }
                puzzle[index] = None;
            }
        }
    }
    puzzle
}
