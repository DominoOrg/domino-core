use crate::graph_models::graph_types::graph::GraphTrait;


pub fn hierholzer<T: GraphTrait + Clone>(graph: &T, len: usize) -> Option<Vec<(T::Node, T::Node)>> {
    let mut graph = graph.clone(); // Clone the graph for manipulation
    let mut path: Vec<(<T as GraphTrait>::Node, <T as GraphTrait>::Node)> = Vec::new();
    let mut stack = Vec::new();

    // Find a starting node. If there are no nodes, return None
    let start_node = match graph.nodes().first() {
        Some(node) => node.clone(),
        None => return None,
    };

    // Push the start node onto the stack
    stack.push(start_node.clone());

    while !stack.is_empty() && path.len() < len {
        let v = stack.last().unwrap().clone();
        println!("v: {v:?}");
        if let Some(edges) = graph.adjacency().get(&v) {
            if !edges.is_empty() {
                // Get the next node from the first edge of v
                let next_edge = edges[0].clone();
                let next_node = match find_next_node(&graph, &v, &next_edge) {
                    Some(node) => node,
                    None => return None, // This shouldn't happen if the graph is Eulerian
        };

                // Remove the edge from the graph
                graph.remove_edge(&v, &next_edge);

                // Add the next node to the stack to explore further
                stack.push(next_node.clone());
            } else {
                // If there are no more edges, pop the node and add it to the path
                if let Some(node) = stack.pop() {
                    if !path.is_empty() {
                        path.push((path.last().unwrap().1.clone(), node.clone()));
                    } else {
                        // This is the first node, we'll connect it later
                        path.push((node.clone(), node.clone()));
                    }
                }
            }
        } else {
            // This shouldn't happen if graph.adjacency() is always consistent with graph.nodes()
            return None;
        }
    }
    println!("path: {path:?}");
    // If path is not empty, we've found an Eulerian path
    if !path.is_empty() {
        Some(path)
    } else {
        None
    }
}

// Helper function to find the next node given an edge
fn find_next_node<T: GraphTrait>(graph: &T, from: &T::Node, edge: &T::Edge) -> Option<T::Node> {
    let adjacency = graph.adjacency();
    for (node, edges) in adjacency.iter() {
        if node != from && edges.contains(edge) {
            return Some(node.clone());
        }
    }
    None
}