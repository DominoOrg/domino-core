use crate::{utils::get_n, DominoError, Puzzle};
pub use complexity_class::ComplexityClass;
use formula::{compute_threshold, find_threshold_index};

mod complexity_class;
mod formula;

pub const NUMBER_OF_CLASSES: usize = 3;

/// Classifies the given puzzle and returns its complexity as a `ComplexityClass`.
///
/// This function retrieves the puzzle's dimension, calculates a derived length `l` based on whether the dimension
/// is even or odd, and determines if the puzzle is planar (n ≤ 3). Depending on planarity, it computes a maximum
/// allowed hole length (`max_hole`). Holes in the puzzle are then detected, and if none are found, a
/// `DominoError::InvalidLength` is returned. If the entire puzzle consists of empty tiles, a `DominoError::EmptyPuzzle`
/// is thrown. Otherwise, the puzzle's complexity ComplexityClass is computed.
///
/// # Arguments
///
/// * `puzzle` - A reference to the puzzle, represented as a `Vec<Option<Tile>>`.
///
/// # Returns
///
/// * `Ok(ComplexityClass)` containing the computed ComplexityClass, or
/// * `Err(DominoError)` if an error occurs (for example, if no holes are detected, the puzzle is empty, or if `get_n(puzzle)` fails).
pub fn classify_puzzle(puzzle: &Puzzle) -> Result<ComplexityClass, DominoError> {
    // Check if the puzzle consists entirely of empty tiles
    if puzzle.0.iter().all(|tile| tile.is_none()) {
        return Err(DominoError::EmptyPuzzle); // Throw error if all tiles are empty
    }
    println!("puzzle: {puzzle:?}");

    // Retrieve the dimension of the puzzle (n) and propagate errors if any.
    let n: usize = get_n(puzzle)? as usize;

    // Calculate a derived length `l` based on the puzzle dimension.
    // For even n: l = (n + 1) * (n + 2) / 2; for odd n: l = (n + 1) * (n + 1) / 2.
    let l: usize = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1) * (n + 1) / 2
    };

    // Compute the maximum allowed hole length in a generic puzzle leaving it valid.
    // If planar, subtract floor(n/2) from l; otherwise, subtract (n + 1) from l.
    let max_hole: usize = if n >= 4 { n + 1 } else { (n + 1) * 2 - 1 };

    // Detect holes within the puzzle. Each hole is represented as a tuple (start_index, end_index).
    let holes: Vec<(usize, usize)> = detect_holes(puzzle);

    // Return an error if no holes are detected.
    if holes.len() == 0 {
        return Err(DominoError::InvalidLength);
    }

    // Compute and return the complexity ComplexityClass based on the detected holes and derived metrics.
    let class = compute_complexity(holes, max_hole, l);
    class
}

/// Returns the range of the number of tiles to remove in a puzzle
/// to match the specified complexity class, based on the classification system and puzzle size `n`.
///
/// # Arguments
///
/// * `class` - A `ComplexityClass` representing the puzzle's difficulty level.
/// * `n` - The puzzle's dimension.
///
/// # Returns
///
/// A tuple `(usize, usize)`, where:
/// - The first value is the minimum number of tiles to remove (always ≥ 1).
/// - The second value is the maximum number of tiles to remove.
///
/// # Panics
///
/// This function panics if an invalid `ComplexityClass` is provided.
#[allow(dead_code)]
pub fn tiles_to_remove_range(class: ComplexityClass, n: usize) -> (usize, usize) {
    // Compute the derived length `l` based on the puzzle's dimension `n`
    let l = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1) * (n + 1) / 2
    } as f32;

    // Determine whether the puzzle is planar (n <= 3)
    let is_planar = n <= 3;

    // Compute the maximum allowed hole size.
    let max_hole = if is_planar {
        l - (n as f32 / 2.0).floor()
    } else {
        l - (n as f32 + 1.0)
    };

    // Inverted formula to compute min-max values within 0.0-1.0 range
    // that can generate the current complexity
    // The class is in [1,NUMBER_OF_CLASSES] so panics only if the ComplexityClass implementation
    // changes and has no more the check on the values
    let (lower_relative, upper_relative) =
        inverse_class_mapping(class).expect("The provided class is invalid");
    // Invert the formula to compute the number of tiles to remove:
    //     t = relative_complexity * max_hole
    // so t must lie in:
    //     [ lower_relative * max_hole, upper_relative * max_hole )
    let lower_bound_float = lower_relative * max_hole;
    let upper_bound_float = upper_relative * max_hole;

    // The minimum valid number of tiles is the flooring of the lower bound.
    // However, we ensure it is at least 1 (since a valid puzzle must have at least one removed tile).
    let min_tiles = std::cmp::max(1, lower_bound_float.floor() as usize);

    // The maximum valid number of tiles is the largest integer strictly less than the upper bound.
    let max_tiles = if upper_bound_float.fract() == 0.0 {
        (upper_bound_float as usize).saturating_sub(1)
    } else {
        upper_bound_float.floor() as usize
    };

    // Ensure the maximum is not over the maximum number of tiles removable.
    let mut max_tiles = std::cmp::min(max_tiles, max_hole as usize);

    // If the class is equal to the max NUMBER_OF_CLASSES, ensure the maximum is the maximum number of tiles removable.
    if class == NUMBER_OF_CLASSES {
        max_tiles = max_hole as usize;
    }

    // Ensure that the maximum is not below the minimum.
    let (min_tiles, max_tiles) = if max_tiles < min_tiles {
        (max_tiles, min_tiles)
    } else {
        (min_tiles, max_tiles)
    };

    (min_tiles, max_tiles)
}

/// Returns the min and max decimal values that could have produced a given class.
///
/// This function performs an inverse mapping from the computed class (1 to NUMBER_OF_CLASSES)
/// back to the original `x` range.
///
/// # Arguments
///
/// * `class` - The computed class value (expected to be in `[1, NUMBER_OF_CLASSES]`)
///
/// # Returns
///
/// A tuple `(min_x, max_x)` representing the range of decimal values that
/// could have resulted in the given class, or `None` if the class is out of bounds.
///
/// # Examples
///
/// ```
/// const NUMBER_OF_CLASSES: usize = 3;
///
/// fn compute_threshold(class: usize) -> f32 {
///     class as f32 / NUMBER_OF_CLASSES as f32
/// }
///
/// let (min_x, max_x) = inverse_class_mapping(2).unwrap();
/// assert_eq!(min_x, compute_threshold(1));
/// assert_eq!(max_x, compute_threshold(2));
/// ```
#[allow(dead_code)]
fn inverse_class_mapping(class: ComplexityClass) -> Option<(f32, f32)> {
    if class < 1 || class > NUMBER_OF_CLASSES {
        return None;
    }

    let min_x = if class == 1 {
        0.0
    } else {
        compute_threshold(class.0 - 1)
    };
    let max_x = compute_threshold(class.0);

    Some((min_x, max_x))
}

/// Computes the overall complexity ComplexityClass of a puzzle.
///
/// The complexity is derived by first computing an absolute complexity based on the detected holes,
/// then normalizing that value to obtain a relative complexity, and finally converting it into an integer
/// ComplexityClass.
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
/// A `ComplexityClass` representing the puzzle's complexity.
fn compute_complexity(
    holes: Vec<(usize, usize)>,
    max_hole: usize,
    len: usize,
) -> Result<ComplexityClass, DominoError> {
    // Calculate the absolute complexity from the detected holes
    // The returned complexity is relative to the hardest case possible (1 hole with the max size).
    // It may exceed 1.0 if multiple holes are present.
    let absolute_complexity = compute_absolute_complexity(holes.clone(), max_hole, len);

    // Normalize the absolute complexity to obtain a relative complexity score between 0.0 and 1.0.
    let relative_complexity = absolute_complexity.clamp(0.0, 1.0);

    // Convert the relative complexity into an integer ComplexityClass.
    let class = find_threshold_index(relative_complexity);
    ComplexityClass::new(class)
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
fn compute_absolute_complexity(holes: Vec<(usize, usize)>, max_hole: usize, len: usize) -> f32 {
    // Ensures we don't incour into a division by 0, returning 0.0
    if holes.is_empty() {
        return 0.0;
    }

    // Calculate a factor that decreases as the number of holes increases.
    let number_of_holes_factor = 1.0 / ((holes.len() as f32).powf(0.1));

    // Sum the squared normalized lengths of each hole.
    let length_factor = holes
        .clone()
        .into_iter()
        .map(|hole| {
            let hole_length: usize = if hole.1 > hole.0 {
                hole.1.saturating_sub(hole.0)
            } else {
                // println!("({len} - {}) + {}", hole.0, hole.1);
                (len - hole.0) + hole.1
            };
            // println!("hole: {hole:?}, hole_length: {hole_length}");
            // println!("number_of_holes_factor: {number_of_holes_factor} hole_lenght: {hole_length} length_factor: {}", (hole_length as f32/ max_hole as f32).powf(2.0));
            (hole_length as f32 / max_hole as f32).powf(2.0)
        })
        .sum::<f32>();

    // The absolute complexity is the product of the two factors.
    number_of_holes_factor * length_factor
}

/// Detects holes in the given puzzle and returns their index ranges.
///
/// A "hole" is defined as a contiguous sequence of missing tiles (`None` values)
/// whose boundaries are determined by the presence of a tile (`Some`) on both sides.
/// This function treats the puzzle as cyclic; that is, the neighbor of the first element
/// is the last element, and the neighbor of the last element is the first element.
/// If a hole spans the end and beginning of the puzzle, it is treated as a single
/// contiguous hole rather than two separate ones.
/// If the puzzle consists entirely of `None` values, it is considered a single hole.
///
/// # Arguments
///
/// * `puzzle` - A reference to the puzzle represented as a `Vec<Option<Tile>>`,
///   where `Tile` is a tuple struct.
///
/// # Returns
///
/// A vector of tuples `(usize, usize)` where each tuple represents a detected hole:
/// - The first element is the starting index (inclusive) of the hole.
/// - The second element is the index immediately after the last missing tile (exclusive).
pub fn detect_holes(puzzle: &Puzzle) -> Vec<(usize, usize)> {
    let len = puzzle.0.len();
    let mut holes = Vec::new();
    let mut maybe_start: Option<usize> = None;
    let mut wraps_around = false;
    let mut has_some = false;

    // Iterate through the puzzle to detect holes.
    for i in 0..len {
        if puzzle.0[i].is_none() {
            // Mark the start of a new hole if not already marked.
            if maybe_start.is_none() {
                maybe_start = Some(i);
            }
        } else {
            has_some = true; // Mark that we have at least one Some value.
            if let Some(start) = maybe_start.take() {
                // If a hole was being tracked and a tile (`Some`) is found, finalize the hole range.
                holes.push((start, i));
            }
        }
    }

    // Handle the case where a hole extends to the end of the puzzle.
    if let Some(start) = maybe_start {
        if !holes.is_empty() && holes[0].0 == 0 {
            // If the first hole starts at index 0, it means the hole wraps around.
            wraps_around = true;
            holes[0] = (start, holes[0].1);
        } else {
            // Otherwise, add the hole as a separate entry.
            holes.push((start, len));
        }
    }

    // Merge the last and first holes if they form a cyclic sequence.
    if wraps_around && holes.len() > 1 {
        let first = holes.remove(0); // Remove the first hole (which starts at 0).
        let last = holes.pop().unwrap(); // Take the last hole.
        holes.insert(0, (last.0, first.1)); // Merge the two as a single hole.
    }

    // If the entire puzzle is a hole (no `Some` elements exist), return it as a single hole.
    if !has_some {
        return vec![(0, len)];
    }

    holes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ComplexityClass, DominoError, Puzzle, Tile};

    /// Creates a mock puzzle based on the given parameters.
    ///
    /// **Input:**
    /// - `size`: The number of tiles in the puzzle.
    /// - `holes`: A vector of indices representing missing tiles.
    ///
    /// **Output:**
    /// - A `Puzzle` with `Some(Tile(0,0))` at non-hole indices and `None` at hole indices.
    fn mock_puzzle(size: usize, holes: Vec<usize>) -> Puzzle {
        let mut puzzle = vec![Some(Tile(0, 0)); size];
        for index in holes {
            puzzle[index] = None;
        }
        puzzle.into()
    }

    /// Helper function to generate a list of test cases
    fn test_cases() -> Vec<(ComplexityClass, usize, (usize, usize))> {
        let mut cases = Vec::new();
        for n in [3, 4, 5, 6] {
            for class in 1..=NUMBER_OF_CLASSES {
                let expected = tiles_to_remove_range(ComplexityClass::new(class).unwrap(), n);
                cases.push((ComplexityClass::new(class).unwrap(), n, expected));
            }
        }
        cases
    }

    #[test]
    fn test_tiles_to_remove_range() {
        for (class, n, expected) in test_cases() {
            assert_eq!(
                tiles_to_remove_range(class, n),
                expected,
                "Failed for class {:?} and n = {}",
                class,
                n
            );
        }
    }
    mod classify_puzzle_tests {
        use super::*;

        /// Tests classification of an empty puzzle.
        ///
        /// **Input:** A puzzle consisting entirely of `None` values.
        /// **Expected Output:** `Err(DominoError::EmptyPuzzle)`.
        #[test]
        fn test_classify_puzzle_empty_puzzle() {
            let puzzle = mock_puzzle(8, (0..8).collect()); // All empty tiles
            assert_eq!(classify_puzzle(&puzzle), Err(DominoError::EmptyPuzzle));
        }

        /// Tests classification of a puzzle with no holes.
        ///
        /// **Input:** A puzzle containing only `Some(Tile(0,0))` values.
        /// **Expected Output:** `Err(DominoError::InvalidLength)`.
        #[test]
        fn test_classify_puzzle_no_holes() {
            let puzzle = mock_puzzle(8, vec![]); // No holes
            assert_eq!(classify_puzzle(&puzzle), Err(DominoError::InvalidLength));
        }
    }

    mod detect_holes_tests {
        use super::*;

        /// Tests hole detection in a puzzle.
        ///
        /// **Input:** A puzzle containing `Some(Tile(0,0))` and `None` values with two distinct hole regions.
        /// **Expected Output:** A vector of hole indices correctly identifying the start and end of each hole.
        #[test]
        fn test_classify_detect_holes_correctly() {
            let puzzle = mock_puzzle(8, vec![1, 2, 4]);
            let holes = detect_holes(&puzzle);
            assert_eq!(holes, vec![(1, 3), (4, 5)]);
        }

        /// Tests detection when there is a single hole spanning multiple positions.
        ///
        /// **Input:** A puzzle with a single hole spanning indices [3,6].
        /// **Expected Output:** A single tuple (3,6).
        #[test]
        fn test_classify_detect_holes_single_large_hole() {
            let puzzle = mock_puzzle(8, vec![3, 4, 5]);
            let holes = detect_holes(&puzzle);
            assert_eq!(holes, vec![(3, 6)]);
        }

        /// Tests detection of a hole that wraps around the end of the puzzle.
        ///
        /// **Input:** A puzzle where holes exist at the end and beginning.
        /// **Expected Output:** The hole should be merged correctly as (6, 2).
        #[test]
        fn test_classify_detect_holes_wraparound() {
            let puzzle = mock_puzzle(8, vec![6, 7, 0, 1]);
            let holes = detect_holes(&puzzle);
            assert_eq!(holes, vec![(6, 2)]);
        }
    }

    #[cfg(test)]
    mod compute_complexity_tests {
        use super::*;

        /// Helper function to create a puzzle with a single hole of a given length.
        ///
        /// **Input:**
        /// - `n`: The dimension of the puzzle.
        /// - `hole_size`: The number of consecutive tiles removed.
        /// - `hole_start`: The index where the hole begins.
        ///
        /// **Output:**
        /// - A `Puzzle` with `Some(Tile(0,0))` at non-hole indices and `None` at hole indices.
        fn create_puzzle_with_hole(n: usize, hole_size: usize, hole_start: usize) -> Puzzle {
            let total_size = if n % 2 == 0 {
                (n + 1) * (n + 2) / 2
            } else {
                (n + 1) * (n + 1) / 2
            };

            let mut puzzle = vec![Some(Tile(0, 0)); total_size];
            for i in 0..hole_size {
                puzzle[(hole_start + i) % total_size] = None;
            }
            Puzzle(puzzle)
        }

        /// Tests complexity calculation for puzzles with n = 3 and various hole lengths.
        ///
        /// **Input:** Puzzles with `n = 3`, each having a single hole of different lengths.
        /// **Expected Output:** `Ok(ComplexityClass)` ensuring successful complexity classification.
        #[test]
        fn test_compute_complexity_n3_various_holes() {
            let n = 3;
            let total_size = (n + 1) * (n + 1) / 2;
            let max_hole = (n + 1) * 2 - 1;

            for hole_size in 1..=max_hole as usize {
                let puzzle = create_puzzle_with_hole(n, hole_size, 0);
                let holes = detect_holes(&puzzle);
                let complexity = compute_complexity(holes, max_hole, total_size);
                let expected_abs = (hole_size as f32 / max_hole as f32).powf(2.0);
                let expected_rel = expected_abs.clamp(0.0, 1.0);
                let expected_class = find_threshold_index(expected_rel);

                assert_eq!(
                    complexity.ok(),
                    Some(ComplexityClass(expected_class)),
                    "Failed for hole_size = {}",
                    hole_size
                );
            }
        }

        /// Tests complexity classification for a puzzle with n = 3 and a single large hole.
        ///
        /// **Input:** A puzzle of size `n = 3` with a single large hole spanning the maximum allowed length.
        /// **Expected Output:** `Ok(ComplexityClass)`, verifying correct classification for large holes.
        #[test]
        fn test_compute_complexity_n3_large_hole() {
            let n = 3;
            let total_size = (n + 1) * (n + 1) / 2;
            let max_hole = (n + 1) * 2 - 1;

            let puzzle = create_puzzle_with_hole(n, max_hole as usize, 0);

            let holes = detect_holes(&puzzle);
            let complexity = compute_complexity(holes, max_hole, total_size);

            assert!(complexity.is_ok(), "Failed for a single large hole");
        }

        /// Tests complexity classification for a puzzle with n = 3 and multiple small holes.
        ///
        /// **Input:** A puzzle of size `n = 3` with multiple small holes at different positions.
        /// **Expected Output:** `Ok(ComplexityClass)`, verifying the handling of multiple distinct holes.
        #[test]
        fn test_compute_complexity_n3_multiple_small_holes() {
            let n = 3;
            let total_size = (n + 1) * (n + 1) / 2;
            let max_hole = (n + 1) * 2 - 1;
            let puzzle = mock_puzzle(total_size, vec![1, 3, 5]);

            let holes = detect_holes(&puzzle);
            let complexity = compute_complexity(holes, max_hole, total_size);

            assert!(complexity.is_ok(), "Failed for multiple small holes");
        }
    }

    /// Tests inverse class mapping for valid classes (1 to NUMBER_OF_CLASSES).
    ///
    /// **Input:** Valid `ComplexityClass` values from 1 to `NUMBER_OF_CLASSES`.
    /// **Expected Output:** `(min_x, max_x)` computed using the function's logic.
    #[test]
    fn test_classify_inverse_class_mapping() {
        for class_value in 1..=NUMBER_OF_CLASSES {
            let class = ComplexityClass::new(class_value).unwrap();
            let result = inverse_class_mapping(class);
            assert!(result.is_some());
            let (min_x, max_x) = result.unwrap();
            let expected_min_x = compute_threshold(class_value - 1);
            let expected_max_x = compute_threshold(class_value);

            assert_eq!(min_x, expected_min_x);
            assert_eq!(max_x, expected_max_x);
        }
    }
}
