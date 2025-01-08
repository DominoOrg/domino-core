use std::collections::HashSet;

use rand::Rng;

pub fn generate_puzzle(sequence: &Vec<(usize, usize)>, n: usize) -> Vec<Option<(usize, usize)>> {
    let mut seed: rand::prelude::ThreadRng = rand::thread_rng();
    let mut puzzle: Vec<Option<(usize, usize)>> = sequence
        .clone()
        .iter()
        .map(|tile| Some(*tile))
        .collect::<Vec<Option<(usize, usize)>>>();
    let to_remove: usize = seed.gen_range(0..=(sequence.len() - n - 1));
    let mut removed: HashSet<usize> = HashSet::new();

    while removed.len() != to_remove {
        let mut index = seed.gen_range(0..puzzle.len());
        while removed.contains(&index) {
            index = seed.gen_range(0..puzzle.len());
        }
        removed.insert(index);
    }

    for index in removed.into_iter() {
        puzzle[index] = None;
    }

    return puzzle;
}
