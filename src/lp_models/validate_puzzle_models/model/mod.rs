use helpers::sorting_label;
use itertools::Itertools;

use crate::lp_models::validate_puzzle_models::model::bounds::{
    each_position_filled_bound, each_tile_used_once_bound, next_adjacent_bound,
};
use crate::lp_models::validate_puzzle_models::model::variables::{variables, Variables};
use crate::{lp_models::validate_puzzle_models::Puzzle, stringify_variables};

mod bounds;
mod helpers;
pub mod variables;

fn bounds(puzzle: &Puzzle, vars: &Variables, sequence_length: usize) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    prob_bounds.extend(each_tile_used_once_bound(vars));
    prob_bounds.extend(each_position_filled_bound(vars));
    prob_bounds.extend(next_adjacent_bound(puzzle, vars, sequence_length));

    prob_bounds
}

fn objective_function(vars: &Variables) -> String {
    let labels = vars
        .vars
        .iter()
        .map(|variable| variable.label.clone())
        .collect::<Vec<String>>();
    let obj = stringify_variables!(labels, " ");
    obj
}

pub fn compute_model(puzzle: &Puzzle, n: usize) -> String {
    let sequence_length = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        ((n + 1).pow(2) / 2) as usize
    };

    // Assuming variables, objective_function, bounds, and stringify_variables functions are defined
    let prob_vars = variables(puzzle, n);
    let prob_obj = objective_function(&prob_vars);
    let prob_bounds = bounds(puzzle, &prob_vars, sequence_length);
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
    println!("{}", model);
    return model;
}
