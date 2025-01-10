use std::collections::HashMap;

use crate::graph_models::graph_types::{graph::GraphTrait, pog_graph::PogGraph, Orientation};

use super::{graph::GraphEdge, GraphNode};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DirectedGraphEdge {
    from: GraphNode,
    to: GraphNode,
    orientation: Orientation,
}

impl GraphEdge for DirectedGraphEdge {
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

impl From<(GraphNode, Orientation)> for DirectedGraphEdge {
    fn from(value: (GraphNode, Orientation)) -> Self {
        Self {
            from: value.0,
            to: value.0,
            orientation: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirectedGraph {
    nodes: Vec<GraphNode>,
    adjacency: HashMap<GraphNode, Vec<DirectedGraphEdge>>,
}

impl DirectedGraph {
    pub fn from(pog_graph: &PogGraph) -> DirectedGraph {
        let mut digraph = DirectedGraph {
            nodes: Vec::new(),
            adjacency: HashMap::new(),
        };

        let arcs: Vec<(GraphNode, GraphNode)> = pog_graph
            .adjacency()
            .into_iter()
            .map(|(node, neighbors)| {
                neighbors
                    .into_iter()
                    .filter_map(|edge| {
                        if edge.orientation() == Orientation::Positive {
                            Some((node.clone(), edge.to_node()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(GraphNode, GraphNode)>>()
            })
            .flatten()
            .collect::<Vec<(GraphNode, GraphNode)>>();

        for (u, v) in arcs {
            digraph.insert_node(u.clone());
            digraph.insert_node(v.clone());
            digraph.insert_or_update(
                u.clone(),
                Some((v.clone(), Orientation::Zero).into()),
                (v.clone(), Orientation::Positive).into(),
            );
            digraph.insert_or_update(
                v,
                Some((u.clone(), Orientation::Zero).into()),
                (u, Orientation::Negative).into(),
            );
        }

        digraph
    }
}

impl GraphTrait for DirectedGraph {
    type Node = GraphNode;
    type Edge = DirectedGraphEdge;

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
