use model::compute_model;
use crate::{lps::Model, DominoError, Puzzle, Solution};

mod model;

pub fn validate_puzzle(puzzle: &Puzzle, solution: &Solution) -> Result<(), DominoError> {
    let string_model = compute_model(puzzle, solution)?;
    let solver_result = Model::execute(string_model);
    let objective_value = solver_result.map(|translator| translator.get_objective());
    let missing_tiles = puzzle.iter()
    .filter(|tile| tile.is_none())
    .count() as f64;
    if let Ok(objective) = objective_value {
        if objective == missing_tiles {
            Ok(())
        } else {
            Err(DominoError::ModelError("Invalid objective value".to_string()))
        }
    } else {
        Err(DominoError::ModelError("Model failed execution".to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::validate_puzzle;

    #[test]
    fn test_validate() {
        let mut puzzle = vec![];
        let mut solution = vec![];
        assert!(validate_puzzle(&puzzle, &solution).is_err());
        puzzle = vec![Some((0,0).into()), Some((0,1).into()), Some((1,1).into()), Some((1,2).into()), Some((2,2).into()), None, None, None];
        solution = vec![(0,0).into(), (0,1).into(), (1,1).into(), (1,2).into(), (2,2).into(), (2,3).into(), (3,3).into(), (3,0).into()];
        assert!(validate_puzzle(&puzzle, &solution).is_ok());
        puzzle = vec![None, Some((0,1).into()), None, Some((1,2).into()), Some((2,2).into()), None, None, None];
        solution = vec![(0,0).into(), (0,1).into(), (1,1).into(), (1,2).into(), (2,2).into(), (2,3).into(), (3,3).into(), (3,0).into()];
        assert!(validate_puzzle(&puzzle, &solution).is_ok());
        puzzle = vec![None; 8];
        solution = vec![(0,0).into(), (0,1).into(), (1,1).into(), (1,2).into(), (2,2).into(), (2,3).into(), (3,3).into(), (3,0).into()];
        assert!(validate_puzzle(&puzzle, &solution).is_err());
    }

}
