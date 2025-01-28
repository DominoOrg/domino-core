use rand::Rng;
use crate::types::{graph_types::graph::Graph, Puzzle, Solution, Tile};

use super::graph_common::find_eulerian_cycle;

pub fn generate_puzzle(n: usize, minimum_removals: usize, random: bool) -> Puzzle {
    let graph = Graph::regular(n);
    let eulerian_cycle = find_eulerian_cycle(&graph, random);
    let solution: Solution = eulerian_cycle
    .windows(2).map(|arc| {
        Tile(arc[0].clone().try_into().unwrap(), arc[1].clone().try_into().unwrap())
    })
    .collect();
    let mut puzzle: Puzzle = solution.into_iter()
        .map(|tile| Some(tile))
        .collect();
    if random {
        let mut seed = rand::thread_rng();        
        if puzzle.len() > 0 {
            let removals = seed.gen_range(minimum_removals..=(puzzle.len() - ((2 * n) + 1)));            
            for _ in 0..removals {
                let mut index = seed.gen_range(0..puzzle.len());
                while puzzle[index].is_none() {
                    index = seed.gen_range(0..puzzle.len());
                }
                puzzle[index] = None;
            }    
        }    
    } else {
        puzzle[1] = None;
    }
    puzzle
}
