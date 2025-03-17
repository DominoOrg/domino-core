use std::collections::HashSet;

use crate::{Arc, DominoError, Node, Tournament};

use super::{hamiltonian::compute_hamiltonian_path_r, PuzzleData};

pub fn compute_hamiltonian_cycles(puzzle_data: &PuzzleData) -> Result<Vec<Vec<Node>>, DominoError> {
    let arcs: Vec<Arc> = puzzle_data
        .graph
        .adjacency
        .values()
        .flatten()
        .cloned()
        .collect();
    let mut hamiltonian_paths: Vec<Vec<Node>> = vec![];
    let mut visited_arcs: HashSet<Arc> = HashSet::new();

    while hamiltonian_paths.len() < (puzzle_data.n.saturating_sub(1) as f32 /2.0).floor() as usize {
        let tournament: Option<Tournament> = puzzle_data.tournament.clone();
        let result: (Vec<Node>, HashSet<Arc>) =
            compute_hamiltonian_path(&tournament.unwrap(), visited_arcs.clone())?;
        // println!("Result: {result:?}");
        let hamiltonian_path = result.0;
        visited_arcs = result.1;
        hamiltonian_paths.push(hamiltonian_path);
    }

    if hamiltonian_paths.is_empty() {
        return Err(DominoError::NotValidPuzzle);
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
    if total_arcs - nodes.len() <= visited_arcs.len() {
        return Err(DominoError::NotValidPuzzle);
    }

    compute_hamiltonian_path_r(tournament, visited_arcs)
}

#[cfg(test)]
mod tests {
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
        let result = compute_hamiltonian_path(&tournament, HashSet::new());
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
        let mut result = compute_hamiltonian_path(&tournament, HashSet::new());
        println!("result: {result:?}");
        assert!(result.is_ok());
        let visited_arcs = result.unwrap().1;

        result = compute_hamiltonian_path(&tournament, visited_arcs);
        println!("result: {result:?}");
        assert!(result.is_ok());
    }
}
