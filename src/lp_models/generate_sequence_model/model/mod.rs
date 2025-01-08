use helpers::sorting_label;
use itertools::Itertools;
use rand::{thread_rng, Rng};

use crate::lp_models::generate_sequence_model::model::bounds::{
    each_position_filled_bound, each_tile_used_once_bound, next_adjacent_bound,
};
use crate::lp_models::generate_sequence_model::model::variables::{variables, Variables};
use crate::stringify_variables;

mod bounds;
mod helpers;
pub mod variables;

fn bounds(vars: &Variables, sequence_length: usize) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    prob_bounds.extend(each_tile_used_once_bound(vars));
    prob_bounds.extend(each_position_filled_bound(vars));
    prob_bounds.extend(next_adjacent_bound(vars, sequence_length));

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

pub fn compute_model(n: usize, random: bool) -> String {
    let sequence_length = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        ((n + 1).pow(2) / 2) as usize
    };

    // Assuming variables, objective_function, bounds, and stringify_variables functions are defined
    let prob_vars = variables(n);
    let prob_obj = objective_function(&prob_vars);
    let mut prob_bounds = bounds(&prob_vars, sequence_length);
    let ordered_vars: Vec<&String> = prob_vars
        .by_label
        .keys()
        .sorted_by(|label1, label2| sorting_label(label1, label2))
        .collect();

    // If random is true set an additional bound for a random variable to 1
    if random {
        let mut seed = thread_rng();
        let variable = prob_vars.vars[seed.gen_range(0..prob_vars.vars.len())].clone();
        let label = variable.label;
        let bound = label + " = 1";
        prob_bounds.push(bound);
    }

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
    return model;
}
