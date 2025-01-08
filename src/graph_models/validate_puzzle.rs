use crate::domino_types::puzzle::Puzzle;
use crate::domino_types::tile::Tile;
use crate::domino_types::tileset::Tileset;
use crate::graph_models::solve_puzzle::solve;

use super::solve_puzzle::get_n;

pub fn validate(puzzle: &Puzzle) -> Result<(), String> {
  // Check the puzzle has a solution
  println!("Starting solver on {puzzle}...");
  let solution = solve(puzzle);
  if solution.is_none() { return Err("No solution found".to_string()); }
  println!("Solved: {solution:?}");

  // Check forcing different position for the unused tiles that there is no solution or the solution is the same to the first
  let n = get_n(puzzle);
  let tileset = Tileset::new(n);
  let free_tiles = tileset.into_iter().filter(|tile| !puzzle.vec.contains(&Some(*tile))).collect::<Vec<Tile>>();
  let solution = solution.unwrap();
  let solution_tiles = solution.into_iter().map(|tile| Tile::try_from(tile).unwrap()).collect::<Vec<Tile>>();
  println!("solution_tiles: {solution_tiles:?}");

  for tile in free_tiles {
    for hole in puzzle.vec.iter().enumerate().filter(|tile| tile.1.is_none()).map(|tile| tile.0) {
      let mut altered_puzzle = puzzle.clone();
      altered_puzzle.insert(hole, Some(tile));
      println!("Starting solver...");
      let altered_solution = solve(&altered_puzzle);
      if let Some(altered_solution) = altered_solution {
        println!("Solved: {altered_solution:?}");
        let altered_solution_tiles = altered_solution.into_iter().map(|tile| Tile::try_from(tile).unwrap()).collect::<Vec<Tile>>();
        println!("altered_solution_tiles: {altered_solution_tiles:?}");
        if altered_solution_tiles != solution_tiles.clone() {
          return Err("Puzzle has multiple solutions".to_string());
        }          
      }
      else {
        println!("This alteration has no solution");
      }
    }
  }
  

  Ok(())
}