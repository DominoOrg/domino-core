//! This module implements Hierholzer's algorithm for finding an Eulerian cycle in a graph.
//!
//! The `hierholzer` function generates an Eulerian circuit by traversing all edges exactly once,
//! maintaining a stack-based approach to reconstruct the circuit.

use super::{first_node::first_node, next_node::NextNodeBuilder};
use crate::utils::graphs::{Arc, Graph, Node};
use std::collections::HashSet;

/// Generates a function that finds an Eulerian cycle using Hierholzer's algorithm.
///
/// The function returned by `hierholzer(graph)` takes a boolean flag (`random`) to determine
/// whether the next node selection is randomized. It constructs an Eulerian circuit using a
/// stack-based depth-first traversal.
///
/// # Arguments
///
/// * `graph` - A reference to the `Graph` structure representing the input graph.
///
/// # Returns
///
/// A closure `(bool) -> Vec<Node>` that computes an Eulerian cycle when called.
pub fn hierholzer<'a>(graph: &'a Graph) -> impl Fn(bool) -> Vec<Node> + 'a {
    move |random: bool| {
        let mut circuit: Vec<Node> = Vec::new();
        let mut visited: HashSet<Arc> = HashSet::new();
        let mut stack: Vec<Node> = vec![first_node(graph)(random)];

        while let Some(current_vertex) = stack.pop() {
            // Choose the next node using NextNodeBuilder
            let unvisited_edge_index: Option<usize> = NextNodeBuilder::new(graph)
                .with_random(random)
                .with_visited(visited.clone())
                .with_current_vertex(current_vertex.clone())
                .build();

            if let Some(unvisited_index) = unvisited_edge_index {
                stack.push(current_vertex.clone());

                // Retrieve the next vertex from the adjacency list
                let next_vertex = graph
                    .adjacency
                    .get(&current_vertex)
                    .unwrap()
                    .get(unvisited_index)
                    .unwrap()
                    .destination
                    .clone();

                // Mark edges as visited (handling both directions)
                if current_vertex != next_vertex {
                    visited.insert(Arc::from((current_vertex.clone(), next_vertex.clone())));
                    visited.insert(Arc::from((next_vertex.clone(), current_vertex.clone())));
                } else {
                    visited.insert(Arc::from((next_vertex.clone(), current_vertex.clone())));
                }

                stack.push(next_vertex);
            } else {
                circuit.push(current_vertex.clone());
            }
        }

        circuit.reverse(); // Reverse to get the correct order
        circuit
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use crate::utils::graphs::find_eulerian_cycle;

    /// Tests the `find_eulerian_cycle` function for graphs of different sizes.
    ///
    /// The expected length of the Eulerian cycle is computed based on whether `n` is even or odd.
    #[test]
    fn test_find_eulerian_cycle() {
        (3..=6).for_each(|n| {
            let graph = Graph::regular(n);
            let eulerian_cycle = find_eulerian_cycle(&graph)(false);
            let expected_len = if n % 2 == 0 {
                (n + 1) * (n + 2) / 2
            } else {
                (n + 1) * (n + 1) / 2
            };
            assert_eq!(eulerian_cycle.len(), expected_len + 1);
        });
    }
}
