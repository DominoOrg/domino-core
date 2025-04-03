use crate::{Graph, Node};

use super::hierholzer::hierholzer;

pub fn find_eulerian_cycle<'a>(graph: &'a Graph, random: bool) -> Vec<Node> { hierholzer(graph,random) }
