use std::collections::HashMap;

use crate::domino_types::puzzle::Puzzle;
use crate::graph_models::generate_sequence::sequence::as_sequence;
use crate::graph_models::graph_types::{
    aux_graph::AuxiliaryGraph, di_graph::DirectedGraph, graph::GraphTrait, pog_graph::PogGraph,
    regular_graph::RegularGraph, Orientation,
};
use coloring::lexicographic2_coloring;

mod coloring;

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

pub fn solve(puzzle: &Puzzle) -> Option<Vec<(String, String)>> {
    // Create a pog graph representing the puzzle
    let n = get_n(puzzle);
    let reg = RegularGraph::new(n);
    let mut pog = PogGraph::from(&reg);
    let puzzle: Vec<Option<(usize, usize)>> = puzzle.clone().into();
    for tile in puzzle {
        if let Some(tile) = tile {
            pog.insert_or_update(
                tile.0.to_string(),
                Some((tile.1.to_string(), Orientation::Zero)),
                (tile.1.to_string(), Orientation::Positive),
            );
            pog.insert_or_update(
                tile.1.to_string(),
                Some((tile.0.to_string(), Orientation::Zero)),
                (tile.0.to_string(), Orientation::Negative),
            );
        }
    }
    // Create a directed graph from pog_graph
    let mut arc_graph = DirectedGraph::from(&pog);
    let aux_graph = AuxiliaryGraph::from(&pog);
    let coloring: Option<HashMap<String, i32>> = lexicographic2_coloring(&aux_graph);

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
            .collect::<Vec<(String, String)>>(),
    )
}

#[cfg(test)]
mod test {

    use crate::domino_types::puzzle::Puzzle;
    use crate::graph_models::generate_sequence::generate_solution;
    use crate::graph_models::solve_puzzle::solve;

    #[test]
    fn solve_test() {
        for n in 2..=12 {
            let sequence = generate_solution(n, false);
            let mut puzzle = sequence
                .clone()
                .into_iter()
                .map(|tile| Some((i32::from_str_radix(&tile.0, 10).unwrap() as usize, i32::from_str_radix(&tile.1, 10).unwrap() as usize)))
                .collect::<Vec<Option<(usize, usize)>>>();
            puzzle[0] = None;
            let puzzle = Puzzle::from(puzzle);
            let solved = solve(&puzzle).unwrap();
            assert_eq!(solved, sequence);
        }
    }
}
