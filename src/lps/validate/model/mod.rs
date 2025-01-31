use helpers::sorting_label;
use itertools::Itertools;

use super::model::bounds::{
    each_position_filled_bound, each_tile_used_once_bound, next_adjacent_bound,
};
use super::model::variables::{variables, Variables};
use crate::{stringify_variables, DominoError, Puzzle, Solution};

mod bounds;
mod helpers;
pub mod variables;

fn bounds(puzzle: &Puzzle, vars: &Variables) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    prob_bounds.extend(each_tile_used_once_bound(vars));
    prob_bounds.extend(each_position_filled_bound(vars));
    prob_bounds.extend(next_adjacent_bound(puzzle, vars));

    prob_bounds
}

fn objective_function(vars: &Variables, puzzle: &Puzzle, solution: &Solution) -> String {
    let labels = solution
    .clone()
    .into_iter()
    .enumerate()
    .filter_map(|(i, tile)| {
        if puzzle[i].is_some() {
            return None;
        }
        let tile: (usize, usize) = (tile.0.try_into().unwrap(), tile.1.try_into().unwrap());
        if let Some(tiles) = vars.by_tile.get(&tile) {
            let variable = tiles.iter().filter(|var| var.position == i).next()?;
            Some(variable.label.clone())
        } else {
            None
        }
    })
    .collect::<Vec<String>>();
    let obj = stringify_variables!(labels, " ");
    obj
}

pub fn compute_model(puzzle: &Puzzle, solution: &Solution) -> Result<String, DominoError> {
    // Assuming variables, objective_function, bounds, and stringify_variables functions are defined
    let prob_vars = variables(puzzle)?;
    let prob_obj = objective_function(&prob_vars, puzzle, solution);
    let prob_bounds = bounds(puzzle, &prob_vars);
    let ordered_vars: Vec<&String> = prob_vars
        .by_label
        .keys()
        .sorted_by(|label1, label2| sorting_label(label1, label2))
        .collect();

    // Building the model string
    let mut model = "Minimize\n".to_string();
    model.push_str(format!(" obj: {}\n", prob_obj).as_str());
    model.push_str("Subject To\n");
    for (i, constraint) in prob_bounds.iter().enumerate() {
        if !constraint.is_empty() {
            model.push_str(format!(" c{}: {}\n", i, constraint).as_str());
        }
    }
    model.push_str("Binary\n");
    for variable in ordered_vars {
        model.push_str(format!(" {}\n", variable).as_str());
    }
    model.push_str("End");
    println!("Model: {}", model);
    return Ok(model);
}
