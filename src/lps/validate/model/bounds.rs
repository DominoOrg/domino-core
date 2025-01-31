use crate::{graphs::get_n, Tile};

use super::{helpers::{collect_labels, create_bound_string}, stringify_variables, variables::{Variable, Variables}, Puzzle};

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
pub fn next_adjacent_bound(puzzle: &Puzzle, vars: &Variables) -> Vec<String> {
    let mut prob_bounds = Vec::new();
    let n = get_n(puzzle).expect("Puzzle does not have a valid length") as usize;

    for position in 0..puzzle.len() {
        if puzzle[position].is_some() {
            continue;
        }
        if let Some(tile) = puzzle[(position + 1) % puzzle.len()] {
            
            if let Some(bound) = next_enforced_bound(vars, tile, position) {
                prob_bounds.push(bound);                
            }

        } else {

            for number in 0..=n {
                if let Some(bound) = next_bound(puzzle, vars, position, number) {
                    prob_bounds.push(bound);
                }
            }
        }
    }

    prob_bounds.push(String::new());
    prob_bounds
}

fn next_enforced_bound (vars: &Variables, tile: Tile, position: usize) -> Option<String> {
    let number = tile.0 as usize;
    let condition = |var: &Variable| var.tile.1 == number.try_into().unwrap();
    let left_member_variables: Vec<String> = variables_at_position_with_condition(vars, position, condition);
    if left_member_variables.len() == 0 {
        return None;
    }
    let bound = 
        format!("{} = 1",
            stringify_variables!(left_member_variables, " + "),
        );
    Some(bound)
}

fn next_bound(puzzle: &Puzzle, vars: &Variables, position: usize, number: usize) -> Option<String> {
    let condition = |var: &Variable| var.tile.1 == number.try_into().unwrap();
    let left_member_variables: Vec<String> = variables_at_position_with_condition(vars, position, condition);
    let next_position = (position + 1) % puzzle.len();
    let condition = |var: &Variable| var.tile.0 == number.try_into().unwrap();
    let right_member_variables: Vec<String> = variables_at_position_with_condition(vars, next_position, condition);
    if left_member_variables.len() == 0 {
        return None;
    }
    let bound = 
        format!("{} - {} = 0",
            stringify_variables!(left_member_variables, " + "),
            stringify_variables!(right_member_variables, " - ")
        );
    Some(bound)
}

fn variables_at_position_with_condition(vars: &Variables, position: usize, condition: impl Fn(&Variable) -> bool) -> Vec<String> {
    vars.clone().by_position
    .get(&position).unwrap_or(&vec![])
    .clone().into_iter()
    .filter(|var| condition(var))
    .map(|var| var.label)
    .collect()
}