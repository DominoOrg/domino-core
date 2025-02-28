use std::vec;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reinsert::reinsert_tile_and_check;
use crate::{types::Graph, Puzzle, Solution, Tile};
use crate::graphs::find_eulerian_cycle;

mod reinsert;

pub fn generate_valid_puzzle(n: usize) ->  Box<dyn Fn(usize) -> Box<dyn Fn(bool) -> Puzzle>> {
  Box::new(move |c| {
    Box::new(move |random| {
      let graph = Graph::regular(n);
      // Since the eulerian path is represented as the sequence of the nodes traversed
      // The solution is built groupping 2 by 2 each node traversed and
      // building a tile from each couple
      let solution: Solution = find_eulerian_cycle(&graph)(random)
        .windows(2)
        .map(|arc| Tile(arc[0].clone().try_into().unwrap(), arc[1].clone().try_into().unwrap()))
        .collect();
      let puzzle = generate_puzzle(solution)(c)(random);
      puzzle
    })
  })
}

fn generate_puzzle(solution: Solution) -> Box<dyn Fn(usize) -> Box<dyn Fn(bool) -> Puzzle>> {
  Box::new(move |c: usize| {
    Box::new({
    let value = solution.clone();
    move |random: bool| -> Puzzle {
      let starting_puzzle: Vec<Option<Tile>> = vec![None; value.clone().len()];
      let removed_tiles: Vec<Tile> = if random {
        let mut removed =  value.clone();
        removed.shuffle(&mut thread_rng());
        removed
      } else { value.clone() };
      let puzzle: Puzzle = reinsert_tile_and_check(starting_puzzle, value.clone(), removed_tiles, c, random);
      puzzle
    }
    })
  })
}
