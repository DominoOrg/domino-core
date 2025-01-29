mod model;

use model::compute_model;

use crate::{lps::execute, Puzzle};

pub fn validate(puzzle: &Puzzle, n: usize) -> Result<(), String> {
    let model = compute_model(puzzle, n);
    println!("{}", model);
    let result = execute(model);
    if let Ok(_) = result {
        Ok(())
    } else {
        Err(result.err().unwrap())
    }
}