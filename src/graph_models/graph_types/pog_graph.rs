use std::collections::HashMap;

use crate::graph_models::graph_types::{
    graph::GraphTrait, regular_graph::RegularGraph, GraphNode, Orientation,
};

#[derive(Debug)]
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
                        .map(|node| {
                            if key == node {
                                vec![(node.clone(), Orientation::Zero), (node, Orientation::Zero)]
                            } else {
                                vec![(node, Orientation::Zero)]
                            }
                        })
                        .flatten()
                        .collect(),
                )
            })
            .collect::<HashMap<String, Vec<(String, Orientation)>>>();

        pog_graph
    }

    pub fn update_orientation(&mut self, u: &GraphNode, v: &GraphNode, orientation: Orientation) {
        if let Some(neighbors) = self.adjacency.get_mut(u) {
            if let Some(index) = neighbors
                .iter()
                .position(|el| el.0 == *v && el.1 == Orientation::Zero)
            {
                neighbors[index].1 = orientation;
            }
        }
    }

    pub fn orient_arc(&mut self, u: &GraphNode, v: &GraphNode) {
        self.update_orientation(u, v, Orientation::Positive);
        self.update_orientation(v, u, Orientation::Negative);
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

#[cfg(test)]
mod tests {
    use crate::graph_models::graph_types::{
        helpers::{generate, hierholzer, init_graph},
        pog_graph::PogGraph,
    };

    use super::*;

    #[test]
    fn new_pog_graph() {
        let pog = PogGraph::new();
        assert!(pog.nodes().is_empty());
        assert!(pog.adjacency().is_empty());
        for edges in pog.adjacency().values() {
            assert_eq!(edges.len(), 0);
        }
    }

    #[test]
    fn pog_graph_from() {
        for n in 2..=12 {
            let regular = RegularGraph::new(n);
            let pog = PogGraph::from(&regular);
            assert_eq!(pog.nodes().len(), n + 1);
            assert_eq!(pog.adjacency().len(), n + 1);
            for edges in pog.adjacency().values() {
                assert_eq!(edges.len(), n - n % 2 + 2);
            }
        }
    }

    #[test]
    fn hierholzer_pog_graph() {
        for n in 2..=12 {
            let mut regular = RegularGraph::new(n);
            let pog = hierholzer(&mut regular);
            let mut global_degree = 0;
            let mut removed = 0;

            assert_eq!(pog.nodes().len(), n + 1);
            assert_eq!(pog.adjacency().len(), n + 1);
            for edges in pog.adjacency().values() {
                let mut entering = 0;
                let mut unoriented = 0;
                let mut exiting = 0;

                assert_eq!(edges.len(), n - n % 2 + 2);
                for edge in edges {
                    match edge.1 {
                        Orientation::Negative => entering += 1,
                        Orientation::Zero => unoriented += 1,
                        Orientation::Positive => exiting += 1,
                    }
                }
                global_degree += entering - exiting;
                removed += unoriented;
            }
            assert_eq!(global_degree, 0);
            assert_eq!(removed, 0);
        }
    }

    #[test]
    fn generate_pog_graph() {
        for n in 2..=12 {
            let puzzle = generate(n);
            let pog = init_graph(puzzle);
            let mut global_degree = 0;
            let mut removed = 0;

            assert_eq!(pog.nodes().len(), n + 1);
            assert_eq!(pog.adjacency().len(), n + 1);
            for edges in pog.adjacency().values() {
                let mut entering = 0;
                let mut unoriented = 0;
                let mut exiting = 0;

                assert_eq!(edges.len(), n - n % 2 + 2);
                for edge in edges {
                    match edge.1 {
                        Orientation::Negative => entering += 1,
                        Orientation::Zero => unoriented += 1,
                        Orientation::Positive => exiting += 1,
                    }
                }
                global_degree += entering - exiting;
                removed += unoriented;
            }
            assert_eq!(global_degree, 0);
            assert!(removed > 0);
        }
    }
}
