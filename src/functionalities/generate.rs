use rand::Rng;

use crate::types::{domino_types::{Puzzle, Solution, Tile}, graph_types::{graph::Graph, EulerianCycleFinder}};

pub fn generate_puzzle(n: usize) -> Puzzle {
    let graph = Graph::regular(n);
    let eulerian_cycle = EulerianCycleFinder::find_cycle(&graph);
    let solution: Solution = eulerian_cycle
    .into_iter().map(|arc| {
        Tile(arc.source.try_into().unwrap(), arc.destination.try_into().unwrap())
    })
    .collect();
    let mut puzzle: Puzzle = solution.into_iter()
        .map(|tile| Some(tile))
        .collect();
    let mut seed = rand::thread_rng();
    let removals = seed.gen_range(1..=puzzle.len() - (n + 1));
    for _ in 0..removals {
        let mut index = seed.gen_range(0..puzzle.len());
        while puzzle[index].is_none() {
            index = seed.gen_range(0..puzzle.len());
        }
        puzzle[index] = None;
    }
    puzzle
}
