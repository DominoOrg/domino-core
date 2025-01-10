use std::collections::HashMap;

use crate::graph_models::graph_types::{
    graph::GraphTrait, pog_graph::PogGraph, under_graph::UnderlyingGraph, GraphNode,
};

#[derive(Debug, Clone)]
pub struct AuxiliaryGraph {
    nodes: Vec<String>,
    adjacency: HashMap<String, Vec<String>>,
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
                    .filter(|neighbor| *neighbor != node)
                    .map(|neighbor| (node.clone(), neighbor))
                    .collect::<Vec<(GraphNode, GraphNode)>>()
            })
            .flatten()
            .collect::<Vec<(GraphNode, GraphNode)>>();

        // Insert nodes
        edges.iter().for_each(|edge| {
            graph.insert_node(edge.0.to_string() + "," + &edge.1.to_string());
            graph.insert_node(edge.1.to_string() + "," + &edge.0.to_string());
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
                            || (x == u && !edges.contains(&(y.clone(), v.clone())))
                            || (y == v && !edges.contains(&(x.clone(), u))));
                })
                .for_each(|node2| {
                    graph.insert_or_update(node.clone(), None, node2.clone());
                });
        });
        graph
    }

    #[allow(dead_code)]
    fn edge_to_string(edge: (&String, &String)) -> String {
        edge.0.to_string() + "," + edge.1.as_str()
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
    type Edge = String;

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
