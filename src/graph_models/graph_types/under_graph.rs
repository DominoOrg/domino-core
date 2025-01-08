use std::collections::HashMap;

use crate::graph_models::graph_types::{
    graph::GraphTrait, pog_graph::PogGraph, GraphNode, Orientation,
};

// UnderlyingGraph struct implementing GraphTrait
pub struct UnderlyingGraph {
    nodes: Vec<GraphNode>,
    adjacency: HashMap<GraphNode, Vec<GraphNode>>,
}

impl UnderlyingGraph {
    pub fn from(pog_graph: &PogGraph) -> Self {
        let mut graph = UnderlyingGraph {
            nodes: Vec::new(),
            adjacency: HashMap::new(),
        };

        graph.adjacency = pog_graph
            .adjacency()
            .into_iter()
            .map(|(node, adjacencies)| {
                let neighbors: Vec<String> = adjacencies
                    .into_iter()
                    .filter_map(|(neighbor, orientation)| {
                        if orientation == Orientation::Zero {
                            Some(neighbor)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>();
                (node, neighbors)
            })
            .collect();
        graph.nodes = graph
            .adjacency()
            .keys()
            .map(|key| key.clone())
            .collect::<Vec<String>>();

        graph
    }
}

impl GraphTrait for UnderlyingGraph {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_models::graph_types::helpers::{generate, init_graph};

    #[test]
    fn new_underlying_graph() {
        for n in 2..=12 {
            let puzzle = generate(n);
            let pog = init_graph(puzzle);
            let under = UnderlyingGraph::from(&pog);

            // Check that the nodes in UnderlyingGraph are a subset of PogGraph's nodes
            assert!(under.nodes().len() <= pog.nodes().len());
            for node in under.nodes() {
                assert!(pog.nodes().contains(&node));
            }

            // Check the unoriented edges count
            let mut unoriented = 0;
            for edges in under.adjacency().values() {
                unoriented += edges.len();
            }
            assert!(unoriented > 0);
        }
    }
}
