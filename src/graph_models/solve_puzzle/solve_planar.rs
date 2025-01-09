use crate::graph_models::graph_types::{graph::GraphTrait, pog_graph::PogGraph, Orientation};

pub fn solve_planar(puzzle: Vec<Option<(usize, usize)>>, mut pog: PogGraph) -> Option<Vec<(usize, usize)>> {
    let mut solved = false;
    solve_planar_r(&mut pog, None, &mut solved);
    if solved {
        return Some(flatten_oriented_graph(&puzzle, pog));
    }
    None
}

fn flatten_oriented_graph(puzzle: &Vec<Option<(usize, usize)>>, pog: PogGraph) -> Vec<(usize, usize)> {
    
    let solution = puzzle.clone()
    .into_iter()
    .map(|tile| tile.unwrap())
    .collect();
    solution
}

fn solve_planar_r(pog: &mut PogGraph, last: Option<(String, (String, Orientation))>, solved: &mut bool) {
  if *solved {
      return;
  }

  let edge_to_fix = find_unoriented_edge(&pog);
  if let Some(edge_to_fix) = edge_to_fix {
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
  } else {
      if is_eulerian(&pog) {
          *solved = true;
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

fn find_unoriented_edge(pog: &PogGraph) -> Option<(String, (String, Orientation))> {
  for (node, edges) in pog.adjacency().iter() {
      for edge in edges {
          if edge.1 == Orientation::Zero {
              return Some((node.to_string(), edge.clone()));
          }
      }
  }
  None
}