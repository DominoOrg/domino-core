use std::collections::HashMap;

use crate::graph_models::graph_types::{graph::GraphTrait, pog_graph::PogGraph, Orientation};

pub struct DirectedGraph {
    nodes: Vec<String>,
    adjacency: HashMap<String, Vec<(String, Orientation)>>,
}

impl DirectedGraph {
    pub fn from(pog_graph: &PogGraph) -> DirectedGraph {
        let mut digraph = DirectedGraph {
            nodes: Vec::new(),
            adjacency: HashMap::new(),
        };

        let arcs: Vec<(String, String)> = pog_graph
            .adjacency()
            .into_iter()
            .map(|(node, neighbors)| {
                neighbors
                    .into_iter()
                    .filter_map(|(neighbor, orientation)| {
                        if orientation == Orientation::Positive {
                            Some((node.clone(), neighbor))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(String, String)>>()
            })
            .flatten()
            .collect::<Vec<(String, String)>>();

        for (u, v) in arcs {
            digraph.insert_node(u.clone());
            digraph.insert_node(v.clone());
            digraph.insert_or_update(
                u.clone(),
                Some((v.clone(), Orientation::Zero)),
                (v.clone(), Orientation::Positive),
            );
            digraph.insert_or_update(
                v,
                Some((u.clone(), Orientation::Zero)),
                (u, Orientation::Negative),
            );
        }

        digraph
    }
}

impl GraphTrait for DirectedGraph {
    type Node = String;
    type Edge = (String, Orientation);

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
