//! This module provides utility functions and macros for sorting variable labels,
//! formatting variable collections, and constructing constraint expressions.

use crate::stringify_variables;

use super::variables::Variable;
use std::cmp::Ordering;

/// Compares two variable labels and determines their ordering.
///
/// The labels follow a specific format where:
/// - The second character represents the tile index.
/// - The third character represents the position.
///
/// This function parses those values and sorts labels first by tile index, then by position.
///
/// # Arguments
///
/// * `label1` - A reference to the first label as a `String`.
/// * `label2` - A reference to the second label as a `String`.
///
/// # Returns
///
/// * `Ordering::Less` if `label1` should appear before `label2`.
/// * `Ordering::Greater` if `label1` should appear after `label2`.
/// * `Ordering::Equal` if they are identical.
pub fn sorting_label(label1: &String, label2: &String) -> Ordering {
    let parse_label = |label: &String| {
        let (tile_index, position) = (
            label[1..2].parse::<usize>().unwrap(),
            label[2..3].parse::<usize>().unwrap(),
        );
        (tile_index, position)
    };

    let (l1tileindex, l1position) = parse_label(label1);
    let (l2tileindex, l2position) = parse_label(label2);

    // Compare tile index first, then position
    match l1tileindex.cmp(&l2tileindex) {
        Ordering::Equal => l1position.cmp(&l2position),
        other => other,
    }
}

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
