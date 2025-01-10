use std::collections::HashMap;

use crate::graph_models::graph_types::{
    graph::GraphTrait, regular_graph::RegularGraph, GraphNode, Orientation,
};

#[derive(Debug, Clone)]
pub struct PogGraph {
    nodes: Vec<GraphNode>,
    adjacency: HashMap<GraphNode, Vec<(GraphNode, Orientation)>>,
}

impl PogGraph {
    pub fn new() -> Self {
        PogGraph {
            nodes: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn from(reg_graph: &RegularGraph) -> Self {
        let mut pog_graph = PogGraph::new();

        for node in reg_graph.nodes() {
            pog_graph.insert_node(node.clone());
        }

        pog_graph.adjacency = reg_graph
            .adjacency()
            .into_iter()
            .map(|(key, values)| {
                (
                    key.clone(),
                    values
                        .into_iter()
                        .map(|node| (node, Orientation::Zero))
                        .collect(),
                )
            })
            .collect::<HashMap<GraphNode, Vec<(GraphNode, Orientation)>>>();

        pog_graph
    }

    pub fn update_orientation(&mut self, u: &GraphNode, v: &GraphNode, orientation: Orientation) {
        if let Some(neighbors) = self.adjacency.get_mut(u) {
            if let Some(index) = neighbors
                .iter()
                .position(|el| el.0 == *v)
            {
                neighbors[index].1 = orientation;
            }
        }
    }

    pub fn orient_arc(&mut self, u: &GraphNode, v: &GraphNode) {
        self.update_orientation(u, v, Orientation::Positive);
        self.update_orientation(v, u, Orientation::Negative);
    }

    pub fn deorient_arc(&mut self, u: &GraphNode, v: &GraphNode) {
        self.update_orientation(u, v, Orientation::Zero);
        self.update_orientation(v, u, Orientation::Zero);
    }
}

impl GraphTrait for PogGraph {
    type Node = GraphNode;
    type Edge = (GraphNode, Orientation);

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
