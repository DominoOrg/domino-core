use std::collections::HashSet;
use rand::seq::{IteratorRandom, SliceRandom};

use crate::types::{Graph, Node};

pub fn find_eulerian_cycle(graph: &Graph, random: bool) -> Vec<Node> {
    #[derive(Debug, Clone, Eq, Hash)]
    struct Arc(Node, Node);
    impl PartialEq for Arc {
        fn eq(&self, other: &Self) -> bool {
            (self.0 == other.0 && self.1 == other.1) ||
            (self.0 == other.1 && self.1 == other.0)
        }
    }
    fn first_node(graph: &Graph, random: bool) -> Node {
        let mut seed = rand::thread_rng();
        if random {
            graph.nodes.choose(&mut seed).unwrap().clone()
        } else {
            graph.nodes.first().unwrap().clone()
        }
    }
    fn next_node(graph: &Graph, random: bool, visited: HashSet<Arc>, current_vertex: Node) -> Option<usize> {
        let mut seed = rand::thread_rng();
        let mut edges_iterator = graph.adjacency
        .get(&current_vertex.clone()).unwrap()
        .iter();
        if random {
            edges_iterator
            .enumerate()
            .filter(|(_, arc)|
                !visited.contains(&Arc(current_vertex.clone(), arc.destination.clone())) &&
                !visited.contains(&Arc(arc.destination.clone(), current_vertex.clone()))
            )
            .choose(&mut seed)
            .map(|(index, _)| index)
        } else {
            let double_position = if !visited.contains(&Arc(current_vertex, current_vertex)) {
                graph.adjacency
                .get(&current_vertex.clone()).unwrap()
                .iter()
                .position(|arc| {
                    return arc.destination == current_vertex;
                })
            } else {
                None
            };
            if double_position.is_none() {
                let position = edges_iterator
                .position(|arc| {
                    return current_vertex != arc.destination &&
                    !visited.contains(&Arc(current_vertex.clone(), arc.destination.clone())) &&
                    !visited.contains(&Arc(arc.destination.clone(), current_vertex.clone()));
                });
                return position;
            }
            double_position
        }
    }
    let mut circuit: Vec<Node> = Vec::new();
    let mut visited: HashSet<Arc> = HashSet::new();
    let mut stack: Vec<Node> = vec![
        first_node(graph, random)
    ];

    while !stack.is_empty() {
        if let Some(current_vertex) = stack.pop() {
            // Choice of the next node
            let unvisited_edge_index = next_node(graph, random, visited.clone(), current_vertex.clone());

            // Process unvisided edges of the next node 
            if let Some(unvisited_index) = unvisited_edge_index {
                stack.push(current_vertex.clone());
                let next_vertex = graph.adjacency
                .get(&current_vertex).unwrap()
                .get(unvisited_index)
                .unwrap().destination.clone();
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