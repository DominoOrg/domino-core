use std::collections::HashSet;
use rand::{seq::IteratorRandom, thread_rng};
use crate::{Arc, DominoError, Node, Tournament};

/// Computes a Hamiltonian path in a tournament graph.
///
/// # Arguments
/// * `tournament` - The tournament graph.
///
/// # Returns
/// * `Ok((Vec<Node>, HashSet<Arc>))` if a valid path is found.
/// * `Err(DominoError::NotValidPuzzle)` if no path exists.
pub fn compute_hamiltonian_path_r(
  tournament: Tournament
) -> Result<
  (Vec<Node>, HashSet<Arc>),
  DominoError
> {
  let nodes = extract_nodes(&tournament);
  let start_node = select_start_node(&nodes)?;
  let initial_path = vec![start_node];
  let initial_arcs = HashSet::new();
  compute_hamiltonian_path_recursive(
      tournament,
      nodes,
      initial_path,
      initial_arcs,
      start_node
  )
}

/// Selects the next node to visit that is not already in the current path.
fn select_next_node(current_path: &Vec<Node>, nodes: &HashSet<Node>) -> Option<Node> {
    let mut rng = thread_rng();
    nodes
        .iter()
        .filter(|node| !current_path.contains(node))
        .choose(&mut rng)
        .copied()
}

/// Processes the selected next node by verifying arc validity and updating the path.
fn process_next_node(
    tournament: Tournament,
    nodes: HashSet<Node>,
    current_path: Vec<Node>,
    visited_arcs: HashSet<Arc>,
    current_node: Node,
    next_node: Node,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
    let next_arc = Arc::from((current_node, next_node));
    match is_valid_arc(&tournament, &current_node, &next_arc) {
        true => proceed_to_next_node(
            tournament,
            nodes,
            current_path,
            visited_arcs,
            next_node,
            next_arc,
        ),
        false => attempt_backtrack(
          tournament,
          nodes,
          current_path,
          visited_arcs,
          current_node),
    }
}

/// Checks if an arc exists in the tournament graph.
fn is_valid_arc(tournament: &Tournament, current_node: &Node, next_arc: &Arc) -> bool {
    tournament
        .adjacency
        .get(current_node)
        .unwrap_or(&Vec::new())
        .contains(next_arc)
}

/// Moves forward by adding a valid arc and recursively computing the next step.
fn proceed_to_next_node(
    tournament: Tournament,
    nodes: HashSet<Node>,
    mut current_path: Vec<Node>,
    mut visited_arcs: HashSet<Arc>,
    next_node: Node,
    next_arc: Arc,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
    current_path.push(next_node);
    visited_arcs.insert(next_arc);
    compute_hamiltonian_path_recursive(
      tournament,
      nodes,
      current_path,
      visited_arcs,
      next_node)
}

/// Attempts to backtrack when a dead-end is encountered.
///
/// If there is a previous node in the path, it removes the last occurrence
/// of the current node and continues the search from the previous node.
/// If no previous node exists, it attempts to retry the recursive computation
/// with the current node instead of returning an error.
fn attempt_backtrack(
  tournament: Tournament,
  nodes: HashSet<Node>,
  current_path: Vec<Node>,
  visited_arcs: HashSet<Arc>,
  current_node: Node,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
  match previous_node(&current_path, current_node) {
      Some(previous) => compute_hamiltonian_path_recursive(
          tournament,
          nodes,
          remove_last_node(&current_path, current_node),
          visited_arcs,
          previous,
      ),
      None => compute_hamiltonian_path_recursive(
          tournament,
          nodes,
          remove_last_node(&current_path, current_node),
          visited_arcs,
          current_node,
      ),
  }
}

/// Finds the previous node in the path.
fn previous_node(current_path: &Vec<Node>, current_node: Node) -> Option<Node> {
    current_path
        .iter()
        .position(|&node| node == current_node)
        .filter(|&index| index > 0)
        .map(|index| current_path[index - 1])
}

/// Returns a new path with the last occurrence of the given node removed.
fn remove_last_node(current_path: &Vec<Node>, node_to_remove: Node) -> Vec<Node> {
    current_path
        .iter()
        .cloned()
        .filter(|&node| node != node_to_remove)
        .collect()
}

/// Extracts all unique nodes from the tournament graph.
fn extract_nodes(
    tournament: &Tournament
) -> HashSet<Node> {
    tournament.adjacency.keys().cloned().collect()
}

/// Selects an arbitrary starting node from the set of nodes.
fn select_start_node(
    nodes: &HashSet<Node>
) -> Result<Node, DominoError> {
    nodes.iter().next().copied().ok_or(DominoError::NotValidPuzzle)
}

/// Recursively computes a Hamiltonian path by selecting valid arcs.
fn compute_hamiltonian_path_recursive(
    tournament: Tournament,
    nodes: HashSet<Node>,
    current_path: Vec<Node>,
    visited_arcs: HashSet<Arc>,
    current_node: Node
) -> Result<
    (Vec<Node>, HashSet<Arc>),
    DominoError
> {
    if path_is_complete(&current_path, &nodes) {
        Ok((current_path, visited_arcs))
    } else {
        match select_next_node(&current_path, &nodes) {
            Some(next_node) => process_next_node(
                tournament,
                nodes,
                current_path,
                visited_arcs,
                current_node,
                next_node
            ),
            None => Err(DominoError::NotValidPuzzle),
        }
    }
}

/// Checks if the Hamiltonian path is complete.
fn path_is_complete(
    current_path: &Vec<Node>,
    nodes: &HashSet<Node>
) -> bool {
    current_path.len() == nodes.len()
}


/// Unit tests for the compute_hamiltonian_path_r function.
#[cfg(test)]
mod tests {
    use crate::Tournament;

    use super::*;

    #[test]
    fn test_hamiltonian_valid_path() {
        let tournament = Tournament::new(
          vec![(0,0).into(), (0,1).into(), (1,1).into(), (1,2).into(), (2,2).into(), (2,0).into()]
        ).unwrap();
        let result = compute_hamiltonian_path_r(tournament);
        assert!(result.is_ok());
        let path = result.unwrap().0;
        println!("path: {:?}", path);
        assert!(path == vec![0,1,2] || path == vec![1,2,0] || path == vec![2,0,1]);
    }

}
