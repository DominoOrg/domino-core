use std::collections::HashSet;

use crate::{Arc, DominoError, Node};

use super::PuzzleData;

pub fn compute_hamiltonian_cycles(puzzle_data: &PuzzleData) -> Result<Vec<Vec<Node>>, DominoError> {
  let arcs: Vec<Arc> = puzzle_data.graph.adjacency.values().flatten().cloned().collect();
  let mut hamiltonian_paths: Vec<Vec<Node>> = vec![];
  let mut visited_arcs: HashSet<Arc> = HashSet::new();

  while visited_arcs.len() < arcs.len() {
    let hamiltonian_path = compute_hamiltonian_path(puzzle_data.tournament.clone(), visited_arcs.clone())?;
    hamiltonian_paths.push(hamiltonian_path);
  }

  if hamiltonian_paths.is_empty() {
    return Err(DominoError::NotValidPuzzle)
  }

  Ok(hamiltonian_paths)
}

fn compute_hamiltonian_path(tournament: Option<crate::Tournament>, visited_arcs: HashSet<Arc>) -> Result<Vec<Node>, DominoError> {
    todo!()
}
