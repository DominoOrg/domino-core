use crate::{lp_models::generate_sequence_model::model::variables::Variables, stringify_variables};

use super::helpers::{collect_labels, create_bound_string, fetch_adjacent_variables};

// Function to ensure each tile is used exactly once
pub fn each_tile_used_once_bound(vars: &Variables) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    for variables in vars.by_tile.values() {
        let first_var = variables.get(0).unwrap(); // Directly unwrap the first variable

        let rotated_tile = (first_var.tile.1, first_var.tile.0);
        let empty_vec = vec![];
        let unwrapped_rotated_vars = vars.by_tile.get(&rotated_tile).unwrap_or(&empty_vec);
        let mut remapped_vars = collect_labels(variables);

        // Check and extend only if the tiles are different
        if first_var.tile.0 != first_var.tile.1 {
            remapped_vars.extend(collect_labels(unwrapped_rotated_vars));
        }

        let bound = create_bound_string(remapped_vars);
        prob_bounds.push(bound);
    }

    prob_bounds.push(String::new());
    prob_bounds
}

// Function to ensure each position is filled exactly once
pub fn each_position_filled_bound(vars: &Variables) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    for variables in vars.by_position.values() {
        let labels = collect_labels(variables);
        let bound = create_bound_string(labels);
        prob_bounds.push(bound);
    }

    prob_bounds.push(String::new());
    prob_bounds
}

// Function to ensure adjacency of next tiles
pub fn next_adjacent_bound(vars: &Variables, sequence_length: usize) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    for variable in vars.by_label.values() {
        if variable.position == sequence_length - 1 {
            continue;
        }

        let adjacent_vars =
            fetch_adjacent_variables(vars, variable.tile, variable.position, 1, sequence_length);

        let adjacent_vars_str = if !adjacent_vars.is_empty() {
            stringify_variables!(adjacent_vars, " - ")
        } else {
            "0".to_string()
        };

        let bound = format!("{} - {} <= 0", variable.label, adjacent_vars_str);
        prob_bounds.push(bound);
    }

    prob_bounds.push(String::new());
    prob_bounds
}
