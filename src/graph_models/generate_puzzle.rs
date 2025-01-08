use std::collections::HashSet;

use rand::{thread_rng, Rng};

use crate::domino_types::puzzle::Puzzle;

fn get_n(sequence: &Vec<(String, String)>) -> usize {
    let l = sequence.len();
    let n_p = (-3.0 + (1.0 + 8.0 * (l as f64)).sqrt()) / 2.0;
    let n_d = (-2.0 + (8.0 * (l as f64)).sqrt()) / 2.0;
    let n = if (n_p - n_p.floor()).abs() == 0.0 {
        n_p.floor() as usize
    } else {
        n_d.floor() as usize
    };
    n
}

pub fn generate_puzzle(sequence: &Vec<(String, String)>) -> Puzzle {
    let mut puzzle: Vec<Option<(String, String)>> = sequence
        .clone()
        .into_iter()
        .map(|tile| Some(tile))
        .collect();
    let n = get_n(&sequence);
    let mut seed = thread_rng();
    let n_removals = seed.gen_range(1..(sequence.len() - n - 1));
    let mut removed = HashSet::new();

    for _ in 0..n_removals {
        let mut index = seed.gen_range(0..sequence.len());
        while removed.contains(&index) {
            index = seed.gen_range(0..sequence.len());
        }
        removed.insert(index);
        puzzle[index] = None;
    }

    puzzle.into_iter().map(|tile| 
        tile.map(|tile| (i32::from_str_radix(&tile.0, 10).unwrap() as usize, i32::from_str_radix(&tile.1, 10).unwrap() as usize))
    )
    .collect::<Vec<Option<(usize, usize)>>>()    
    .into()
}
