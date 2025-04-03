//! This module provides functionality for handling variables used in an optimization model.
//!
//! It includes structures and functions to manage variables, generate tile combinations,
//! and construct labeled variables for a given puzzle.

use itertools::Itertools;

use crate::utils::{get_n, DominoError, Puzzle};
use std::collections::HashMap;

/// Represents a decision variable in the optimization model.
///
/// A variable consists of:
/// - A unique `label`
/// - A `tile` represented as a tuple `(usize, usize)`
/// - A `position` indicating its placement in the puzzle
#[derive(Debug, Clone)]
pub struct Variable {
    pub label: String,
    pub tile: (usize, usize),
    pub position: usize,
}

/// Represents a collection of decision variables, with lookup maps for efficient access.
///
/// This structure provides mappings by:
/// - `by_label`: Lookup by variable label.
/// - `by_tile`: Lookup by tile tuple.
/// - `by_position`: Lookup by puzzle position.
///
/// It allows quick access to variables based on different criteria.
#[derive(Debug, Default, Clone)]
pub struct Variables {
    pub(super) vars: Vec<Variable>,
    pub(super) by_label: HashMap<String, Variable>,
    pub(super) by_tile: HashMap<(usize, usize), Vec<Variable>>,
    pub(super) by_position: HashMap<usize, Vec<Variable>>,
}

impl Variables {
    /// Creates a new `Variables` collection from a list of variables.
    ///
    /// # Arguments
    ///
    /// * `combinations` - A vector of `Variable` instances.
    ///
    /// # Returns
    ///
    /// A `Variables` struct populated with the provided variable combinations.
    pub fn new(combinations: Vec<Variable>) -> Self {
        let mut vars = Variables::default();

        for el in combinations {
            vars.insert(el);
        }

        vars
    }

    /// Inserts a new variable into the collection and updates lookup maps.
    ///
    /// # Arguments
    ///
    /// * `variable` - The `Variable` instance to insert.
    fn insert(&mut self, variable: Variable) {
        // Helper function to insert or update a map entry.
        fn insert_or_update<K: std::cmp::Eq + std::hash::Hash, V: Clone>(
            map: &mut HashMap<K, Vec<V>>,
            key: K,
            value: V,
        ) {
            map.entry(key).or_insert_with(Vec::new).push(value);
        }

        self.vars.push(variable.clone());
        self.by_label
            .insert(variable.label.clone(), variable.clone());
        insert_or_update(&mut self.by_tile, variable.tile, variable.clone());
        insert_or_update(&mut self.by_position, variable.position, variable);
    }

    pub fn sort_by_label(&mut self) {
      self.vars = self.vars.clone().into_iter().sorted_by_key(|v| v.label.clone()).collect();
    }
}

/// Generates variables for a given puzzle by determining valid tile placements.
///
/// # Arguments
///
/// * `puzzle` - A reference to the `Puzzle` structure representing the puzzle configuration.
///
/// # Returns
///
/// * `Ok(Variables)` - A `Variables` structure containing all valid decision variables.
/// * `Err(DominoError::InvalidPuzzle)` - If the puzzle is invalid.
pub fn variables(puzzle: &Puzzle) -> Result<Variables, DominoError> {
    let n = get_n(puzzle)? as usize;

    let tileset: Vec<(usize, (usize, usize))> = create_tileset(n)
        .into_iter()
        .enumerate()
        .map(|(i, tile)| (i, tile))
        .collect();

    let mapped_variables = generate_combinations(tileset, n)
        .into_iter()
        .collect();

    let mut vars = Variables::new(mapped_variables);
    vars.sort_by_label();
    Ok(vars)
}

/// Generates a set of tiles based on a given value `N`.
///
/// # Arguments
///
/// * `n` - The maximum tile value.
///
/// # Returns
///
/// A vector of tuples representing all valid tiles `(usize, usize)`.
pub fn create_tileset(n: usize) -> Vec<(usize, usize)> {
    let length: usize = (n + 1).pow(2);
    let mut tileset: Vec<(usize, usize)> = (0..length)
        .map(|i| (i / (n + 1), i % (n + 1)))
        .collect::<Vec<(usize, usize)>>();

    // Adjust tile selection for odd `n` values
    if n % 2 == 1 {
        tileset.retain(|&(i, j)| {
            !(i <= j && j == i + (n + 1) / 2) && !(i > j && i == j + (n + 1) / 2)
        });
    }

    tileset
}

/// Generates variable combinations for a given set of tiles.
///
/// Each tile is assigned a unique label based on its index and position.
///
/// # Arguments
///
/// * `tileset` - A vector of tuples containing tile indices and tiles.
/// * `n` - The maximum tile value.
///
/// # Returns
///
/// A vector of `Variable` instances representing all possible tile placements.
fn generate_combinations(tileset: Vec<(usize, (usize, usize))>, n: usize) -> Vec<Variable> {
    let sequence_length: usize = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1).pow(2) / 2
    };

    let tileset_length = tileset.len();
    let tileset_digits = (tileset_length as f32).log10().floor() as usize + 1;
    let sequence_digits = (sequence_length as f32).log10().floor() as usize + 1;

    let positions: Vec<usize> = (0..sequence_length).collect::<Vec<usize>>();

    tileset
        .iter()
        .flat_map(|(tile_index, tile)| {
            positions.iter().map(move |&position| {
                let label: String = format!(
                    "x{}{}",
                    format_on_n_digits(*tile_index, tileset_digits),
                    format_on_n_digits(position, sequence_digits)
                );
                Variable {
                    label,
                    tile: *tile,
                    position,
                }
            })
        })
        .collect::<Vec<Variable>>()
}

/// Formats a number with leading zeros to match a specified digit width.
///
/// # Arguments
///
/// * `number` - The number to format.
/// * `digits` - The desired width of the formatted number.
///
/// # Returns
///
/// A string representation of the number, zero-padded as needed.
fn format_on_n_digits(number: usize, digits: usize) -> String {
    format!("{:0width$}", number, width = digits)
}
