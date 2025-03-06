use std::collections::HashSet;

use rand::seq::IteratorRandom;

use crate::{utils::Arc, Graph, Node};

pub(super) struct NextNodeBuilder<'a> {
    graph: &'a Graph,
    random: Option<bool>,
    visited: Option<HashSet<Arc>>,
    current_vertex: Option<Node>,
}

impl<'a> NextNodeBuilder<'a> {
    // Constructor: takes the graph as the first parameter
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            random: None,
            visited: None,
            current_vertex: None,
        }
    }

    // Builder methods: each returns Self to enable chaining
    pub fn with_random(mut self, random: bool) -> Self {
        self.random = Some(random);
        self
    }

    pub fn with_visited(mut self, visited: HashSet<Arc>) -> Self {
        self.visited = Some(visited);
        self
    }

    pub fn with_current_vertex(mut self, current_vertex: Node) -> Self {
        self.current_vertex = Some(current_vertex);
        self
    }

    // Build method: computes and returns the next Node
    pub fn build(self) -> Option<usize> {
        let graph = self.graph;
        let random = self.random.unwrap_or(false);
        let visited = self.visited.unwrap_or(HashSet::new());
        let current_vertex = self.current_vertex.unwrap();
        let mut seed = rand::thread_rng();
        let mut edges_iterator = graph.adjacency.get(&current_vertex.clone()).unwrap().iter();
        if random {
            edges_iterator
                .enumerate()
                .filter(|(_, arc)| {
                    !visited.contains(&Arc::from((
                        current_vertex.clone(),
                        arc.destination.clone(),
                    ))) && !visited.contains(&Arc::from((
                        arc.destination.clone(),
                        current_vertex.clone(),
                    )))
                })
                .choose(&mut seed)
                .map(|(index, _)| index)
        } else {
            let position = edges_iterator.position(|arc| {
                return !visited.contains(&Arc::from((
                    current_vertex.clone(),
                    arc.destination.clone(),
                ))) && !visited.contains(&Arc::from((
                    arc.destination.clone(),
                    current_vertex.clone(),
                )));
            });
            return position;
            // let double_position = if !visited.contains(&Arc(current_vertex, current_vertex)) {
            //   graph.adjacency
            //   .get(&current_vertex.clone()).unwrap()
            //   .iter()
            //   .position(|arc| {
            //       return arc.destination == current_vertex;
            //   })
            // } else {
            //   None
            // };
            // if double_position.is_none() {
            //   let position = edges_iterator
            //     .position(|arc| {
            //           return current_vertex != arc.destination &&
            //           !visited.contains(&Arc(current_vertex.clone(), arc.destination.clone())) &&
            //           !visited.contains(&Arc(arc.destination.clone(), current_vertex.clone()));
            //         });
            //   return position;
            // }
            // double_position
        }
    }
}
