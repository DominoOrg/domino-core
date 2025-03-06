//! This module provides utility functions and macros for sorting variable labels,
//! formatting variable collections, and constructing constraint expressions.

use std::cmp::Ordering;
use super::variables::Variable;

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
///
/// # Example
///
/// ```rust
/// let label1 = "A12".to_string();
/// let label2 = "A13".to_string();
/// assert_eq!(sorting_label(&label1, &label2), std::cmp::Ordering::Less);
/// ```
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

/// A macro to concatenate a list of variable labels into a formatted string.
///
/// This macro accepts a list of labels and a separator, with an optional third argument to
/// control line breaks. If a line length is provided, a newline is inserted after every `line_length` elements.
///
/// # Arguments
///
/// * `$labels` - A list of variable labels (`Vec<String>`).
/// * `$separator` - A separator string to join the labels.
/// * `$line_length` (optional) - An optional maximum number of labels per line.
///
/// # Returns
///
/// A formatted string where labels are concatenated with the given separator.
///
/// # Example
///
/// ```rust
/// let labels = vec!["x1".to_string(), "x2".to_string(), "x3".to_string()];
/// let result = stringify_variables!(labels, " ");
/// assert_eq!(result, "x1 x2 x3");
/// ```
#[macro_export]
macro_rules! stringify_variables {
    // Handle two arguments, setting the default to `None` for the third parameter
    ($labels:expr, $separator:expr) => {
        stringify_variables!($labels, $separator, Option::<usize>::None)
    };
    // Handle three arguments where $line_length may be `Some(usize)` or `None`
    ($labels:expr, $separator:expr, $line_length:expr) => {{
        let mut result = String::new();
        let newline_each: usize = $line_length.unwrap_or($labels.len()); // Default to the length of labels if None

        for (i, label) in $labels.iter().enumerate() {
            result.push_str(label);

            // Add a separator if this isn't the last label
            if i < $labels.len() - 1 {
                result.push_str($separator);
            }

            // Add newline every `newline_each` labels, except at the end
            if (i + 1) % newline_each == 0 && i < $labels.len() - 1 {
                result.push('\n');
            }
        }

        result
    }};
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
///
/// # Example
///
/// ```rust
/// let vars = vec![
///     Variable { label: "x1".to_string(), position: 0 },
///     Variable { label: "x2".to_string(), position: 1 },
/// ];
/// let labels = collect_labels(&vars);
/// assert_eq!(labels, vec!["x1".to_string(), "x2".to_string()]);
/// ```
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
///
/// # Example
///
/// ```rust
/// let labels = vec!["x1".to_string(), "x2".to_string(), "x3".to_string()];
/// let constraint = create_bound_string(labels);
/// assert_eq!(constraint, "x1 x2 x3 = 1");
/// ```
pub fn create_bound_string(labels: Vec<String>) -> String {
    format!("{} = 1", stringify_variables!(labels, " "))
}
