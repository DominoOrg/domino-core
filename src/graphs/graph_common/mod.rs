pub use crate::types::{Graph, Node};
use next_node::NextNodeBuilder;
use rand::seq::SliceRandom;
use std::collections::HashSet;

mod next_node;

pub fn find_eulerian_cycle<'a>(graph: &'a Graph) -> impl Fn(bool) -> Vec<Node> + use<'a> {
    move |random: bool| -> Vec<Node> { hierholzer(graph)(random) }
}

fn hierholzer<'a>(graph: &'a Graph) -> impl Fn(bool) -> Vec<Node> + use<'a> {
    move |random: bool| {
        let mut circuit: Vec<Node> = Vec::new();
        let mut visited: HashSet<Arc> = HashSet::new();
        let mut stack: Vec<Node> = vec![first_node(graph)(random)];
        while !stack.is_empty() {
            if let Some(current_vertex) = stack.pop() {
                // Choice of the next node
                let unvisited_edge_index = NextNodeBuilder::new(graph)
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
                        visited.insert(Arc(current_vertex.clone(), next_vertex.clone()));
                        visited.insert(Arc(next_vertex.clone(), current_vertex.clone()));
                    } else {
                        visited.insert(Arc(next_vertex.clone(), current_vertex.clone()));
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

#[derive(Debug, Clone, Eq, Hash)]
struct Arc(Node, Node);
impl PartialEq for Arc {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}
fn first_node<'a>(graph: &'a Graph) -> impl Fn(bool) -> Node + 'a {
    move |random: bool| {
        let mut seed = rand::thread_rng();
        if random {
            graph.nodes.choose(&mut seed).unwrap().clone()
        } else {
            graph.nodes.first().unwrap().clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{graphs::find_eulerian_cycle, types::Graph};

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
