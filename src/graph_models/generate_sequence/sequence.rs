use std::collections::HashMap;

use crate::graph_models::graph_types::Orientation;
use rand::{thread_rng, Rng};

fn next_vertex(vertexes: Vec<&String>, random: bool) -> String {
    if random {
        let mut seed = thread_rng();
        let index = seed.gen_range(0..vertexes.len());
        vertexes[index].clone()
    } else {
        vertexes[0].clone()
    }
}

fn remove_reverse_edge(
    adj_list: &mut HashMap<String, Vec<(String, Orientation)>>,
    v1: &str,
    v2: &str,
    orientation: Orientation,
) {
    if let Some(neighbors) = adj_list.get_mut(v2) {
        if let Some(pos) = neighbors
            .iter()
            .position(|(adj, orient)| adj == v1 && *orient == -orientation.clone())
        {
            neighbors.remove(pos);
        }
    }
}

fn dfs(
    adj_list: &mut HashMap<String, Vec<(String, Orientation)>>,
    path: &mut Vec<String>,
    vertex: String,
) {
    while let Some((next_vertex, orientation)) = adj_list
        .get_mut(&vertex)
        .and_then(|neighbors| neighbors.pop())
    {
        remove_reverse_edge(adj_list, &vertex, &next_vertex, orientation);
        dfs(adj_list, path, next_vertex);
    }
    path.push(vertex);
}

pub fn as_sequence(
    adj: &HashMap<String, Vec<(String, Orientation)>>,
    random: bool,
) -> Vec<Option<(String, String)>> {
    let mut path = Vec::new();
    let mut adj_list = adj.clone();

    let mut vertexes = adj_list.keys().into_iter().collect::<Vec<&String>>();
    vertexes.sort();
    let start_vertex = next_vertex(vertexes, random);
    dfs(&mut adj_list, &mut path, start_vertex.clone());

    let mut mapped_path = Vec::new();
    for i in 1..path.len() {
        let node1 = &path[i - 1];
        let node2 = &path[i];

        if let Some(neighbors) = adj.get(node1) {
            if let Some((_, orientation)) = neighbors.iter().find(|(adj, _)| adj == node2) {
                if *orientation == Orientation::Positive || *orientation == Orientation::Negative {
                    mapped_path.push(Some((node1.clone(), node2.clone())));
                } else {
                    mapped_path.push(None);
                }
            }
        }
    }

    mapped_path
}
