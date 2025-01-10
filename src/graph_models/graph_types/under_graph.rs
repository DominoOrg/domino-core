use std::collections::HashMap;

use crate::graph_models::graph_types::{
    graph::GraphTrait, pog_graph::PogGraph, GraphNode, Orientation,
};

#[derive(Debug, Clone)]
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
                let neighbors: Vec<GraphNode> = adjacencies
                    .into_iter()
                    .filter_map(|(neighbor, orientation)| {
                        if orientation == Orientation::Zero {
                            Some(neighbor)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<GraphNode>>();
                (node, neighbors)
            })
            .collect();
        graph.nodes = graph
            .adjacency()
            .keys()
            .map(|key| key.clone())
            .collect::<Vec<GraphNode>>();

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
