use crate::graph_models::graph_types::{
    graph::GraphTrait, pog_graph::PogGraph, regular_graph::RegularGraph,
};
use rand::{thread_rng, Rng};

fn next_node(neighbors: &[String], random: bool) -> String {
    if random {
        let mut seed = thread_rng();
        let index = seed.gen_range(0..neighbors.len());
        neighbors[index].clone()
    } else {
        neighbors[0].clone() // Deterministically pick the first neighbor
    }
}

pub fn hierholzer(reg_graph: &mut RegularGraph, random: bool) -> PogGraph {
    let mut pog_graph = PogGraph::from(reg_graph);
    let mut stack = Vec::new();

    // Start from a random node
    let mut vertexes = reg_graph.nodes().into_iter().collect::<Vec<String>>();
    vertexes.sort();
    let mut current_node = next_node(&vertexes, random);
    let mut neighbors = reg_graph
        .adjacency()
        .get(&current_node)
        .unwrap_or(&Vec::new())
        .clone();
    stack.push(current_node.clone());

    while !stack.is_empty() {
        if !neighbors.is_empty() {
            let previous_node = current_node.clone();
            current_node = next_node(&neighbors, random); // Move to the next neighbor
            stack.push(current_node.clone());

            // Orient the arc for the forward direction and remove the edge from the connGraph
            pog_graph.orient_arc(&previous_node, &current_node);
            reg_graph.remove_edge(&previous_node, &current_node);

            neighbors = reg_graph
                .adjacency()
                .get(&current_node)
                .unwrap_or(&Vec::new())
                .clone();
        } else {
            // Backtrack if all neighbors of the current node are exhausted
            current_node = stack.pop().unwrap();

            // Before continuing, we must ensure the node fully processes its neighbors
            if !stack.is_empty() {
                let next_node_in_stack = stack[stack.len() - 1].clone();

                // Orient the arc in reverse when backtracking and remove the edge
                pog_graph.orient_arc(&current_node, &next_node_in_stack);
                reg_graph.remove_edge(&current_node, &next_node_in_stack);

                // Update neighbors after popping to ensure no neighbors are reprocessed
                neighbors = reg_graph
                    .adjacency()
                    .get(&next_node_in_stack)
                    .unwrap_or(&Vec::new())
                    .clone();
            }
        }
    }

    pog_graph
}
