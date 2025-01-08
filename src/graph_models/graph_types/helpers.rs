use std::collections::{HashMap, HashSet};

use rand::{thread_rng, Rng};

use crate::graph_models::graph_types::{
    graph::GraphTrait, pog_graph::PogGraph, regular_graph::RegularGraph, Orientation,
};

#[allow(dead_code)]
pub fn init_graph(sequence: Vec<Option<(String, String)>>) -> PogGraph {
    let l = sequence.len();
    let n_p = (-3.0 + (1.0 + 8.0 * (l as f64)).sqrt()) / 2.0;
    let n_d = (-2.0 + (8.0 * (l as f64)).sqrt()) / 2.0;
    let n = if (n_p - n_p.floor()).abs() == 0.0 {
        n_p.floor() as usize
    } else {
        n_d.floor() as usize
    };

    // Build the regular graph and convert it to a PogGraph
    let graph = RegularGraph::new(n);
    let mut pog_graph = PogGraph::from(&graph);

    // Insert edges into the PogGraph
    for tile in sequence.iter().filter_map(|tile| {
        if let Some(tile) = tile {
            Some((tile.0.clone(), tile.1.clone()))
        } else {
            None
        }
    }) {
        let (u, v) = tile;
        pog_graph.insert_or_update(
            u.clone(),
            Some((v.clone(), Orientation::Zero)),
            (v.clone(), Orientation::Positive),
        );
        pog_graph.insert_or_update(
            v,
            Some((u.clone(), Orientation::Zero)),
            (u, Orientation::Negative),
        );
    }

    pog_graph
}

#[allow(dead_code)]
fn next_node(neighbors: &[String]) -> String {
    neighbors[0].clone() // Deterministically pick the first neighbor
}

#[allow(dead_code)]
pub fn hierholzer(reg_graph: &mut RegularGraph) -> PogGraph {
    let mut pog_graph = PogGraph::from(reg_graph);
    let mut stack = Vec::new();

    // Start from a random node
    let mut current_node = next_node(&reg_graph.nodes().iter().cloned().collect::<Vec<String>>());
    let mut neighbors = reg_graph
        .adjacency()
        .get(&current_node)
        .unwrap_or(&Vec::new())
        .clone();
    stack.push(current_node.clone());

    while !stack.is_empty() {
        if !neighbors.is_empty() {
            let previous_node = current_node.clone();
            current_node = next_node(&neighbors); // Move to the next neighbor
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

#[allow(dead_code)]
fn find_eulerian_path(graph: &HashMap<String, Vec<String>>) -> Option<Vec<String>> {
    let mut path = Vec::new();
    let mut visited_edges = HashSet::new();

    // Find a start node with an odd degree (Eulerian path starts at odd-degree node, if exists)
    let start_node = graph
        .keys()
        .find(|&node| graph[node].len() % 2 != 0)
        .or_else(|| graph.keys().next())?; // Start at any node if all degrees are even

    eulerian_dfs(start_node, graph, &mut visited_edges, &mut path);
    path.reverse();

    Some(path)
}

#[allow(dead_code)]
fn eulerian_dfs(
    node: &String,
    graph: &HashMap<String, Vec<String>>,
    visited_edges: &mut HashSet<(String, String)>,
    path: &mut Vec<String>,
) {
    for neighbor in &graph[node] {
        // If the edge hasn't been visited, traverse it
        let edge = if node < neighbor {
            (node.clone(), neighbor.clone())
        } else {
            (neighbor.clone(), node.clone())
        };
        if !visited_edges.contains(&edge) {
            visited_edges.insert(edge.clone()); // Mark edge as visited
            eulerian_dfs(&neighbor, graph, visited_edges, path);
        }
    }
    // Backtrack to build the path
    path.push(node.clone());
}

#[allow(dead_code)]
fn generate_solution(n: usize) -> Vec<(String, String)> {
    // Initialize the graph
    let reg_graph = RegularGraph::new(n);

    // Generate the Eulerian path using the Hierholzer algorithm
    let first_node = "0".to_string();
    let mut path = Vec::new();
    eulerian_dfs(
        &first_node,
        &reg_graph.adjacency(),
        &mut HashSet::new(),
        &mut path,
    );

    // Convert the adjacency list of PogGraph into a sequence of edges
    let sequence: Vec<(String, String)> = path
        .windows(2)
        .map(|tuple| (tuple[0].clone(), tuple[1].clone()))
        .collect::<Vec<(String, String)>>();

    sequence
}

#[allow(dead_code)]
fn remove_tiles(puzzle: &mut Vec<Option<(String, String)>>) {
    let mut seed = thread_rng();
    let random = seed.gen_range(0..puzzle.len());
    puzzle[random] = None;
}

#[allow(dead_code)]
fn generate_puzzle(sequence: Vec<(String, String)>) -> Vec<Option<(String, String)>> {
    // Initialize the puzzle (a copy of the sequence)
    let mut puzzle: Vec<Option<(String, String)>> = sequence
        .into_iter()
        .map(Some)
        .collect::<Vec<Option<(String, String)>>>();

    // Remove tiles
    remove_tiles(&mut puzzle);

    puzzle
}

#[allow(dead_code)]
pub fn generate(n: usize) -> Vec<Option<(String, String)>> {
    let solution = generate_solution(n);
    generate_puzzle(solution)
}
