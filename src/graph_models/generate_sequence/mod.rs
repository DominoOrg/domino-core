use crate::graph_models::graph_types::{graph::GraphTrait, regular_graph::RegularGraph};
use hierholzer::hierholzer;
use sequence::as_sequence;

mod hierholzer;
pub mod sequence;

pub fn generate_solution(n: usize, random: bool) -> Vec<(String, String)> {
    let mut reg_graph = RegularGraph::new(n);
    let pog_graph = hierholzer(&mut reg_graph, random);
    let sequence = as_sequence(&pog_graph.adjacency(), random);
    sequence
        .into_iter()
        .map(|tile| {
            let tile = tile.unwrap();
            (tile.0.to_string(), tile.1.to_string())
        })
        .collect()
}
