use std::collections::HashSet;

use crate::{Arc, DominoError, Node, Tournament};

use super::{hamiltonian::compute_hamiltonian_path_r, PuzzleData};

pub fn compute_hamiltonian_cycles(puzzle_data: &PuzzleData) -> Result<Vec<Vec<Node>>, DominoError> {
    let mut hamiltonian_paths: Vec<Vec<Node>> = vec![];
    let doubles: HashSet<Arc> = vec![0; puzzle_data.n + 1]
        .into_iter()
        .enumerate()
        .map(|(i, _)| (i as i32, i as i32).into())
        .collect();
    let mut visited_arcs: HashSet<Arc> = doubles.clone();
    let expected_cycles = (puzzle_data.n.saturating_sub(1) as f32 / 2.0).floor() as usize;

    while hamiltonian_paths.is_empty() || hamiltonian_paths.len() < expected_cycles {
        let tournament: Option<Tournament> = puzzle_data.tournament.clone();
        let result: Result<(Vec<Node>, HashSet<Arc>), DominoError> =
            compute_hamiltonian_path(&tournament.unwrap(), visited_arcs.clone());
        //println!("Result: {result:?}");
        if result.is_err() {
            hamiltonian_paths.clear();
            visited_arcs = doubles.clone();
            continue;
        }
        let (hamiltonian_path, updated_visited_arcs) = result.unwrap();
        visited_arcs = updated_visited_arcs;
        hamiltonian_paths.push(hamiltonian_path);
        //println!("hamiltonian_paths: {hamiltonian_paths:?}");
    }

    Ok(hamiltonian_paths)
}

fn compute_hamiltonian_path(
    tournament: &Tournament,
    visited_arcs: HashSet<Arc>,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
    let nodes: HashSet<Node> = tournament.nodes.clone().into_iter().collect();
    let visited_arcs: HashSet<Arc> = visited_arcs;

    // The total number of arcs inside the tournament
    let total_arcs = tournament
        .adjacency
        .values()
        .flatten()
        .collect::<Vec<&Arc>>()
        .len();

    // If there are no more nodes to extract an additional hamiltonian path return err
    if total_arcs - nodes.len() < visited_arcs.len() {
        return Err(DominoError::NotValidPuzzle);
    }

    compute_hamiltonian_path_r(tournament, visited_arcs)
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    use super::*;

    #[test]
    fn test_hamiltonians_n3() {
        let tournament = Tournament::new(vec![
            (0, 1).into(),
            (1, 1).into(),
            (1, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
            (0, 0).into(),
        ])
        .unwrap();
        let doubles: HashSet<Arc> = vec![0; 4]
            .into_iter()
            .enumerate()
            .map(|(i, _)| (i as i32, i as i32).into())
            .collect();
        let result = compute_hamiltonian_path(&tournament, doubles);
        println!("result: {result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_hamiltonians_n4() {
        let tournament = Tournament::new(vec![
            (0, 1).into(),
            (1, 1).into(),
            (1, 4).into(),
            (4, 4).into(),
            (4, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
            (0, 2).into(),
            (2, 1).into(),
            (1, 3).into(),
            (3, 4).into(),
            (4, 0).into(),
            (0, 0).into(),
        ])
        .unwrap();
        let puzzle_data = PuzzleData {
          puzzle: None,
          tournament: Some(tournament.clone()),
          graph: Graph::regular(4),
          n: 4,
          c: crate::ComplexityClass(3),
          solution: vec![],
        };
        let result = compute_hamiltonian_cycles(&puzzle_data);
        println!("result: {result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_hamiltonians_n5() {
        let n = 5;
        let tournament = Tournament::new(vec![
            (0, 1).into(),
            (1, 1).into(),
            (1, 4).into(),
            (4, 4).into(),
            (4, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
            (0, 2).into(),
            (2, 1).into(),
            (1, 3).into(),
            (3, 4).into(),
            (4, 0).into(),
            (0, 0).into(),
        ])
        .unwrap();
        let puzzle_data = PuzzleData {
          puzzle: None,
          tournament: Some(tournament.clone()),
          graph: Graph::regular(n),
          n,
          c: crate::ComplexityClass(3),
          solution: vec![],
        };
        let result = compute_hamiltonian_cycles(&puzzle_data);
        println!("result: {result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_hamiltonians_n6() {
        let n = 6;
        let tournament = Tournament::new(vec![
            (0, 1).into(),
            (1, 1).into(),
            (1, 4).into(),
            (4, 4).into(),
            (4, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
            (0, 2).into(),
            (2, 1).into(),
            (1, 3).into(),
            (3, 4).into(),
            (4, 0).into(),
            (0, 0).into(),
        ])
        .unwrap();
        let puzzle_data = PuzzleData {
          puzzle: None,
          tournament: Some(tournament.clone()),
          graph: Graph::regular(n),
          n,
          c: crate::ComplexityClass(3),
          solution: vec![],
        };
        let result = compute_hamiltonian_cycles(&puzzle_data);
        println!("result: {result:?}");
        assert!(result.is_ok());
    }
}
