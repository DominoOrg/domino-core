use std::collections::HashMap;

use crate::graph_models::graph_types::{
    graph::GraphTrait, pog_graph::PogGraph, under_graph::UnderlyingGraph,
};

#[derive(Debug)]
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
        let edges = under_graph
            .adjacency()
            .into_iter()
            .map(|(node, neighbors)| {
                neighbors
                    .into_iter()
                    .map(|neighbor| (node.clone(), neighbor))
                    .collect::<Vec<(String, String)>>()
            })
            .flatten()
            .collect::<Vec<(String, String)>>();

        under_graph
            .adjacency()
            .into_iter()
            .for_each(|(node, adjacencies)| {
                adjacencies.into_iter().for_each(|adjacent_node| {
                    let node = Self::edge_to_string((&node, &adjacent_node));
                    graph.insert_node(node);
                })
            });

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

    pub fn edge_to_string(edge: (&String, &String)) -> String {
        edge.0.to_string() + "," + edge.1.as_str()
    }

    pub fn string_to_edge(str: &String) -> Option<(String, String)> {
        let parts = str
            .split(",")
            .map(|str| str.to_string())
            .collect::<Vec<String>>();
        if parts.len() == 2 {
            Some((parts[0].clone(), parts[1].clone()))
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

#[cfg(test)]
mod tests {

    use crate::graph_models::graph_types::{
        graph::GraphTrait,
        helpers::{generate, init_graph},
        pog_graph::PogGraph,
        Orientation,
    };

    use super::AuxiliaryGraph;

    fn removed_tiles(pog: &PogGraph) -> Vec<(String, String)> {
        pog.adjacency()
            .into_iter()
            .map(|(key, adjacencies)| {
                adjacencies
                    .into_iter()
                    .filter_map(|(neighbor, direction)| {
                        if direction == Orientation::Zero {
                            Some((key.clone(), neighbor))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(String, String)>>()
            })
            .flatten()
            .collect::<Vec<(String, String)>>()
    }

    #[test]
    fn new_aux_graph() {
        for n in 2..=12 {
            let puzzle = generate(n);
            let pog: PogGraph = init_graph(puzzle);
            let removed_tiles = removed_tiles(&pog);
            let aux: AuxiliaryGraph = AuxiliaryGraph::from(&pog);

            for tile in &removed_tiles {
                let str_to_check = AuxiliaryGraph::edge_to_string((&tile.0, &tile.1));
                assert!(aux.get_nodes().contains(&str_to_check));
            }

            aux.nodes().into_iter().for_each(|node| {
                let (x, y) = AuxiliaryGraph::string_to_edge(&node).unwrap();
                aux.nodes().into_iter().for_each(|node2| {
                    let (u, v) = AuxiliaryGraph::string_to_edge(&node2).unwrap();
                    if (x == v && y == u)
                        || (x == u && !removed_tiles.contains(&(y.clone(), v.clone())))
                        || (y == v && !removed_tiles.contains(&(x.clone(), u)))
                    {
                        assert!(aux.adjacency().get(&node).unwrap().contains(&node2))
                    }
                });
            });
        }
    }
}
