use super::{first_node::first_node, next_node::NextNodeBuilder};
use crate::utils::graphs::{Arc, Graph, Node};
use std::collections::HashSet;

pub fn hierholzer<'a>(graph: &'a Graph) -> impl Fn(bool) -> Vec<Node> + use<'a> {
    move |random: bool| {
        let mut circuit: Vec<Node> = Vec::new();
        let mut visited: HashSet<Arc> = HashSet::new();
        let mut stack: Vec<Node> = vec![first_node(graph)(random)];
        while !stack.is_empty() {
            if let Some(current_vertex) = stack.pop() {
                // Choice of the next node
                let unvisited_edge_index: Option<usize> = NextNodeBuilder::new(graph)
                    .with_random(random)
                    .with_visited(visited.clone())
                    .with_current_vertex(current_vertex.clone())
                    .build();

                // Process unvisided edges of the next node
                if let Some(unvisited_index) = unvisited_edge_index {
                    stack.push(current_vertex.clone());
                    let next_vertex = graph
                        .adjacency
                        .get(&current_vertex)
                        .unwrap()
                        .get(unvisited_index)
                        .unwrap()
                        .destination
                        .clone();
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
        }
        circuit.reverse(); // Reverse to get the correct order
        circuit
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use crate::utils::graphs::find_eulerian_cycle;

    #[test]
    fn test_find_eulerian_cycle() {
        (3..=6).into_iter().for_each(|n| {
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
