use std::collections::{HashMap, HashSet};

use crate::Tile;

use super::{Node, Arc};

/// A tournament graph where each node has an equal number of incoming and outgoing edges.
#[derive(Debug, Default, Clone)]
pub struct Tournament {
    /// The set of nodes in the tournament.
    pub nodes: HashSet<Node>,
    /// The adjacency list representing outgoing arcs for each node.
    pub adjacency: HashMap<Node, Vec<Arc>>,
}

impl Tournament {
    /// Creates a new tournament from a list of directed edges (arcs).
    ///
    /// # Arguments
    /// * `arcs` - A vector of tuples representing directed edges, where the first element
    ///   is the source node and the second is the destination node.
    ///
    /// # Returns
    /// * `Ok(Tournament)` if the input forms a valid tournament graph.
    /// * `Err(String)` if the input does not satisfy the tournament properties.
    pub fn new(arcs: Vec<Tile>) -> Result<Self, String> {
        let mut nodes: HashSet<Node> = HashSet::new();
        let mut adjacency: HashMap<Node, Vec<Arc>> = HashMap::new();
        let mut in_degrees: HashMap<Node, usize> = HashMap::new();
        let mut out_degrees: HashMap<Node, usize> = HashMap::new();

        for (index, tile) in arcs.iter().enumerate() {
            let source = tile.0;
            let destination = tile.1;
            nodes.insert(source);
            nodes.insert(destination);

            let arc = Arc {
                source,
                destination,
                position: Some(index),
            };
            adjacency.entry(source).or_default().push(arc.clone());

            *out_degrees.entry(source).or_insert(0) += 1;
            *in_degrees.entry(destination).or_insert(0) += 1;
        }

        // Ensure each node has an equal number of incoming and outgoing edges
        let n = nodes.len();
        if n > 1 {
            for &node in &nodes {
                let in_deg = *in_degrees.get(&node).unwrap_or(&0);
                let out_deg = *out_degrees.get(&node).unwrap_or(&0);

                if in_deg != out_deg || in_deg + out_deg != 2 * (n - 1) {
                    return Err(format!(
                        "Invalid tournament: Node {} has in-degree {} and out-degree {}",
                        node, in_deg, out_deg
                    ));
                }
            }
        }

        Ok(Tournament { nodes, adjacency })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tournament_invalid_unbalanced_degrees() {
        // This test checks a case where the tournament is invalid because not all nodes
        // have the required equal in-degree and out-degree.
        let arcs = vec![(0, 1).into(), (1, 2).into()];
        let result = Tournament::new(arcs);
        assert!(result.is_err());
    }

    #[test]
    fn test_tournament_invalid_self_loop() {
        // This test ensures that a tournament with an isolated self-loop is rejected,
        // as self-loops alone do not satisfy the tournament conditions.
        let arcs = vec![(0, 0).into(), (0, 1).into(), (1, 0).into()];
        let result = Tournament::new(arcs);
        assert!(result.is_err());
    }

    #[test]
    fn test_tournament_valid_with_self_loops() {
        // This test ensures that a tournament where every node has a self-loop in addition
        // to other required edges is still considered valid.
        // The self-loops do not interfere with the tournament properties.
        let arcs = vec![(0, 0).into(), (0, 1).into(), (1, 1).into(), (1, 2).into(), (2, 2).into(), (2, 0).into()];
        let result = Tournament::new(arcs);
        assert!(result.is_ok());
    }
}
