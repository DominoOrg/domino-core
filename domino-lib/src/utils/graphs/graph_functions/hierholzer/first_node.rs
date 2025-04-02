use rand::seq::SliceRandom;

use crate::{Graph, Node};

pub(super) fn first_node<'a>(graph: &'a Graph) -> impl Fn(bool) -> Node + 'a {
    move |random: bool| {
        let mut seed = rand::thread_rng();
        if random {
            graph.nodes.choose(&mut seed).unwrap().clone()
        } else {
            graph.nodes.first().unwrap().clone()
        }
    }
}
