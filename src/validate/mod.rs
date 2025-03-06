use model::compute_model;

use crate::utils::{DominoError, Model, Puzzle, Solution};

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
///
/// # Examples
///
/// ```rust
/// use domino_lib::{generate_puzzle, solve_puzzle};
///
/// let puzzle = generate_puzzle(3, 1, false);  // Assuming `Puzzle::new()` initializes a valid puzzle.
/// let solution = solve_puzzle(&puzzle);  // Assuming `Solution::new()` initializes a valid solution.
/// match validate_puzzle(&puzzle, &solution) {
///     Ok(_) => println!("Solution is valid."),
///     Err(e) => println!("Solution is invalid: {:?}", e),
/// }
/// ```
pub fn validate_puzzle(puzzle: &Puzzle, solution: &Solution) -> Result<(), DominoError> {
  // Compute a string-based model representation for the puzzle and solution.
  let string_model = compute_model(puzzle, solution)?;

  // Execute the model to obtain a solver result.
  let solver_result = Model::execute(string_model.clone());

  // Extract the objective value from the solver result.
  // May also see the values of the variables through translator._get_variables() method
  let objective_value = solver_result.map(|translator| translator.get_objective());

  // Count the number of missing tiles in the puzzle.
  let missing_tiles = puzzle.iter().filter(|tile| tile.is_none()).count() as f64;

  // Validate the objective value against the expected missing tiles count.
  if let Ok(objective) = objective_value {
      if objective == missing_tiles {
          Ok(())
      } else {
          Err(DominoError::ModelError(
              "Invalid objective value".to_string(),
          ))
      }
  } else {
      Err(DominoError::ModelError(
          "Model failed execution".to_string(),
      ))
  }
}


#[cfg(test)]
mod tests {

    use super::validate_puzzle;

    #[test]
    fn test_validate() {
        // Invalid puzzle: Empty puzzle is not valid
        let mut puzzle = vec![];
        let mut solution = vec![];
        assert!(validate_puzzle(&puzzle, &solution).is_err());

        // Invalid puzzle: Double tiles do not imply any orientation of the eulerian cycle
        puzzle = vec![Some((0, 0).into()), None, None, None, None, None, None];
        solution = vec![
            (0, 0).into(),
            (0, 1).into(),
            (1, 1).into(),
            (1, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
        ];
        assert!(validate_puzzle(&puzzle, &solution).is_err());

        // Valid puzzle: One single tile that determines the orientation of the eulerian cycle
        puzzle = vec![Some((0, 1).into()), None, None, None, None, None, None];
        solution = vec![
            (0, 1).into(),
            (1, 1).into(),
            (1, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
            (0, 0).into(),
        ];
        assert!(validate_puzzle(&puzzle, &solution).is_err());

        // Valid puzzle with a single hole
        puzzle = vec![
            Some((0, 0).into()),
            Some((0, 1).into()),
            Some((1, 1).into()),
            Some((1, 2).into()),
            Some((2, 2).into()),
            None,
            None,
            None,
        ];
        solution = vec![
            (0, 0).into(),
            (0, 1).into(),
            (1, 1).into(),
            (1, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
        ];
        assert!(validate_puzzle(&puzzle, &solution).is_ok());

        // Valid puzzle with multiple holes
        puzzle = vec![
            None,
            Some((0, 1).into()),
            None,
            Some((1, 2).into()),
            Some((2, 2).into()),
            None,
            None,
            None,
        ];
        solution = vec![
            (0, 0).into(),
            (0, 1).into(),
            (1, 1).into(),
            (1, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
        ];
        assert!(validate_puzzle(&puzzle, &solution).is_ok());

        // Invalid puzzle for emptyness
        puzzle = vec![None; 8];
        solution = vec![
            (0, 0).into(),
            (0, 1).into(),
            (1, 1).into(),
            (1, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
        ];
        assert!(validate_puzzle(&puzzle, &solution).is_err());

        // Invalid puzzle for invalid size
        puzzle = vec![None; 9];
        solution = vec![
            (0, 0).into(),
            (0, 1).into(),
            (1, 1).into(),
            (1, 2).into(),
            (2, 2).into(),
            (2, 3).into(),
            (3, 3).into(),
            (3, 0).into(),
            (0, 0).into(),
        ];
        assert!(validate_puzzle(&puzzle, &solution).is_err());
    }
}
