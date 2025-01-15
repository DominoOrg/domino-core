use std::collections::{HashMap, HashSet, VecDeque};

use crate::types::{domino_types::error::DominoError, graph_types::{graph::Graph, node::Node, Arc}};

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

pub fn find_eulerian_cycle(graph: &Graph) -> Vec<Arc> {

    let mut circuit = Vec::new();
    let mut stack = VecDeque::new();
    let mut visited = HashMap::new();

    let start_node = graph.nodes.first().unwrap().clone();
    stack.push_back(start_node);

    while let Some(node) = stack.pop_back() {
        if let Some(neighbors) = graph.adjacency.get(&node) {
            if let Some(unvisited) = neighbors.iter().position(|arc| !visited.contains_key(&(node.clone(), arc.destination.clone()))) {
                let arc = neighbors[unvisited].clone();
                stack.push_back(node.clone());
                stack.push_back(arc.destination.clone());
                visited.insert((node, arc.destination.clone()), true);
                circuit.push(arc); // Add the arc to the circuit
            } else {
                if !circuit.is_empty() && circuit.last().unwrap().destination != node {
                    // If we've just completed a cycle, we might need to add an arc back to the start of this cycle
                    let last_arc = circuit.last().unwrap();
                    if let Some(back_arc) = graph.adjacency.get(&last_arc.destination).and_then(|arcs| arcs.iter().find(|a| a.destination == node)) {
                        circuit.push(back_arc.clone());
                    }
                }
            }
        }
    }

    // Reverse the circuit since we've been using a stack
    circuit.reverse();

    circuit
}

fn is_eulerian(graph: &Graph) -> bool {
    // Simplified check for Eulerian cycle:
    // - All non-zero degree vertices should have even degree for cycle
    // - Exactly 0 or 2 vertices with odd degree for path
    let mut odd_degree = 0;
    for node in &graph.nodes {
        if let Some(arcs) = graph.adjacency.get(&node) {
            let degree = arcs.len();
            if degree % 2 != 0 {
                odd_degree += 1;
            }
        }
    }
    odd_degree == 0 // For Eulerian cycle, change to `odd_degree <= 2` for Eulerian path
}