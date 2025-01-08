use std::collections::{HashMap, VecDeque};

use crate::graph_models::graph_types::{aux_graph::AuxiliaryGraph, graph::GraphTrait};

fn comes_before(a_str: &String, b_str: &String) -> i32 {
    let a: (String, String) = AuxiliaryGraph::string_to_edge(a_str).unwrap();
    let b: (String, String) = AuxiliaryGraph::string_to_edge(b_str).unwrap();
    if a.0 == b.0 && a.1 < b.1 {
        a.1.parse::<i32>().unwrap() - b.1.parse::<i32>().unwrap()
    } else if a.0 < b.0 {
        a.0.parse::<i32>().unwrap() - b.0.parse::<i32>().unwrap()
    } else {
        b.0.parse::<i32>().unwrap() - a.0.parse::<i32>().unwrap()
    }
}

fn extend_coloring(
    aux_graph: &AuxiliaryGraph,
    color_map: &mut HashMap<String, i32>,
    el: &str,
) -> bool {
    let mut queue = VecDeque::new();
    let empty_vec = vec![];
    let adj_list = aux_graph.adjacency();
    let neighbors = adj_list.get(el).unwrap_or(&empty_vec);

    for node in neighbors {
        if *color_map.get(node).unwrap_or(&-1) == -1 {
            queue.push_back(node.clone());
            color_map.insert(node.clone(), 1);

            while let Some(current_node) = queue.pop_front() {
                let current_color = *color_map.get(&current_node).unwrap();

                for neighbor in adj_list.get(&current_node).unwrap_or(&vec![]) {
                    if *color_map.get(neighbor).unwrap_or(&-1) == -1 {
                        color_map.insert(neighbor.clone(), 1 - current_color);
                        queue.push_back(neighbor.clone());
                    } else if color_map.get(neighbor) == Some(&current_color) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

pub fn lexicographic2_coloring(aux_graph: &AuxiliaryGraph) -> Option<HashMap<String, i32>> {
    let mut coloring: HashMap<String, i32> = aux_graph
        .get_nodes()
        .into_iter()
        .map(|el| (el.clone(), -1))
        .collect();

    let mut lex_ord: Vec<String> = aux_graph.nodes();
    lex_ord
        .to_owned()
        .sort_by(|a, b| comes_before(a, b).cmp(&0));

    while coloring.values().any(|&color| color == -1) {
        let el = lex_ord.remove(0);
        coloring
            .entry(el.clone())
            .and_modify(|color| *color = 0)
            .or_insert(0);
        if !extend_coloring(aux_graph, &mut coloring, &el) {
            return None;
        }
    }

    Some(coloring)
}

// Assume the existence of AuxiliaryGraph and its methods get_adjacency and get_nodes
