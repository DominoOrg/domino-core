use std::collections::{HashMap, VecDeque};
use graph::Graph;

use super::domino_types::{DominoError, SequenceScraper};
use super::graph_types::node::Node;

pub(crate) mod node;
pub(crate) mod graph;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum Orientation {
    #[default]
    Negative,
    Positive
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct Arc {
    pub source: Node,
    pub destination: Node,
    pub orientation: Option<Orientation>,
    pub position: Option<usize>
}

pub(crate) struct BipartiteChecker;

impl BipartiteChecker {
    pub fn is_bipartite(graph: &Graph) -> bool {
        todo!();
        true
    }
}

pub(crate) struct ColoringFinder;

impl ColoringFinder {
    pub fn lexicographic_2coloring(graph: &Graph, ordering: &Vec<Arc>) -> Result<HashMap<Node, bool>, DominoError> {
        todo!()
    }
}

pub(crate) struct OrderingFinder;

impl OrderingFinder {
    pub fn perfect_elimination_order(graph: &Graph) -> Result<Vec<Arc>, DominoError> {
        todo!();
        Err(DominoError::UnsolvableGraph("Unsolvable graph".to_string()))
    }
}

pub(crate) struct EulerianCycleFinder;

impl EulerianCycleFinder {
    pub fn find_cycle(graph: &Graph) -> Vec<Arc> {

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
}