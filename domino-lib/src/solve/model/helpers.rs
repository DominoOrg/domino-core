//! This module provides utility functions and macros for sorting variable labels,
//! formatting variable collections, and constructing constraint expressions.

use crate::stringify_variables;

use super::variables::Variable;

/// Collects variable labels from a slice of `Variable` structs.
///
/// This function extracts the `label` field from each `Variable` in the provided slice.
///
/// # Arguments
///
/// * `variables` - A slice of `Variable` references.
///
/// # Returns
///
/// A `Vec<String>` containing the labels of all variables.
pub fn collect_labels(variables: &[Variable]) -> Vec<String> {
    variables.iter().map(|var| var.label.clone()).collect()
}

/// Creates a constraint expression enforcing a sum of binary variables.
///
/// This function generates a constraint in the form of `"var1 var2 ... varn = 1"`, ensuring that
/// exactly one of the listed variables is active in the solution.
///
/// # Arguments
///
/// * `labels` - A vector of variable labels (`Vec<String>`) that should be included in the constraint.
///
/// # Returns
///
/// A string representing the constraint expression.
pub fn create_bound_string(labels: Vec<String>) -> String {
    format!("{} = 1", stringify_variables!(labels, " "))
}
