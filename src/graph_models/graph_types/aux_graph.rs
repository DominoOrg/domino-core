use std::collections::HashMap;

use crate::graph_models::graph_types::{
    graph::GraphTrait, pog_graph::PogGraph, under_graph::UnderlyingGraph, GraphNode,
};

use super::{graph::GraphEdge, Orientation};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct AuxiliaryGraphEdge {
    from: String,
    to: String,
}

impl From<(String, String)> for AuxiliaryGraphEdge {
    fn from((from, to): (String, String)) -> Self {
        Self { from, to }
    }
}

impl GraphEdge for AuxiliaryGraphEdge {
    type FromNode = String;

    type ToNode = String;

    type Orientation = Orientation;

    fn from_node(&self) -> Self::FromNode {
        self.from.clone()
    }

    fn to_node(&self) -> Self::ToNode {
        self.to.clone()
    }

    fn orientation(&self) -> super::Orientation {
        Orientation::Zero
    }
}

#[derive(Debug, Clone)]
pub struct AuxiliaryGraph {
    nodes: Vec<String>,
    adjacency: HashMap<String, Vec<AuxiliaryGraphEdge>>,
}

impl AuxiliaryGraph {
    pub fn from(pog_graph: &PogGraph) -> Self {
        let mut graph = AuxiliaryGraph {
            nodes: Vec::new(),
            adjacency: HashMap::new(),
        };
        let under_graph = UnderlyingGraph::from(pog_graph);
        println!("{under_graph:?}");

        // Collect unoriented egdes filtering out self-loops
        let edges = under_graph
            .adjacency()
            .into_iter()
            .map(|(node, neighbors)| {
                neighbors
                    .into_iter()
                    .filter(|neighbor| neighbor.to_node() != node)
                    .map(|neighbor| {
                        let mut new_node = node.to_string();
                        new_node.push_str(&neighbor.to_node().to_string());
                        (node.to_string(),new_node).into()
                    })
                    .collect::<Vec<AuxiliaryGraphEdge>>()
            })
            .flatten()
            .collect::<Vec<AuxiliaryGraphEdge>>();

        // Insert nodes
        edges.iter().for_each(|edge| {
            graph.insert_node(edge.from_node().to_string() + "," + &edge.to_node().to_string());
            graph.insert_node(edge.to_node().to_string() + "," + &edge.from_node().to_string());
        });

        // Insert edges
        graph.nodes().iter().for_each(|node| {
            let (x, y) = Self::string_to_edge(&node).unwrap();
            graph
                .nodes()
                .iter()
                .filter(|node2| {
                    let (u, v) = Self::string_to_edge(node2).unwrap();
                    return node != *node2
                        && ((x == v && y == u)
                            || (x == u && !edges.contains(&(y.clone().to_string(), v.clone().to_string()).into()))
                            || (y == v && !edges.contains(&(x.clone().to_string(), u.to_string()).into())));
                })
                .for_each(|node2| {
                    graph.insert_or_update(
                        node.clone(),
                        None,
                        AuxiliaryGraphEdge::from((node.clone(), node2.clone())));
                });
        });
        graph
    }

    pub fn string_to_edge(str: &String) -> Option<(GraphNode, GraphNode)> {
        let parts = str
            .split(",")
            .map(|str| str.to_string())
            .collect::<Vec<String>>();
        if parts.len() == 2 {
            Some(
                (
                    i32::from_str_radix(&parts[0], 10).unwrap() as GraphNode,
                    i32::from_str_radix(&parts[1], 10).unwrap() as GraphNode
                )
            )
        } else {
            None
        }
    }

    pub fn get_nodes(&self) -> &Vec<String> {
        &self.nodes
    }
}

impl GraphTrait for AuxiliaryGraph {
    type Node = String;
    type Edge = AuxiliaryGraphEdge;

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
