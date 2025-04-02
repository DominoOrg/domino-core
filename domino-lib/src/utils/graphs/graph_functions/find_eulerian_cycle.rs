use crate::{Graph, Node};

use super::hierholzer::hierholzer;

pub fn find_eulerian_cycle<'a>(graph: &'a Graph) -> impl Fn(bool) -> Vec<Node> + use<'a> {
    move |random: bool| -> Vec<Node> { hierholzer(graph)(random) }
}
