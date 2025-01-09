use std::collections::HashMap;

use crate::graph_models::graph_types::{graph::GraphTrait, GraphNode};

// Define the RegularGraph struct
#[derive(Debug)]
pub struct RegularGraph {
    nodes: Vec<GraphNode>,                         // Set of nodes in the graph
    adjacency: HashMap<GraphNode, Vec<GraphNode>>, // Adjacency list
}

impl RegularGraph {
    // Constructor to create a new RegularGraph with N nodes
    pub fn new(n: usize) -> Self {
        let mut graph = RegularGraph {
            nodes: Vec::new(),
            adjacency: HashMap::new(),
        };

        // Insert nodes into the graph
        for i in 0..=n {
            graph.insert_node(i.to_string());
        }

        // Build the adjacency list based on the value of N
        if n % 2 == 0 {
            // Even N: connect each node to every other node
            graph.adjacency = graph
                .nodes()
                .into_iter()
                .map(|node| (node, graph.nodes()))
                .collect()
        } else {
            // Odd N: connect nodes based on specific conditions
            graph.adjacency = graph
                .nodes()
                .into_iter()
                .map(|node| {
                    let i = isize::from_str_radix(&node, 10).unwrap();
                    let neighbors = graph
                        .nodes()
                        .into_iter()
                        .filter(|node2| {
                            let j = isize::from_str_radix(&node2, 10).unwrap();
                            return i == j
                                || (j < i && j != i.saturating_sub(((n + 1) / 2) as isize))
                                || (i < j && i != j.saturating_sub(((n + 1) / 2) as isize));
                        })
                        .collect();
                    (node, neighbors)
                })
                .collect();
        }

        graph
    }
}

// Implement GraphTrait for RegularGraph
impl GraphTrait for RegularGraph {
    type Node = GraphNode;
    type Edge = GraphNode;

    fn nodes(&self) -> Vec<Self::Node> {
        self.nodes.clone()
    }

    fn adjacency(&self) -> HashMap<Self::Node, Vec<Self::Edge>> {
        self.adjacency.clone()
    }

    fn mut_nodes(&mut self) -> &mut Vec<Self::Node> {
        &mut self.nodes
    }

    fn mut_adjacency(&mut self) -> &mut HashMap<Self::Node, Vec<Self::Edge>> {
        &mut self.adjacency
    }
}
