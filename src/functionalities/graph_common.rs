use std::collections::{HashMap, HashSet};
use rand::seq::{IteratorRandom, SliceRandom};

use crate::types::{domino_types::error::DominoError, graph_types::{graph::Graph, node::Node}};

pub fn lexicographic_2coloring(graph: &Graph, ordering: &Vec<Node>) -> Result<HashMap<Node, bool>, DominoError> {
    let mut colors = HashMap::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    for node in ordering {
        if !visited.contains(node) {
            stack.push((node.clone(), true)); // node and color (true for one color, false for the other)
            while let Some((current_node, color)) = stack.pop() {
                if visited.contains(&current_node) {
                    if *colors.get(&current_node).unwrap() != color {
                        return Err(DominoError::InvalidPuzzle("Graph is not bipartite".to_string())); // Graph is not bipartite, an odd cycle detected
                    }
                } else {
                    visited.insert(current_node.clone());
                    colors.insert(current_node.clone(), color);

                    if let Some(arcs) = graph.adjacency.get(&current_node) {
                        for arc in arcs {
                            let next_color = !color; // Flip the color
                            if visited.contains(&arc.destination) {
                                if *colors.get(&arc.destination).unwrap() != next_color {
                                    return Err(DominoError::InvalidPuzzle("Graph is not bipartite".to_string())); // Found miscolored edge
                                }
                            } else {
                                stack.push((arc.destination.clone(), next_color));
                            }
                        }
                    }
                }
            }
        }
    }

    // Construct the coloring result if the graph is bipartite
    let coloring = colors.into_iter().map(|(node, color)| (node, color)).collect();
    Ok(coloring)
}

pub fn perfect_elimination_order(graph: &Graph) -> Vec<Node> {
    let mut sigma = vec![graph.nodes.iter().cloned().collect::<HashSet<Node>>()];
    let mut ordering = Vec::new();

    while !sigma.is_empty() {
        // Find and remove a vertex from the first set in sigma
        let first_set = sigma.first_mut().unwrap();
        if let Some(v) = first_set.iter().next().cloned() {
            first_set.remove(&v);
            ordering.push(v.clone());

            // Clean up empty sets
            if first_set.is_empty() {
                sigma.remove(0);
            }

            // Process each arc from v to w where w is still in sigma
            if let Some(arcs) = graph.adjacency.get(&v) {
                for arc in arcs {
                    let w = &arc.destination;
                    let mut w_set_index = None;
                    let mut w_set = None;
                    
                    // Find the set containing w
                    for (i, set) in sigma.clone().into_iter().enumerate() {
                        if set.contains(w) {
                            w_set_index = Some(i);
                            w_set = Some(set);
                            break;
                        }
                    }
                    let is_empty = sigma[w_set_index.unwrap() - 1].is_empty();

                    if let (Some(w_set_index), Some(mut w_set)) = (w_set_index, w_set) {
                        // If w_set hasn't been replaced yet for this v, create or find T
                        let t_index = if w_set_index == 0 || w_set_index > 0 && is_empty {
                            let new_set = HashSet::new();
                            sigma.insert(w_set_index, new_set);
                            w_set_index
                        } else {
                            w_set_index - 1
                        };

                        // Move w to T
                        let mut t = sigma[t_index].clone(); 
                        w_set.remove(w);
                        t.insert(w.clone());

                        // Clean up if w_set becomes empty
                        if w_set.is_empty() {
                            sigma.remove(w_set_index);
                        }
                    }
                }
            }
        }
    }

    ordering
}

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
            edges_iterator
            .position(|arc| 
                !visited.contains(&Arc(current_vertex.clone(), arc.destination.clone())) &&
                !visited.contains(&Arc(arc.destination.clone(), current_vertex.clone()))
            )
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
                visited.insert(Arc(current_vertex.clone(), next_vertex.clone()));
                visited.insert(Arc(next_vertex.clone(), current_vertex.clone()));
                stack.push(next_vertex);
            } else {
                circuit.push(current_vertex.clone());
            }
        }
    }
    circuit.reverse(); // Reverse to get the correct order
    circuit
}