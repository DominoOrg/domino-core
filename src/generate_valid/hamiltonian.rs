use crate::{Arc, DominoError, Node, Tournament};
use rand::{seq::IteratorRandom, thread_rng};
use std::collections::HashSet;

pub fn compute_hamiltonian_path_r(
    tournament: &Tournament,
    visited_arcs: HashSet<Arc>,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
    //println!("Entering compute_hamiltonian_path_r");
    let nodes = extract_nodes(&tournament);
    let start_node = select_start_node(tournament, &visited_arcs)?;
    let initial_path = vec![start_node];
    compute_hamiltonian_path_recursive(tournament, nodes, initial_path, visited_arcs, start_node)
}

fn select_next_node(
    tournament: &Tournament,
    current_node: &Node,
    current_path: &Vec<Node>,
    visited_arcs: &HashSet<Arc>,
) -> Option<Node> {
    //println!("Entering select_next_node, visited_arcs: {visited_arcs:?}");
    let mut rng = thread_rng();
    let empty_vec = vec![];
    let available_arcs: Vec<&Arc> = tournament
        .adjacency
        .get(current_node)
        .unwrap_or(&empty_vec)
        .iter()
        .filter(|arc| {
            !current_path.contains(&arc.destination)
                && !visited_arcs
                    .iter()
                    .any(|visited| *visited == **arc)
        })
        .collect::<Vec<&Arc>>();
    //println!("Available arcs: {available_arcs:?}");
    let next_node = available_arcs
        .into_iter()
        .choose(&mut rng)
        .map(|arc| arc.destination);
    //println!("Selected next node: {:?}", next_node);
    next_node
}

fn process_next_node(
    tournament: &Tournament,
    nodes: HashSet<Node>,
    current_path: Vec<Node>,
    visited_arcs: HashSet<Arc>,
    current_node: Node,
    next_node: Node,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
    //println!("Entering process_next_node");
    let next_arc = Arc::from((current_node, next_node));
    match is_valid_arc(&tournament, &current_node, &next_arc, &visited_arcs) {
        true => proceed_to_next_node(
            tournament,
            nodes,
            current_path,
            visited_arcs,
            next_node,
            next_arc,
        ),
        false => attempt_backtrack(tournament, nodes, current_path, visited_arcs, current_node),
    }
}

fn is_valid_arc(
    tournament: &Tournament,
    current_node: &Node,
    next_arc: &Arc,
    visited_arcs: &HashSet<Arc>,
) -> bool {
    //println!("Entering is_valid_arc");
    tournament
        .adjacency
        .get(current_node)
        .unwrap_or(&Vec::new())
        .contains(next_arc)
        && !visited_arcs.contains(next_arc)
}

fn proceed_to_next_node(
    tournament: &Tournament,
    nodes: HashSet<Node>,
    mut current_path: Vec<Node>,
    mut visited_arcs: HashSet<Arc>,
    next_node: Node,
    next_arc: Arc,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
    //println!("Entering proceed_to_next_node");
    current_path.push(next_node);
    visited_arcs.insert(next_arc);
    compute_hamiltonian_path_recursive(tournament, nodes, current_path, visited_arcs, next_node)
}

fn attempt_backtrack(
    tournament: &Tournament,
    nodes: HashSet<Node>,
    current_path: Vec<Node>,
    visited_arcs: HashSet<Arc>,
    current_node: Node,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
    //println!("Entering attempt_backtrack");
    match previous_node(&current_path, current_node) {
        Some(previous) => compute_hamiltonian_path_recursive(
            tournament,
            nodes,
            remove_last_node(&current_path, current_node),
            visited_arcs,
            previous,
        ),
        None => Err(DominoError::InvalidLength),
    }
}

fn previous_node(current_path: &Vec<Node>, current_node: Node) -> Option<Node> {
    //println!("Entering previous_node");
    let previous = current_path
        .iter()
        .position(|&node| node == current_node)
        .filter(|&index| index > 0)
        .map(|index| current_path[index - 1]);
    //println!("Previous node: {:?}", previous);
    previous
}

fn remove_last_node(current_path: &Vec<Node>, node_to_remove: Node) -> Vec<Node> {
    //println!("Entering remove_last_node");
    current_path
        .iter()
        .cloned()
        .filter(|&node| node != node_to_remove)
        .collect()
}

fn extract_nodes(tournament: &Tournament) -> HashSet<Node> {
    //println!("Entering extract_nodes");
    tournament.adjacency.keys().cloned().collect()
}

fn select_start_node(
    tournament: &Tournament,
    visited_arcs: &HashSet<Arc>,
) -> Result<Node, DominoError> {
    //println!("Entering select_start_node");
    let mut rng = thread_rng();
    let available_arcs: Vec<(&i32, &Vec<Arc>)> = tournament
        .adjacency
        .iter()
        .filter(|(_node, arcs)| {
            arcs.iter()
                .any(|arc| arc.source != arc.destination && !visited_arcs.contains(arc))
        })
        .collect();
    // //println!("Available arcs: {available_arcs:?}");
    let start_node = available_arcs
        .into_iter()
        .map(|(node, _)| node)
        .choose(&mut rng)
        .copied()
        .ok_or(DominoError::NotValidPuzzle);
    //println!("Selected start node: {:?}", start_node);
    start_node
}

fn compute_hamiltonian_path_recursive(
    tournament: &Tournament,
    nodes: HashSet<Node>,
    current_path: Vec<Node>,
    visited_arcs: HashSet<Arc>,
    current_node: Node,
) -> Result<(Vec<Node>, HashSet<Arc>), DominoError> {
    //println!("Entering compute_hamiltonian_path_recursive");
    if path_is_complete(&current_path, &nodes) {
        Ok((current_path, visited_arcs))
    } else {
        match select_next_node(tournament, &current_node, &current_path, &visited_arcs) {
            Some(next_node) => process_next_node(
                tournament,
                nodes,
                current_path,
                visited_arcs,
                current_node,
                next_node,
            ),
            None => process_next_node(
              tournament,
              nodes,
              current_path,
              visited_arcs,
              current_node,
              current_node,
          ),
        }
    }
}

fn path_is_complete(current_path: &Vec<Node>, nodes: &HashSet<Node>) -> bool {
    //println!("Entering path_is_complete");
    let complete = current_path.len() == nodes.len();
    //println!("Is path complete? {:?}, current_path: {current_path:?}", complete);
    complete
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tournament;

    #[test]
    fn test_hamiltonian_valid_path() {
        let tournament = Tournament::new(vec![
            (0, 0).into(),
            (0, 1).into(),
            (1, 1).into(),
            (1, 2).into(),
            (2, 2).into(),
            (2, 0).into(),
        ])
        .unwrap();
        let result = compute_hamiltonian_path_r(&tournament, HashSet::new());
        assert!(result.is_ok());
        let path = result.unwrap().0;
        //println!("Computed path: {:?}", path);
        assert!(path == vec![0, 1, 2] || path == vec![1, 2, 0] || path == vec![2, 0, 1]);
    }
}
