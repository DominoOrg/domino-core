use std::cmp::Ordering;

use super::variables::Variable;

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

// Helper function to collect labels from a slice of variables
pub fn collect_labels(variables: &[Variable]) -> Vec<String> {
    variables.iter().map(|var| var.label.clone()).collect()
}

// Helper function to create a bound string from labels
pub fn create_bound_string(labels: Vec<String>) -> String {
    format!("{} = 1", stringify_variables!(labels, " "))
}