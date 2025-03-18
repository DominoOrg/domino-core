//! This module defines constraints for the optimization model, ensuring valid tile placement and adjacency rules.

use super::{
    helpers::{collect_labels, create_bound_string},
    variables::{Variable, Variables},
    Puzzle,
};
use crate::{stringify_variables, utils::{get_n, Tile}};

/// Generates constraints ensuring each tile is used exactly once.
///
/// This function enforces that:
/// - Each tile appears exactly once in the puzzle.
/// - If a tile has different values (e.g., `(a, b)` vs `(b, a)`), both orientations are considered together.
///
/// # Arguments
///
/// * `vars` - A reference to the `Variables` structure containing decision variables.
///
/// # Returns
///
/// A vector of strings representing the constraints.
pub fn each_tile_used_once_bound(vars: &Variables) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    for variables in vars.by_tile.values() {
        // Directly unwrap the first variable to get the tile
        // This fails only if there are no variables for the tile
        let first_var = variables.get(0).unwrap();
        let mut remapped_vars = collect_labels(variables);

        // Enforce that a tile `(a, b)` does not appear in reverse as `(b, a)`
        if first_var.tile.0 != first_var.tile.1 {
            let rotated_tile = (first_var.tile.1, first_var.tile.0);
            let empty_vec = vec![];
            let unwrapped_rotated_vars = vars.by_tile.get(&rotated_tile).unwrap_or(&empty_vec);
            remapped_vars.extend(collect_labels(unwrapped_rotated_vars));
        }

        if !remapped_vars.is_empty() {
            let bound = create_bound_string(remapped_vars);
            prob_bounds.push(bound);
        }
    }

    prob_bounds.push(String::new()); // Ensure proper formatting with a newline
    prob_bounds
}

/// Generates constraints ensuring each position in the puzzle is filled exactly once.
///
/// This function enforces that every position in the puzzle must be occupied by exactly one tile.
///
/// # Arguments
///
/// * `vars` - A reference to the `Variables` structure containing decision variables.
///
/// # Returns
///
/// A vector of strings representing the constraints.
pub fn each_position_filled_bound(vars: &Variables) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    for variables in vars.by_position.values() {
        let labels = collect_labels(variables);
        let bound = create_bound_string(labels);
        prob_bounds.push(bound);
    }

    prob_bounds.push(String::new()); // Ensure proper formatting with a newline
    prob_bounds
}

/// Generates constraints ensuring the adjacency of tiles in the puzzle.
///
/// This function enforces that each tile must be correctly placed next to its neighboring tile.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` structure representing the puzzle configuration.
/// * `vars` - A reference to the `Variables` structure containing decision variables.
///
/// # Returns
///
/// A vector of strings representing adjacency constraints.
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

    prob_bounds.push(String::new()); // Ensure proper formatting with a newline
    prob_bounds
}

/// Generates a constraint enforcing adjacency when a tile is already placed at the next position.
///
/// This function ensures that if a tile is placed at a position, its adjacent tile must match.
///
/// # Arguments
///
/// * `vars` - A reference to the `Variables` structure containing decision variables.
/// * `tile` - The `Tile` that is already placed at the next position.
/// * `position` - The current position in the puzzle.
///
/// # Returns
///
/// An `Option<String>` containing the adjacency constraint if applicable.
fn next_enforced_bound(vars: &Variables, tile: Tile, position: usize) -> Option<String> {
    let number = tile.0 as usize;
    let condition = |var: &Variable| var.tile.1 == number.try_into().unwrap();
    let left_member_variables: Vec<String> =
        variables_at_position_with_condition(vars, position, condition);

    if left_member_variables.is_empty() {
        return None;
    }

    let bound = format!("{} = 1", stringify_variables!(left_member_variables, " + "));
    Some(bound)
}

/// Generates a constraint enforcing adjacency when the next tile is unknown.
///
/// This function ensures that if a tile is placed at a position, its neighbor must match.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` structure representing the puzzle configuration.
/// * `vars` - A reference to the `Variables` structure containing decision variables.
/// * `position` - The current position in the puzzle.
/// * `number` - The expected adjacent tile number.
///
/// # Returns
///
/// An `Option<String>` containing the adjacency constraint if applicable.
fn next_bound(puzzle: &Puzzle, vars: &Variables, position: usize, number: usize) -> Option<String> {
    let condition = |var: &Variable| var.tile.1 == number.try_into().unwrap();
    let left_member_variables: Vec<String> =
        variables_at_position_with_condition(vars, position, condition);

    let next_position = (position + 1) % puzzle.len();
    let condition = |var: &Variable| var.tile.0 == number.try_into().unwrap();
    let right_member_variables: Vec<String> =
        variables_at_position_with_condition(vars, next_position, condition);

    if left_member_variables.is_empty() {
        return None;
    }

    let bound = format!(
        "{} - {} = 0",
        stringify_variables!(left_member_variables, " + "),
        stringify_variables!(right_member_variables, " - ")
    );
    Some(bound)
}

/// Filters variables at a given position based on a specified condition.
///
/// This function retrieves the variable labels from a given position that satisfy the provided condition.
///
/// # Arguments
///
/// * `vars` - A reference to the `Variables` structure containing decision variables.
/// * `position` - The position in the puzzle grid.
/// * `condition` - A closure that defines the filtering condition.
///
/// # Returns
///
/// A vector of `String` labels representing the variables that satisfy the condition.
fn variables_at_position_with_condition(
    vars: &Variables,
    position: usize,
    condition: impl Fn(&Variable) -> bool,
) -> Vec<String> {
    vars.clone()
        .by_position
        .get(&position)
        .unwrap_or(&vec![])
        .clone()
        .into_iter()
        .filter(|var| condition(var))
        .map(|var| var.label)
        .collect()
}
