use std::collections::HashSet;

use rand::seq::IteratorRandom;

use super::{Arc, Graph, Node};

pub(super) struct NextNodeBuilder<'a> {
    graph: &'a Graph,
    random: Option<bool>,
    visited: Option<HashSet<Arc>>,
    unvisited_edge_index: Option<Option<usize>>, // Option wrapped in Option due to its type
    current_vertex: Option<Node>,
}

impl<'a> NextNodeBuilder<'a> {
  // Constructor: takes the graph as the first parameter
  pub fn new(graph: &'a Graph) -> Self {
      Self {
          graph,
          random: None,
          visited: None,
          unvisited_edge_index: None,
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

  pub fn with_unvisited_edge_index(mut self, unvisited_edge_index: Option<usize>) -> Self {
      self.unvisited_edge_index = Some(unvisited_edge_index);
      self
  }

  pub fn with_current_vertex(mut self, current_vertex: Node) -> Self {
      self.current_vertex = Some(current_vertex);
      self
  }

  // Build method: computes and returns the next Node
  pub fn build(self) -> Option<usize> {
      // Default values for optional fields if not provided
      let random = self.random.unwrap_or(false);
      let visited = self.visited.unwrap_or_else(HashSet::new);
      let _unvisited_edge_index = self.unvisited_edge_index.unwrap_or(None);
      let current_vertex = self.current_vertex.expect("current_vertex is required to build");

      let mut seed = rand::thread_rng();
      let edges = self.graph.adjacency
          .get(&current_vertex)
          .unwrap();

      if random {
          edges.iter()
              .enumerate()
              .filter(|(_, arc)| {
                  !visited.contains(&Arc(current_vertex.clone(), arc.destination.clone())) &&
                  !visited.contains(&Arc(arc.destination.clone(), current_vertex.clone()))
              })
              .choose(&mut seed)
              .map(|(_, arc)| arc.destination.clone() as usize)
      } else {
          let double_position = if !visited.contains(&Arc(current_vertex.clone(), current_vertex.clone())) {
              edges.iter()
                  .position(|arc| arc.destination == current_vertex)
          } else {
              None
          };

          if let Some(pos) = double_position {
              Some(edges[pos].destination.clone() as usize)
          } else {
              edges.iter()
                  .find(|arc| {
                      current_vertex != arc.destination &&
                      !visited.contains(&Arc(current_vertex.clone(), arc.destination.clone())) &&
                      !visited.contains(&Arc(arc.destination.clone(), current_vertex.clone()))
                  })
                  .map(|arc| arc.destination.clone() as usize)
              }
          }
      }
}
