use std::collections::{HashMap, VecDeque};


type Orientation = i32;

#[derive(Debug, Default, Clone, Copy)]
struct Direction(Orientation, i32);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Arc {
    pub source: i32,
    pub destination: i32,
    direction: Option<Direction>
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Graph {
    nodes: Vec<i32>,
    adjacency: HashMap<i32, Vec<Arc>>
}

impl Graph {
    pub fn regular(n: usize) -> Self {
        Graph {
            nodes: (0..n as i32).collect::<Vec<i32>>(),
            adjacency: (0..n as i32)
                .map(|source_node| {
                    let edges = (0..n).map(|destination_node| {
                        Arc {
                            source: source_node,
                            destination: destination_node as i32,
                            direction: None
                        }
                    }).collect::<Vec<Arc>>();
                    (source_node, edges)
                })
                .collect()
        }
    }
}

pub struct EulerianCycleFinder;

impl EulerianCycleFinder {
    pub fn find_cycle(graph: &Graph) -> Vec<Arc> {

        let mut circuit = Vec::new();
        let mut stack = VecDeque::new();
        let mut visited = HashMap::new();

        let start_node = *graph.nodes.first().unwrap();
        stack.push_back(start_node);

        while let Some(node) = stack.pop_back() {
            if let Some(neighbors) = graph.adjacency.get(&node) {
                if let Some(unvisited) = neighbors.iter().position(|arc| !visited.contains_key(&(node, arc.destination))) {
                    let arc = neighbors[unvisited];
                    stack.push_back(node);
                    stack.push_back(arc.destination);
                    visited.insert((node, arc.destination), true);
                    circuit.push(arc); // Add the arc to the circuit
                } else {
                    if !circuit.is_empty() && circuit.last().unwrap().destination != node {
                        // If we've just completed a cycle, we might need to add an arc back to the start of this cycle
                        let last_arc = circuit.last().unwrap();
                        if let Some(back_arc) = graph.adjacency.get(&last_arc.destination).and_then(|arcs| arcs.iter().find(|a| a.destination == node)) {
                            circuit.push(*back_arc);
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
}