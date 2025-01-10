use std::collections::HashMap;

use crate::graph_models::graph_types::{
    graph::GraphTrait, pog_graph::PogGraph, GraphNode, Orientation,
};

use super::graph::GraphEdge;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UnderlyingGraphEdge {
    from: GraphNode,
    to: GraphNode,
    orientation: Orientation,
}

impl GraphEdge for UnderlyingGraphEdge {
    type FromNode = GraphNode;

    type ToNode = GraphNode;

    type Orientation = Orientation;

    fn from_node(&self) -> Self::FromNode {
        self.from
    }

    fn to_node(&self) -> Self::ToNode {
        self.to
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }
}

impl From<(GraphNode, GraphNode)> for UnderlyingGraphEdge {
    fn from(node: (GraphNode, GraphNode)) -> Self {
        UnderlyingGraphEdge {
            from: node.0,
            to: node.1,
            orientation: Orientation::Zero,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnderlyingGraph {
    nodes: Vec<GraphNode>,
    adjacency: HashMap<GraphNode, Vec<UnderlyingGraphEdge>>,
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
                let neighbors: Vec<UnderlyingGraphEdge> = adjacencies
                    .into_iter()
                    .filter_map(|edge| {
                        if edge.orientation() == Orientation::Zero {
                            Some((edge.from_node(), edge.to_node()).into())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<UnderlyingGraphEdge>>();
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
    type Edge = UnderlyingGraphEdge;

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
