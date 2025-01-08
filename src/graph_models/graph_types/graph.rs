use std::collections::HashMap;

pub trait GraphTrait {
    type Node: std::cmp::Eq + std::hash::Hash + Clone; // `Node` type, must be hashable for HashSet and HashMap
    type Edge: std::cmp::Eq + Clone; // `Edge` type

    fn nodes(&self) -> Vec<Self::Node>;
    fn adjacency(&self) -> HashMap<Self::Node, Vec<Self::Edge>>;
    fn mut_nodes(&mut self) -> &mut Vec<Self::Node>;
    fn mut_adjacency(&mut self) -> &mut HashMap<Self::Node, Vec<Self::Edge>>;

    fn insert_or_update(
        &mut self,
        key: Self::Node,
        old_value: Option<Self::Edge>,
        new_value: Self::Edge,
    ) {
        self.mut_adjacency()
            .entry(key)
            .and_modify(|adjacencies| {
                if let Some(old_value) = old_value {
                    if let Some(index) = adjacencies.iter().position(|edge| *edge == old_value) {
                        adjacencies[index] = new_value.clone();
                    } else {
                        adjacencies.push(new_value.clone());
                    }
                } else {
                    adjacencies.push(new_value.clone());
                }
            })
            .or_insert(vec![new_value]);
    }

    fn insert_node(&mut self, node: Self::Node) {
        if !self.nodes().contains(&node) {
            self.mut_nodes().push(node);
        }
    }

    fn remove_edge(&mut self, node: &Self::Node, edge: &Self::Edge) {
        if let Some(edges) = self.mut_adjacency().get_mut(node) {
            if let Some(index) = edges.iter().position(|e| e == edge) {
                edges.remove(index);
            }
        }
    }
}
