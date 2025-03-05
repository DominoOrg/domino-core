use std::{f32, fmt::Display};

use crate::{DominoError, Puzzle};

use super::common::get_n;

const NUMBER_OF_CLASSES: usize = 3;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Classification(pub usize);

impl Classification {
    pub fn new(class: usize) -> Classification {
        if class > NUMBER_OF_CLASSES {
            panic!("Invalid classification with value: {class}");
        }

        Self(class)
    }
}

impl Display for Classification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0).as_str())
    }
}

impl Into<f32> for Classification {
    fn into(self) -> f32 {
        self.0 as f32
    }
}

/// Classifies the given puzzle and returns its complexity as a `Classification`.
///
/// This function retrieves the puzzle's dimension, calculates a derived length `l` based on whether the dimension
/// is even or odd, and determines if the puzzle is planar (n ≤ 3). Depending on planarity, it computes a maximum
/// allowed hole length (`max_hole`). Holes in the puzzle are then detected, and if none are found, a
/// `DominoError::InvalidLength` is returned. Otherwise, the puzzle's complexity classification is computed.
///
/// # Arguments
///
/// * `puzzle` - A reference to the puzzle, represented as a `Vec<Option<Tile>>`.
///
/// # Returns
///
/// * `Ok(Classification)` containing the computed classification, or
/// * `Err(DominoError)` if an error occurs (for example, if no holes are detected or if `get_n(puzzle)` fails).
pub fn classify_puzzle(puzzle: &Puzzle) -> Result<Classification, DominoError> {
    // Retrieve the dimension of the puzzle (n) and propagate errors if any.
    let n: usize = get_n(puzzle)? as usize;

    // Calculate a derived length `l` based on the puzzle dimension.
    // For even n: l = (n + 1) * (n + 2) / 2; for odd n: l = (n + 1) * (n + 1) / 2.
    let l: usize = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1) * (n + 1) / 2
    };

    // Determine if the puzzle is planar (planar if n <= 3).
    let is_planar = n <= 3;

    // Compute the maximum allowed hole length.
    // If planar, subtract floor(n/2) from l; otherwise, subtract (n + 1) from l.
    let max_hole = if is_planar {
        l as f32 - (n as f32 / 2.0).floor()
    } else {
        l as f32 - (n as f32 + 1.0)
    };

    // Detect holes within the puzzle. Each hole is represented as a tuple (start_index, end_index).
    let holes: Vec<(usize, usize)> = detect_holes(puzzle);

    // Return an error if no holes are detected.
    if holes.len() == 0 {
        return Err(DominoError::InvalidLength);
    }

    // Compute and return the complexity classification based on the detected holes and derived metrics.
    let class = compute_complexity(holes, max_hole, l, is_planar, n);
    Ok(class)
}

/// Computes the overall complexity classification of a puzzle.
///
/// The complexity is derived by first computing an absolute complexity based on the detected holes,
/// then normalizing that value to obtain a relative complexity, and finally converting it into an integer
/// classification.
///
/// # Arguments
///
/// * `holes` - A vector of tuples, each representing the start and end indices of a detected hole.
/// * `max_hole` - The maximum allowed hole length for normalization purposes.
/// * `len` - The derived length `l` computed from the puzzle's dimension.
/// * `is_planar` - A boolean indicating whether the puzzle is planar (n ≤ 3).
/// * `n` - The puzzle's dimension.
///
/// # Returns
///
/// A `Classification` representing the puzzle's complexity.
fn compute_complexity(
    holes: Vec<(usize, usize)>,
    max_hole: f32,
    len: usize,
    is_planar: bool,
    n: usize,
) -> Classification {
    // Calculate the absolute complexity from the detected holes.
    let absolute_complexity = compute_absolute_complexity(holes.clone(), max_hole);

    // Normalize the absolute complexity to obtain a relative complexity score.
    let relative_complexity =
        normalize_complexity(absolute_complexity, len, holes, max_hole, is_planar, n);

    // Convert the relative complexity into an integer classification.
    let class = (relative_complexity * 2.0 + 1.0).floor() as usize;
    Classification::new(class)
}

/// Computes the absolute complexity of a puzzle based on its holes.
///
/// This is done by combining a factor that penalizes the total number of holes and a factor
/// that sums the squared normalized lengths of each hole.
///
/// # Arguments
///
/// * `holes` - A vector of tuples representing the start and end indices of each hole.
/// * `max_hole` - The maximum allowed hole length used to normalize each hole's length.
///
/// # Returns
///
/// The absolute complexity as a floating-point value.
fn compute_absolute_complexity(holes: Vec<(usize, usize)>, max_hole: f32) -> f32 {
    // Calculate a factor that decreases as the number of holes increases.
    let number_of_holes_factor = 1.0 / ((holes.len() as f32).powf(0.1));

    // Sum the squared normalized lengths of each hole.
    let length_factor = holes
        .into_iter()
        .map(|hole| {
            let hole_length = hole.1.saturating_sub(hole.0) as f32;
            (hole_length / max_hole).powf(2.0)
        })
        .sum::<f32>();

    // The absolute complexity is the product of the two factors.
    number_of_holes_factor * length_factor
}

/// Normalizes the absolute complexity to yield a relative complexity score.
///
/// The normalization takes into account a base measure derived from the puzzle length and the number of holes,
/// adjusting for whether the puzzle is planar or not.
///
/// # Arguments
///
/// * `num` - The absolute complexity value computed from the puzzle's holes.
/// * `len` - The derived length `l` from the puzzle's dimension.
/// * `holes` - A vector of tuples representing the detected holes.
/// * `max_hole` - The maximum allowed hole length used for normalization.
/// * `is_planar` - A boolean indicating whether the puzzle is planar (n ≤ 3).
/// * `n` - The puzzle's dimension.
///
/// # Returns
///
/// A normalized (relative) complexity as a floating-point value.
fn normalize_complexity(
    num: f32,
    len: usize,
    holes: Vec<(usize, usize)>,
    max_hole: f32,
    is_planar: bool,
    n: usize,
) -> f32 {
    // Calculate a base measure 's' that depends on planarity.
    // For planar puzzles, s = len - (number of holes).
    // For non-planar puzzles, s = len - (n + 2).
    let s = if is_planar {
        len - holes.len()
    } else {
        len - (n + 2)
    };

    // Determine the number of complete `max_hole` segments that fit into s.
    let n0 = (s as f32 / max_hole).floor();

    // Calculate the remainder after accounting for the complete segments.
    let r = s as f32 - (n0 * max_hole);

    // Compute two candidate normalization factors:
    // 1. Based on the squared ratio of the remainder to max_hole.
    // 2. Based solely on the number of complete segments (n0).
    let max = f32::max(
        (n0 + (r / max_hole).powf(2.0)) / (n0 + 1.0).powf(0.1),
        n0.powf(0.9),
    );

    // Normalize the absolute complexity by the maximum candidate factor.
    num / max
}

/// Detects holes in the given puzzle and returns their index ranges.
///
/// A "hole" is defined as a contiguous sequence of missing tiles (`None` values)
/// whose boundaries are determined by the presence of a tile (`Some`) on either side.
/// This function treats the puzzle as cyclic; that is, the neighbor of the first element
/// is the last element, and the neighbor of the last element is the first element.
///
/// The implementation uses iterator combinators to traverse the puzzle:
/// - It employs `enumerate` to process each tile along with its index.
/// - It uses `scan` to maintain an optional state (`maybe_start`) representing the starting
///   index of the current hole.
/// - For each tile, `map_or_else` is used to handle the case when a tile is missing (`None`)
///   versus when it is present (`Some`):
///   - For a `None` tile, it checks if the previous neighbor exists (with wrap-around)
///     to potentially mark the start of a hole, and if the next neighbor exists
///     (with wrap-around) to determine the end of the hole. If a hole is detected,
///     the function yields a tuple `(start_index, end_index)`.
///   - For a `Some` tile, it yields `None`, effectively ignoring it.
/// - Finally, `filter_map` is used to discard `None` values, collecting only the valid hole tuples.
///
/// # Arguments
///
/// * `puzzle` - A reference to the puzzle represented as a `Vec<Option<Tile>>`, where `Tile` is a tuple struct.
///
/// # Returns
///
/// A vector of tuples `(usize, usize)` where each tuple represents a detected hole:
/// - The first element is the starting index (inclusive) of the hole.
/// - The second element is the index immediately after the last missing tile (exclusive).
///
/// # Example
///
/// ```
/// // Assume Puzzle is defined as Vec<Option<Tile>> and Tile is a tuple struct.
/// let puzzle: Puzzle = vec![Some(Tile(1, 2)), None, None, Some(Tile(3, 4))];
/// let holes = detect_holes(&puzzle);
/// // Example output (depending on wrap-around logic): [(0, 3)]
/// ```
fn detect_holes(puzzle: &Puzzle) -> Vec<(usize, usize)> {
    let len = puzzle.len();
    // Traverse the puzzle with enumerated indices while maintaining a state for the start index of a hole.
    puzzle
        .iter()
        .enumerate()
        .scan(None, |maybe_start, (i, &tile)| {
            // Use `map_or_else` to handle the case when a tile is missing or present:
            // - For a missing tile (None), process potential hole boundaries.
            // - For a present tile (Some), yield None (no hole to record).
            tile.map_or_else(
                || {
                    // For a missing tile, check if the previous neighbor is present.
                    // This check uses wrap-around: if i == 0, the previous tile is at index len - 1.
                    let candidate_start: Option<usize> = Some(i).filter(|_| {
                        (if i > 0 {
                            puzzle[i - 1]
                        } else {
                            puzzle[len - 1]
                        })
                        .is_some()
                    });
                    // Update the tracked start index: if a candidate is found, or keep the existing one.
                    *maybe_start = candidate_start.or(*maybe_start);

                    // Check if the next neighbor is present.
                    // This check also uses wrap-around: if i is the last index, the next tile is at index 0.
                    // If the next tile is present, this indicates the end of a hole.
                    Some(
                        Some(i + 1)
                            .filter(|_| {
                                (if i < len - 1 {
                                    puzzle[i + 1]
                                } else {
                                    puzzle[0]
                                })
                                .is_some()
                            })
                            // If an end is determined and a start index was previously recorded,
                            // produce a tuple representing the hole, then reset the start index.
                            .and_then(|end: usize| {
                                maybe_start.take().map(|start: usize| (start, end))
                            }),
                    )
                },
                // If the tile is present, simply yield None (no hole processing).
                |_| Some(None),
            )
        })
        // Remove None values, retaining only valid hole tuples.
        .filter_map(|hole| hole)
        // Collect all hole tuples into a vector.
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::Tile;

    #[test]
    fn test_classify() {
        // Empty puzzle should be classified as error
        let puzzle: Vec<Option<Tile>> = vec![];
        let complexity = super::classify_puzzle(&puzzle);
        assert!(complexity.is_err());

        // Puzzle with invalid length should be classified as error
        let puzzle: Vec<Option<Tile>> = vec![None, None, None, None, None, None, None];
        let complexity = super::classify_puzzle(&puzzle);
        assert!(complexity.is_err());

        // Puzzle with only all None tiles should be classified as error
        let puzzle: Vec<Option<Tile>> = vec![None, None, None, None, None, None, None, None];
        let complexity = super::classify_puzzle(&puzzle);
        assert!(complexity.is_err());

        // Puzzle with only all Some tiles should be classified as error
        let puzzle: Vec<Option<Tile>> = vec![
            Some(Tile(0, 1)),
            Some(Tile(1, 1)),
            Some(Tile(1, 2)),
            Some(Tile(2, 2)),
            Some(Tile(2, 3)),
            Some(Tile(3, 3)),
            Some(Tile(3, 0)),
            Some(Tile(0, 0)),
        ];
        let complexity = super::classify_puzzle(&puzzle);
        assert!(complexity.is_err());

        // Valid classifications should range between 1 and NUMBER_OF_CLASSES
    }
}
