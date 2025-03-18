//! This module provides functionality for computing optimization models for a given `Puzzle` and `Solution`.
//!
//! It includes constraint generation, objective function computation, and model formulation
//! using mathematical optimization principles.

use helpers::sorting_label;
use itertools::Itertools;

use crate::stringify_variables;
use crate::utils::{DominoError, Puzzle, Solution};

use super::model::bounds::{
    each_position_filled_bound, each_tile_used_once_bound, next_adjacent_bound,
};
use super::model::variables::{variables, Variables};

mod bounds;
mod helpers;
pub mod variables;

/// Generates a set of constraints (bounds) for the given puzzle and variable set.
///
/// The generated constraints ensure that:
/// - Each tile is used only once.
/// - Each position in the puzzle grid is filled.
/// - Tiles maintain adjacency rules.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` for which constraints are generated.
/// * `vars` - A reference to the `Variables` structure containing decision variables.
///
/// # Returns
///
/// A vector of strings representing the mathematical constraints for the optimization model.
fn bounds(puzzle: &Puzzle, vars: &Variables) -> Vec<String> {
    let mut prob_bounds = Vec::new();

    // Add constraints to ensure each tile is used only once.
    prob_bounds.extend(each_tile_used_once_bound(vars));

    // Add constraints to ensure each position in the puzzle is filled.
    prob_bounds.extend(each_position_filled_bound(vars));

    // Add constraints to enforce adjacency rules.
    prob_bounds.extend(next_adjacent_bound(puzzle, vars));

    prob_bounds
}

/// Constructs the objective function for the optimization model.
///
/// The objective function minimizes the number of missing tiles in the puzzle by
/// summing decision variables corresponding to empty positions.
///
/// # Arguments
///
/// * `vars` - A reference to the `Variables` structure containing decision variables.
/// * `puzzle` - A reference to the `Puzzle` for which the objective function is created.
/// * `solution` - A reference to the `Solution` representing the proposed solution.
///
/// # Returns
///
/// A string representing the linear objective function to be minimized.
fn objective_function(vars: &Variables) -> String {
    let labels: Vec<String> = vars.by_label.clone().into_values().map(|var| var.label).collect();
    // Convert the list of variable labels into a formatted string representation.
    let obj = stringify_variables!(labels, " ");
    obj
}

/// Computes a mathematical optimization model for the given puzzle and solution.
///
/// The generated model follows the linear programming (LP) format, including:
/// - **Objective Function:** Minimize the number of missing tiles.
/// - **Constraints:** Ensure valid tile placement and adjacency.
/// - **Binary Variables:** Representing the placement of tiles in the puzzle.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` structure representing the puzzle configuration.
/// * `solution` - A reference to the `Solution` structure representing the proposed solution.
///
/// # Returns
///
/// * `Ok(String)` - A valid optimization model represented as a string.
/// * `Err(DominoError::ModelError)` - If an error occurs while generating the model.
///
/// # Errors
///
/// This function returns an error if:
/// - The variable generation fails, the puzzle is malformed.
/// - There is an issue forming the constraints or objective function, the puzzle is malformed.
pub fn compute_model(puzzle: &Puzzle) -> Result<String, DominoError> {
    // Generate decision variables for the puzzle.
    let prob_vars = variables(puzzle)?;

    // Compute the objective function to minimize missing tiles.
    let prob_obj = objective_function(&prob_vars);

    // Generate constraints (bounds) for valid tile placement.
    let prob_bounds = bounds(puzzle, &prob_vars);

    // Sort variable labels for consistent model formatting.
    let ordered_vars: Vec<&String> = prob_vars
        .by_label
        .keys()
        .sorted_by(|label1, label2| sorting_label(label1, label2))
        .collect();

    // Construct the optimization model in LP format.
    let mut model = "Minimize\n".to_string();
    model.push_str(format!(" obj: {}\n", prob_obj).as_str());
    model.push_str("Subject To\n");

    // Append each constraint with a unique label.
    for (i, constraint) in prob_bounds.iter().enumerate() {
        if !constraint.is_empty() {
            model.push_str(format!(" c{}: {}\n", i, constraint).as_str());
        }
    }

    // Define binary decision variables for tile placement.
    model.push_str("Binary\n");
    for variable in ordered_vars {
        model.push_str(format!(" {}\n", variable).as_str());
    }

    // Finalize the model.
    model.push_str("End");

    Ok(model)
}
