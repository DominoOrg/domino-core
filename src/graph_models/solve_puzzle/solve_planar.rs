use crate::graph_models::graph_types::{graph::GraphTrait, pog_graph::PogGraph, under_graph::UnderlyingGraph, GraphNode, Orientation};
use super::hierholzer::hierholzer;

pub fn solve_planar(puzzle: Vec<Option<(usize, usize)>>, pog: PogGraph) -> Option<Vec<(usize, usize)>> {
    let mut solved = false;
    solve_planar_r(&mut pog.clone(), None, &mut solved);
    if solved {
        return Some(flatten_oriented_graph(&puzzle, pog));
    }
    None
}

fn flatten_oriented_graph(puzzle: &Vec<Option<(usize, usize)>>, pog: PogGraph) -> Vec<(usize, usize)> {
    println!("pog: {pog:?}");
    let mut fixed_puzzle = puzzle.clone();
    println!("fixed_puzzle: {fixed_puzzle:?}");
    while fixed_puzzle.clone().into_iter().any(|el| el.is_none()) {
        println!("new iteration to fix the puzzle");
        // Count consecutive missing tiles
        let mut len = 0;
        let mut start_node = None;
        let mut last_none = None;
        for i in 0..puzzle.len() {
            if puzzle[i].is_some() && puzzle[(i+1)%puzzle.len()].is_none() {
                start_node = Some(puzzle[i].unwrap().1);
                last_none = None;
                len = 0;
            }
        }

        for i in 0..puzzle.len() {
            if puzzle[i].is_none() {
                last_none = Some(i);
                len += 1;
                if puzzle[(i+1)%puzzle.len()].is_some() {
                    break;
                }
            }
        }
        println!("len: {len}");
        println!("start_node: {start_node:?}");
        println!("last_none: {last_none:?}");
        // Replace the consecutive none with the solution
        if len > 0 && start_node.is_some() {
            let under_graph = UnderlyingGraph::from(&pog);
            println!("under_graph: {under_graph:?}");
            let solution: Vec<(usize, usize)> = hierholzer(&under_graph, start_node.unwrap(), len).unwrap();
            println!("solution: {solution:?}");
            let start_index = if last_none.unwrap() >= len {
                last_none.unwrap() - len
            } else {
                (2 * len - last_none.unwrap())%puzzle.len() 
            };
            for i in start_index..last_none.unwrap() {
                fixed_puzzle[i as usize] = Some(solution[(i-last_none.unwrap()) as usize]);
            }
        }

    }
    fixed_puzzle.into_iter().map(|tile| tile.unwrap()).collect()
}

fn solve_planar_r(pog: &mut PogGraph, last: Option<(GraphNode, (GraphNode, Orientation))>, solved: &mut bool) {
    println!("Recurring to solve planar");
    println!("pog: {pog:?}");
    println!("solved: {solved}");
    if *solved {
        return;
    }

    let edge_to_fix = find_unoriented_edge(&pog);
    if let Some(edge_to_fix) = edge_to_fix {
        if edge_to_fix.0 == edge_to_fix.1.0 {
            pog.insert_or_update(edge_to_fix.0, None, (edge_to_fix.1.0, Orientation::Zero));
        }
        pog.orient_arc(&edge_to_fix.0, &edge_to_fix.1.0);
        solve_planar_r(pog, Some((edge_to_fix.0.clone(), edge_to_fix.1.clone())), solved);
        if *solved {
            return;
        }
        pog.deorient_arc(&edge_to_fix.0, &edge_to_fix.1.0);
        pog.orient_arc(&edge_to_fix.1.0, &edge_to_fix.0);
        solve_planar_r(pog, Some((edge_to_fix.0.clone(), edge_to_fix.1.clone())), solved);
        if *solved {
            return;
        }
        pog.deorient_arc(&edge_to_fix.1.0, &edge_to_fix.0);
        if edge_to_fix.0 == edge_to_fix.1.0 {
            pog.remove_edge(&edge_to_fix.0, &(edge_to_fix.1.0.clone(), Orientation::Zero));
        }
    } else {
        if is_eulerian(&pog) {
            *solved = true;
            println!("Solved");
        } else {
            if let Some(last) = last {
                pog.deorient_arc(&last.0, &last.1.0);
                return;
            }
        }
    }
}

fn is_eulerian(pog: &PogGraph) -> bool {
    pog.adjacency()
    .iter()
    .all(|(_node, edges)| {
        let node_degree = edges
        .iter()
        .map(|edge| match edge.1 {
            Orientation::Positive => 1,
            Orientation::Negative => -1,
            Orientation::Zero => 0,
        })
        .reduce(|acc, el| acc + el)
        .unwrap();
        node_degree == 0
    })
}

fn find_unoriented_edge(pog: &PogGraph) -> Option<(GraphNode, (GraphNode, Orientation))> {
    for (node, edges) in pog.adjacency().iter() {
        for edge in edges {
            if edge.1 == Orientation::Zero {
                return Some((node.clone(), edge.clone()));
            }
        }
    }
    None
}