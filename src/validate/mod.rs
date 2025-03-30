use model::{compute_model, variables::create_tileset};

use crate::{
    utils::{get_n, DominoError, Model, Puzzle, ResultTranslator},
    Tile,
};

mod model;

/// Validates a given puzzle solution by computing a model and checking the objective value.
///
/// This function takes a reference to a `Puzzle` and a `Solution`, then performs the following steps:
/// - Computes a string-based model representation using `compute_model()`.
/// - Executes the computed model using `Model::execute()`.
/// - Extracts the objective value from the solver result.
/// - Compares the objective value against the expected missing tile count.
/// - Returns `Ok(())` if the objective value matches the missing tile count, otherwise returns a `DominoError`.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` structure representing the puzzle to be validated.
/// * `solution` - A reference to the `Solution` structure representing the proposed solution.
///
/// # Returns
///
/// * `Ok(())` - If the computed objective value matches the expected number of missing tiles.
/// * `Err(DominoError::ModelError)` - If the model execution fails or the objective value is incorrect.
///
/// # Errors
///
/// This function returns a `DominoError::ModelError` in the following cases:
/// - If `compute_model()` returns an error.
/// - If `Model::execute()` fails to execute the computed model.
/// - If the extracted objective value does not match the expected missing tile count.
pub fn validate_puzzle(puzzle: &Puzzle) -> Result<(), DominoError> {
    // Compute a string-based model representation for the puzzle and solution.
    let string_model = compute_model(puzzle)?;
    println!("string_model: {string_model}");
    // Execute the model to obtain a solver result.
    let solver_result = Model::execute(string_model.clone());

    // Extract the objective value from the solver result.
    // May also see the values of the variables through translator._get_variables() method
    if let Ok(translator) = solver_result {
        // Count the number of missing tiles in the puzzle.
        let missing_tiles = puzzle.0.iter().filter(|tile| tile.is_none()).count() as f64;

        // Validate the objective value against the expected missing tiles count.
        let objective = translator.get_objective();
        if objective == missing_tiles {
            Ok(())
        } else {
            let solution: Vec<Option<Tile>> = model_solution_parse(translator, puzzle).expect("Failed to parse solution");
            Err(DominoError::ModelError(
                  format!("Found another solution: {solution:?}")// with model: {string_model}"),
              ))
        }
    } else {
        Err(DominoError::ModelError(
            "Model failed execution".to_string(),
        ))
    }
}

// Function that given the space of tiles existing for a given puzzle and the result of an lp model having as variables names:
// x_{i,j} where i is the index of the tile in the tileset used and j the position where it got inserted within the puzzle space
// returns the solution computed by the lp model
fn model_solution_parse(
    translator: ResultTranslator,
    puzzle: &Puzzle,
) -> Result<Vec<Option<Tile>>, DominoError> {
    let variables: std::collections::HashMap<String, f64> = translator._get_variables();
    let n: i32 = get_n(puzzle)?;
    let tileset: Vec<(usize, usize)> = create_tileset(n as usize);
    let tileset_digits: usize = (tileset.len() as f32).log10().floor() as usize + 1;
    let sequence_digits: usize = (puzzle.0.len() as f32).log10().floor() as usize + 1;
    let mut solution: Vec<Option<Tile>> = puzzle.0.clone();
    for variable in variables.into_iter().filter(|variable| variable.1 == 1.0) {
        let variable_label: String = variable.0;
        let tile_index: usize = variable_label[1..1 + tileset_digits]
            .parse::<usize>()
            .unwrap();
        let position_index: usize = variable_label
            [1 + tileset_digits..1 + tileset_digits + sequence_digits]
            .parse::<usize>()
            .unwrap();
        solution[position_index] =
            Some((tileset[tile_index].0 as i32, tileset[tile_index].1 as i32).into());
    }
    Ok(solution)
}

#[cfg(test)]
mod tests {

    use super::validate_puzzle;

    #[test]
    fn test_validate_valid_puzzle_with_single_hole() {
        let puzzle = vec![
            Some((0, 0).into()),
            Some((0, 1).into()),
            Some((1, 1).into()),
            Some((1, 2).into()),
            Some((2, 2).into()),
            None,
            None,
            None,
        ];
        println!("Testing valid puzzle with single hole: {:?}", puzzle);
        assert!(validate_puzzle(&puzzle.into()).is_ok());
    }

    #[test]
    fn test_validate_valid_puzzle_with_multiple_holes() {
        let puzzle = vec![
            None,
            Some((0, 1).into()),
            None,
            Some((1, 2).into()),
            Some((2, 2).into()),
            None,
            None,
            None,
        ];
        println!("Testing valid puzzle with multiple holes: {:?}", puzzle);
        assert!(validate_puzzle(&puzzle.into()).is_ok());
    }

    #[test]
    fn test_validate_empty_puzzle() {
        let puzzle = vec![];
        println!("Testing empty puzzle: {:?}", puzzle);
        let result = validate_puzzle(&puzzle.into());
        println!("Validation result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_double_tiles_no_orientation() {
        let puzzle = vec![Some((0, 0).into()), None, None, None, None, None, None];
        println!("Testing double tiles no orientation: {:?}", puzzle);
        let result = validate_puzzle(&puzzle.into());
        println!("Validation result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_single_tile_orientation() {
        let puzzle = vec![Some((0, 1).into()), None, None, None, None, None, None];
        println!("Testing single tile orientation: {:?}", puzzle);
        let result = validate_puzzle(&puzzle.into());
        println!("Validation result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_puzzle_empty() {
        let puzzle = vec![None; 8];
        println!("Testing invalid empty puzzle: {:?}", puzzle);
        let result = validate_puzzle(&puzzle.into());
        println!("Validation result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_puzzle_invalid_size() {
        let puzzle = vec![None; 9];
        let result = validate_puzzle(&puzzle.into());
        println!("Validation result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_puzzle_with_ambiguous_solution() {
        let puzzle = vec![
            Some((0, 0).into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        println!("Testing puzzle with an ambiguous solution: {:?}", puzzle);
        let result = validate_puzzle(&puzzle.into());
        println!("Validation result: {:?}", result);
        assert!(result.is_err());
    }
}
