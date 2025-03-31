mod hamiltonian;
mod hamiltonians;

use crate::utils::find_eulerian_cycle;
use crate::{classify_puzzle, ComplexityClass, DominoError};
use crate::{Graph, Puzzle, Solution, Tile, Tournament};
use hamiltonians::compute_hamiltonian_cycles;
use std::vec;

/// Represents the puzzle parameters and solution
#[derive(Clone)]
struct PuzzleData {
    graph: Graph,
    tournament: Option<Tournament>,
    solution: Solution,
    puzzle: Option<Puzzle>,
    n: usize,
    c: ComplexityClass,
}

/// Orchestrates the puzzle generation flow using currying
pub fn generate_valid_puzzle(n: usize) -> Box<dyn Fn(usize) -> Result<Puzzle, DominoError>> {
    Box::new(move |c| {
        ComplexityClass::new(c).and_then(|valid_c| {
            validate_input(n)(valid_c)
                .map(generate_solution)
                .map(|puzzle_data| {
                    let solution = puzzle_data.solution.clone();
                    (puzzle_data, Tournament::new(solution).unwrap())
                })
                .map(|(puzzle_data, tournament)| PuzzleData {
                    tournament: Some(tournament),
                    ..puzzle_data
                })
                .map(generate_puzzle)
                .map(refine_puzzle)
        })
    })
}

/// Validates input parameters using currying (one input function)
fn validate_input(n: usize) -> Box<dyn Fn(ComplexityClass) -> Result<PuzzleData, DominoError>> {
    Box::new(move |c| {
        if n < 1 {
            Err(DominoError::InvalidLength)
        } else {
            Ok(PuzzleData {
                graph: Graph::regular(n),
                tournament: None,
                solution: vec![], // Empty solution initially
                puzzle: None,     // Empty puzzle initially
                c,
                n,
            })
        }
    })
}

// Upates the puzzle until it does not match the required complexity
fn refine_puzzle(puzzle_data: PuzzleData) -> Puzzle {
    let mut puzzle = puzzle_data.puzzle.unwrap().0;
    let expected_complexity_class = puzzle_data.c.0;
    let mut actual_complexity_class = 3;
    let mut first_iteration = true;

    while actual_complexity_class < expected_complexity_class {
        let index = puzzle_data
            .solution
            .iter()
            .position(|&t| !puzzle.contains(&Some(t)))
            .unwrap();
        puzzle[index] = Some(puzzle_data.solution[index]);
        //println!("puzzle: {puzzle:?}");
        if !first_iteration {
            actual_complexity_class = classify_puzzle(&puzzle.clone().into()).unwrap().0;
        }
        first_iteration = false;
        //println!("actual_complexity_class: {actual_complexity_class}");
    }

    puzzle.into()
}

/// Reinserts one tile per Hamiltonian path, reinserts all the double tiles and returns updated PuzzleData
fn generate_puzzle(puzzle_data: PuzzleData) -> PuzzleData {
    let mut puzzle: Puzzle = vec![None; puzzle_data.solution.len()].into();
    let hamiltonians = compute_hamiltonian_cycles(&puzzle_data);
    // println!("hamiltonians: {hamiltonians:?}");
    let hamiltonians = hamiltonians.unwrap();

    hamiltonians.iter().for_each(|hamiltonian| {
        let tile_to_reinsert: Tile = hamiltonian
            .windows(2)
            .map(|couple| Tile(couple[0], couple[1]))
            .next()
            .unwrap();
        let index = puzzle_data
            .solution
            .iter()
            .position(|&t| t == tile_to_reinsert)
            .unwrap();
        puzzle.0[index] = Some(tile_to_reinsert);
    });

    if puzzle_data.n >= 4 {
        let double_tiles: Vec<&Tile> = puzzle_data
            .solution
            .iter()
            .filter(|tile| tile.0 == tile.1)
            .collect();

        double_tiles.iter().for_each(|tile| {
            let index = puzzle_data
                .solution
                .iter()
                .position(|&t| t == **tile)
                .unwrap();
            puzzle.0[index] = Some(**tile);
        });
    }

    PuzzleData {
        puzzle: Some(puzzle),
        ..puzzle_data
    }
}

/// Generates a solution using Hierholzer's algorithm
fn generate_solution(puzzle_data: PuzzleData) -> PuzzleData {
    let cycle = find_eulerian_cycle(&puzzle_data.graph)(true);
    let sequence = cycle
        .windows(2)
        .map(|arc| {
            Tile(
                arc[0].clone().try_into().unwrap(),
                arc[1].clone().try_into().unwrap(),
            )
        })
        .collect();

    PuzzleData {
        solution: sequence,
        ..puzzle_data
    }
}

mod tests {

    #[test]
    fn it_works() {
      use crate::solve_puzzle;
      use crate::{validate_puzzle, generate_valid_puzzle, NUMBER_OF_CLASSES};
        const RETRIALS: usize = 10;
        (3..=6).into_iter().for_each(|n| {
            (1..=NUMBER_OF_CLASSES).into_iter().rev().for_each(|c| {
                (0..=RETRIALS).into_iter().for_each(|_| {
                    println!("Generating puzzle for n = {n} and c = {c}");
                    let puzzle_result = generate_valid_puzzle(n)(c);
                    println!("puzzle_result: {puzzle_result:?}");
                    assert_eq!(puzzle_result.is_ok(), true, "puzzle should be valid");
                    let puzzle = puzzle_result.unwrap();
                    let solution = solve_puzzle(&puzzle).unwrap();
                    let _ = validate_puzzle(&puzzle, &solution);
                });
            });
        });
    }
}
