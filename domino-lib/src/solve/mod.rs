mod model;

use model::{compute_model, variables::create_tileset};

use crate::{
    utils::{get_n, Model},
    DominoError, Puzzle, Solution, Tile,
};

/// Attempts to solve a given puzzle using a recursive backtracking approach.
///
/// This function clones the puzzle, determines the missing tiles, and then attempts to solve
/// the puzzle by filling in the missing tiles while ensuring valid adjacency constraints.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` structure representing the current puzzle state.
///
/// # Returns
///
/// * `Ok(Solution)` - If a valid solution is found.
/// * `Err(DominoError::UnsolvablePuzzle)` - If no solution exists.
/// * `Err(DominoError::InvalidPuzzle)` - If the puzzle input is invalid.
pub fn solve_puzzle(puzzle: &Puzzle) -> Result<Solution, DominoError> {
    let model_string = compute_model(puzzle)?;
    // println!("Model: {}", model_string);
    // Execute the model to obtain a solver result.
    let solver_result = Model::execute(model_string.clone());

    let n = get_n(puzzle)?;
    let tileset: Vec<Tile> = create_tileset(n as usize)
        .iter()
        .map(|tuple| Tile((*tuple).0 as i32, (*tuple).1 as i32).into())
        .collect();
    let tileset_digits = (tileset.len() as f32).log10().floor() as usize + 1;
    let sequence_digits = (puzzle.0.len() as f32).log10().floor() as usize + 1;
    if let Ok(translator) = solver_result {
        let mut solution = puzzle.clone();
        let variables = translator._get_variables();
        let labels: Vec<String> = variables
            .iter()
            .filter_map(|entry: (&String, &f64)| {
                if entry.1 == &1.0 {
                    Some(entry.0.clone())
                } else {
                    None
                }
            })
            .collect();
        // println!("Labels: {labels:?}");
        labels.iter().for_each(|label| {
            let tile_index: usize = label[1..1 + tileset_digits].parse().unwrap();
            let position_index: usize = label
                [1 + tileset_digits..1 + tileset_digits + sequence_digits]
                .parse()
                .unwrap();
            solution.0[position_index] = Some(tileset[tile_index])
        });
        Ok(solution.0.iter().map(|option| option.unwrap()).collect())
    } else {
        Err(DominoError::ModelError(
            "Model failed execution".to_string(),
        ))
    }
}
