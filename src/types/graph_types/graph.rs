use std::collections::HashMap;

use crate::{functionalities::common::SequenceScraper, types::domino_types::{error::DominoError, Puzzle}};

use super::{Arc, Node, Orientation};

#[derive(Debug, Default, Clone)]
pub(crate) struct Graph {
    pub(crate) nodes: Vec<Node>,
    pub(crate) adjacency: HashMap<Node, Vec<Arc>>
}

impl Graph {
    pub fn regular(n: usize) -> Self {
        Graph {
            nodes: (0..n as i32).into_iter().map(|node| Node::Regular(node)).collect::<Vec<Node>>(),
            adjacency: (0..n as i32)
                .map(|source_node| {
                    let edges = (0..n).map(|destination_node| {
                        Arc {
                            source: source_node.into(),
                            destination: (destination_node as i32).into(),
                            orientation: None,
                            position: None
                        }
                    }).collect::<Vec<Arc>>();
                    (Node::Regular(source_node), edges)
                })
                .collect()
        }
    }

    pub fn partially_oriented(puzzle: &Puzzle) -> Result<Self, DominoError> {
        let n = SequenceScraper::get_n(puzzle)?;
        let mut graph = Graph::regular(n.try_into().unwrap());
        for (index, tile) in puzzle.iter().enumerate() {
            if let Some(tile) = tile {
                graph.adjacency.get_mut(&(index as i32).into()).unwrap().push(Arc {
                    source: (tile.0 as i32).into(),
                    destination: (tile.1 as i32).into(),
                    orientation: Some(Orientation::Positive),
                    position: Some(index)
                });
            }
        }
        Ok(graph)
    }

    pub fn underlying(pog_graph: &Graph) -> Self {
        let mut underlying_graph = pog_graph.clone();
        for node in &pog_graph.nodes {
            underlying_graph.adjacency.get_mut(node).unwrap().retain(|arc| arc.orientation.is_none());
        }
        underlying_graph
    }

    pub fn auxiliary(underlying_graph: &Graph) -> Self {
        let mut auxiliary_graph = Graph::default();
        for (_, arcs) in &underlying_graph.adjacency {
            for arc in arcs {
                auxiliary_graph.nodes.push(arc.clone().try_into().unwrap());
            }
        }
        auxiliary_graph
    }
}