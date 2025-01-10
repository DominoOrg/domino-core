use std::collections::HashMap;

use crate::domino_types::puzzle::Puzzle;
use crate::graph_models::generate_sequence::sequence::as_sequence;
use crate::graph_models::graph_types::{
    aux_graph::AuxiliaryGraph, di_graph::DirectedGraph, graph::GraphTrait, pog_graph::PogGraph,
    regular_graph::RegularGraph, Orientation,
};
use coloring::lexicographic2_coloring;
use solve_planar::solve_planar;

mod coloring;
mod solve_planar;
mod hierholzer;

pub fn get_n(puzzle: &Puzzle) -> usize {
    let l = puzzle.len();
    let n_p = (-3.0 + (1.0 + 8.0 * (l as f64)).sqrt()) / 2.0;
    let n_d = (-2.0 + (8.0 * (l as f64)).sqrt()) / 2.0;
    let n = if (n_p - n_p.floor()).abs() == 0.0 {
        n_p.floor() as usize
    } else {
        n_d.floor() as usize
    };
    n
}

fn solve_non_planar(
    puzzle: Vec<Option<(usize, usize)>>,
    pog: PogGraph,
) -> Option<Vec<(usize, usize)>> {
    let _ = puzzle;
    // Create a directed graph from pog_graph
    let mut arc_graph = DirectedGraph::from(&pog);
    let aux_graph = AuxiliaryGraph::from(&pog);
    let coloring: Option<HashMap<String, i32>> = lexicographic2_coloring(&pog, &aux_graph);
    println!("{coloring:?}");

    // If coloring is None, return None
    let coloring = coloring?;

    // Iterate over nodes with color == 0 and update arc_graph accordingly
    for (node, _) in coloring.iter().filter(|&(_, &color)| color == 0) {
        let (u, v) = AuxiliaryGraph::string_to_edge(&node)?;
        arc_graph.insert_or_update(
            u.clone(),
            Some((v.clone(), Orientation::Zero)),
            (v.clone(), Orientation::Positive),
        );
        arc_graph.insert_or_update(
            v.clone(),
            Some((u.clone(), Orientation::Zero)),
            (u.clone(), Orientation::Negative),
        );
    }
    Some(
        as_sequence(&arc_graph.adjacency(), false)
            .into_iter()
            .map(|tile| tile.unwrap())
            .collect::<Vec<(usize, usize)>>(),
    )
}

pub fn solve(puzzle: &Puzzle) -> Option<Vec<(usize, usize)>> {
    // Create a pog graph representing the puzzle
    println!("puzzle: {puzzle}");
    let n = get_n(puzzle);
    println!("n: {n}");
    let reg = RegularGraph::new(n);
    println!("reg: {reg:?}");
    let mut pog = PogGraph::from(&reg);
    let puzzle: Vec<Option<(usize, usize)>> = puzzle.clone().into();

    for tile in puzzle.iter() {
        if let Some(tile) = tile {
            pog.insert_or_update(
                tile.0,
                Some((tile.1, Orientation::Zero)),
                (tile.1, Orientation::Positive),
            );
            pog.insert_or_update(
                tile.1,
                Some((tile.0, Orientation::Zero)),
                (tile.0, Orientation::Negative),
            );
        }
    }
    println!("pog: {pog:?}");

    // If the pog has less than 5 nodes brute force the completion until it's not complete and each node has even degree
    // Else if the pog has more than 5 nodes use lexicographic coloring
    if pog.nodes().len() <= 5 {
        solve_planar(puzzle.clone(), pog)
    } else {
        solve_non_planar(puzzle, pog)
    }
}
