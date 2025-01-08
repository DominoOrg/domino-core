use crate::model_execution_lib::execute;
use model::compute_model;
use crate::domino_types::{puzzle::{FitPuzzleChecker, Puzzle}, solution::SolutionBuilder, tile::Tile, tileset::Tileset};

mod model;

fn free_tiles(puzzle: &Puzzle, n: usize) -> Vec<Tile> {
    let tileset: Tileset = Tileset::new(n);
    let mut free_tiles = Vec::new();

    for tile in tileset {
        if !puzzle.contains(tile).is_some() {
            free_tiles.push(tile);
        }
    }

    free_tiles
}

fn free_positions(puzzle: &Puzzle) -> Vec<usize> {
    let mut free_positions = Vec::new();

    for position in 0..puzzle.len() {
        if puzzle.at(position).is_none() {
            free_positions.push(position);
        }
    }

    free_positions
}

pub fn validate(puzzle: &Puzzle, n: usize) -> Result<(), String> {
    let mut solutions: Vec<Puzzle> = Vec::new();
    let free_tiles = free_tiles(puzzle, n);
    let free_positions = free_positions(puzzle);
    let checker = FitPuzzleChecker{};
    for tile in free_tiles {
        for position in &free_positions {
            if checker.check(puzzle, *position, tile) {
                let mut new_puzzle = puzzle.clone();
                new_puzzle.insert(*position, tile);
                let model = compute_model(&new_puzzle, n);
                let result = execute(model);
                if let Ok(variables_map) = result {
                    let solution_builder = SolutionBuilder{};
                    let solution = solution_builder.build(&new_puzzle, variables_map, n);
                    solutions.push(solution);
                } else {
                    let error_message = format!("The new_puzzle has no solutions");
                    return Err(error_message);
                }
                new_puzzle.remove(*position);
            }
        }
    }

    for solution in solutions.iter() {
        for solution2 in solutions.iter() {
            if solution != solution2 {
                let error_message = format!("Found multiple solutions to the same puzzle, the puzzle is not valid");
                return Err(error_message);
            }    
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use crate::lp_models::generate_sequence_model::generate_sequence;
    use crate::domino_types::puzzle::Puzzle;
    use crate::lp_models::validate_puzzle_models::validate;

    #[test]
    fn validate_valid_puzzle() {
        for n in 2..=4 {
            let sequence = generate_sequence(n, false).unwrap();
            let mut puzzle = sequence
                .into_iter()
                .map(|tile| Some(tile))
                .collect::<Vec<Option<(usize, usize)>>>();
            puzzle[0] = None;
            let puzzle = Puzzle::from(puzzle);
            assert!(validate(&puzzle, n).is_ok());
        }
    }

    #[test]
    fn validate_invalid_puzzle() {
        for n in 2..=4 {
            let sequence = generate_sequence(n, false).unwrap();
            let mut puzzle = sequence
                .into_iter()
                .map(|tile| Some(tile))
                .collect::<Vec<Option<(usize, usize)>>>();
            for i in 0..puzzle.len() {
                puzzle[i] = None;
            }
            let puzzle = Puzzle::from(puzzle);
            assert!(validate(&puzzle, n).is_err());
        }
    }
}
