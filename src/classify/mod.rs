use crate::{utils::get_n, DominoError, Puzzle};
pub use complexity_class::ComplexityClass;

mod complexity_class;

const NUMBER_OF_CLASSES: usize = 3;

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
    if puzzle.iter().all(|tile| tile.is_none()) {
        return Err(DominoError::EmptyPuzzle); // Throw error if all tiles are empty
    }

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

    // Compute the maximum allowed hole length in a generic puzzle leaving it valid.
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

    // Compute and return the complexity ComplexityClass based on the detected holes and derived metrics.
    let class = compute_complexity(holes, max_hole, l, is_planar, n);
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

    // The forward classification formula is:
    //     class = floor(2 * relative_complexity + 1)
    // For a given class `c`, valid relative_complexity values lie in:
    //     [ (c - 1)/2, c/2 )
    let lower_relative = (class.0 as f32 - 1.0) / 2.0;
    let upper_relative = class.0 as f32 / 2.0;

    // Invert the formula to compute the number of tiles to remove:
    //     t = relative_complexity * max_hole
    // so t must lie in:
    //     [ lower_relative * max_hole, upper_relative * max_hole )
    let lower_bound_float = lower_relative * max_hole;
    let upper_bound_float = upper_relative * max_hole;

    // The minimum valid number of tiles is the ceiling of the lower bound.
    // However, we ensure it is at least 1 (since a valid puzzle must have at least one removed tile).
    let min_tiles = std::cmp::max(1, lower_bound_float.ceil() as usize);

    // The maximum valid number of tiles is the largest integer strictly less than the upper bound.
    let max_tiles = if upper_bound_float.fract() == 0.0 {
        (upper_bound_float as usize).saturating_sub(1)
    } else {
        upper_bound_float.floor() as usize
    };

    // Ensure that the maximum is not below the minimum.
    let max_tiles = if max_tiles < min_tiles {
        min_tiles
    } else {
        max_tiles
    };

    (min_tiles, max_tiles)
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
    max_hole: f32,
    len: usize,
    is_planar: bool,
    n: usize,
) -> Result<ComplexityClass, DominoError> {
    // Calculate the absolute complexity from the detected holes.
    let absolute_complexity = compute_absolute_complexity(holes.clone(), max_hole);

    // Normalize the absolute complexity to obtain a relative complexity score.
    let relative_complexity =
        normalize_complexity(absolute_complexity, len, holes, max_hole, is_planar, n);

    // Convert the relative complexity into an integer ComplexityClass.
    let class = (relative_complexity * 2.0 + 1.0).floor() as usize;
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
/// # Arguments
///
/// * `puzzle` - A reference to the puzzle represented as a `Vec<Option<Tile>>`, where `Tile` is a tuple struct.
///
/// # Returns
///
/// A vector of tuples `(usize, usize)` where each tuple represents a detected hole:
/// - The first element is the starting index (inclusive) of the hole.
/// - The second element is the index immediately after the last missing tile (exclusive).
pub fn detect_holes(puzzle: &Puzzle) -> Vec<(usize, usize)> {
    let len = puzzle.len();
    let mut holes = Vec::new();
    let mut maybe_start: Option<usize> = None;

    for i in 0..len {
        if puzzle[i].is_none() {
            // If we haven't marked the start of a hole yet, do so now.
            if maybe_start.is_none() {
                maybe_start = Some(i);
            }
        } else if let Some(start) = maybe_start.take() {
            // If we reach a present tile and had a started hole, store the hole range.
            holes.push((start, i));
        }
    }

    // Handle wrap-around case where the hole extends to the end and connects to the beginning.
    if let Some(start) = maybe_start {
        if !holes.is_empty() && holes[0].0 == 0 {
            // Merge wrap-around hole with the first detected hole.
            holes[0] = (start, holes[0].1);
        } else {
            // Otherwise, add the hole separately.
            holes.push((start, len));
        }
    }

    holes
}

#[cfg(test)]
mod tests {
    use super::classify_puzzle;
    use crate::{Puzzle, Tile};

    #[test]
    fn test_empty_puzzle_should_return_error() {
        // Empty puzzle should be classified as error
        let puzzle: Vec<Option<Tile>> = vec![];
        let complexity = super::classify_puzzle(&puzzle);
        assert!(complexity.is_err());
    }

    #[test]
    fn test_puzzle_with_invalid_length_should_return_error() {
        // Puzzle with invalid length should be classified as error
        let puzzle: Vec<Option<Tile>> = vec![None, None, None, None, None, None, None];
        let complexity = super::classify_puzzle(&puzzle);
        assert!(complexity.is_err());
    }

    #[test]
    fn test_puzzle_with_all_none_tiles_should_return_error() {
        // Puzzle with only all None tiles should be classified as error
        let puzzle: Vec<Option<Tile>> = vec![None, None, None, None, None, None, None, None];
        let complexity = super::classify_puzzle(&puzzle);
        assert!(complexity.is_err());
    }

    #[test]
    fn test_puzzle_with_all_some_tiles_should_return_error() {
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
    }

    /// Computes the expected puzzle length for a given puzzle dimension `n`
    /// using the same formulas as in `classify_puzzle`.
    fn puzzle_length(n: usize) -> usize {
        if n % 2 == 0 {
            (n + 1) * (n + 2) / 2
        } else {
            (n + 1) * (n + 1) / 2
        }
    }

    /// Builds a puzzle (Vec<Option<Tile>>) for a given puzzle dimension `n`
    /// and inserts a single contiguous hole of length `hole_length` starting
    /// at index `hole_start` (wrap-around is supported).
    fn build_puzzle_with_hole(n: usize, hole_start: usize, hole_length: usize) -> Puzzle {
        let len = puzzle_length(n);
        // Fill with dummy Some(Tile) values.
        let mut puzzle: Vec<Option<Tile>> =
            (0..len).map(|i| Some(Tile(i as i32, i as i32))).collect();

        // Insert the hole (set the specified contiguous indices to None).
        for j in 0..hole_length {
            let idx = (hole_start + j) % len;
            puzzle[idx] = None;
        }
        puzzle
    }

    // --- Tests for planar puzzles (n <= 3) ---

    // n = 2 (planar): length = (2+1)*(2+2)/2 = 6, max_hole = 6 - floor(2/2) = 5.
    mod n2 {
        use super::*;
        const N: usize = 2;

        #[test]
        fn test_classification_class1_n2() {
            // Minimal hole (length = 1) yields relative complexity = (1/5)² ≈ 0.04.
            let puzzle = build_puzzle_with_hole(N, 1, 1);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one small hole should classify successfully");
            assert_eq!(
                classification.0, 1,
                "n=2: Minimal hole should be classified as 1"
            );
        }

        #[test]
        fn test_classification_class2_n2() {
            // Moderate hole (length = 4) gives (4/5)² = 0.64 → floor(2*0.64 + 1) = 2.
            let puzzle = build_puzzle_with_hole(N, 1, 4);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one moderate hole should classify successfully");
            assert_eq!(
                classification.0, 2,
                "n=2: Moderate hole should be classified as 2"
            );
        }

        #[test]
        fn test_classification_class3_n2() {
            // Maximum hole (length = 5) gives (5/5)² = 1 → floor(2*1 + 1) = 3.
            let puzzle = build_puzzle_with_hole(N, 1, 5);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one large hole should classify successfully");
            assert_eq!(
                classification.0, 3,
                "n=2: Maximum hole should be classified as 3"
            );
        }
    }

    // n = 3 (planar): length = (3+1)²/2 = 8, max_hole = 8 - floor(3/2) = 7.
    mod n3 {
        use super::*;
        const N: usize = 3;

        #[test]
        fn test_classification_class1_n3() {
            // Minimal hole (length = 1) yields (1/7)² ≈ 0.02.
            let puzzle = build_puzzle_with_hole(N, 1, 1);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one small hole should classify successfully");
            assert_eq!(
                classification.0, 1,
                "n=3: Minimal hole should be classified as 1"
            );
        }

        #[test]
        fn test_classification_class2_n3() {
            // Moderate hole (length = 5) yields (5/7)² ≈ 0.51.
            let puzzle = build_puzzle_with_hole(N, 1, 5);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one moderate hole should classify successfully");
            assert_eq!(
                classification.0, 2,
                "n=3: Moderate hole should be classified as 2"
            );
        }

        #[test]
        fn test_classification_class3_n3() {
            // Maximum hole (length = 7) yields (7/7)² = 1.
            let puzzle = build_puzzle_with_hole(N, 1, 7);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one large hole should classify successfully");
            assert_eq!(
                classification.0, 3,
                "n=3: Maximum hole should be classified as 3"
            );
        }
    }

    // --- Tests for non-planar puzzles (n > 3) ---

    // n = 4 (non-planar): length = (4+1)*(4+2)/2 = 15, max_hole = 15 - (4+1) = 10.
    mod n4 {
        use super::*;
        const N: usize = 4;

        #[test]
        fn test_classification_class1_n4() {
            // Minimal hole (length = 1) yields a very small relative complexity.
            let puzzle = build_puzzle_with_hole(N, 2, 1);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one small hole should classify successfully");
            assert_eq!(
                classification.0, 1,
                "n=4: Minimal hole should be classified as 1"
            );
        }

        #[test]
        fn test_classification_class2_n4() {
            // Moderate hole (length = 8) gives a relative complexity in the range for classification 2.
            let puzzle = build_puzzle_with_hole(N, 2, 8);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one moderate hole should classify successfully");
            assert_eq!(
                classification.0, 2,
                "n=4: Moderate hole should be classified as 2"
            );
        }

        #[test]
        fn test_classification_class3_n4() {
            // Maximum hole (length = 10) gives a relative complexity of 1, yielding classification 3.
            let puzzle = build_puzzle_with_hole(N, 2, 10);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one large hole should classify successfully");
            assert_eq!(
                classification.0, 3,
                "n=4: Maximum hole should be classified as 3"
            );
        }
    }

    // n = 5 (non-planar): length = (5+1)²/2 = 18, max_hole = 18 - (5+1) = 12.
    mod n5 {
        use super::*;
        const N: usize = 5;

        #[test]
        fn test_classification_class1_n5() {
            // Minimal hole (length = 1)
            let puzzle = build_puzzle_with_hole(N, 3, 1);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one small hole should classify successfully");
            assert_eq!(
                classification.0, 1,
                "n=5: Minimal hole should be classified as 1"
            );
        }

        #[test]
        fn test_classification_class2_n5() {
            // Moderate hole (length = 8) yields a relative complexity just above the threshold.
            let puzzle = build_puzzle_with_hole(N, 3, 8);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one moderate hole should classify successfully");
            assert_eq!(
                classification.0, 2,
                "n=5: Moderate hole should be classified as 2"
            );
        }

        #[test]
        fn test_classification_class3_n5() {
            // Maximum hole (length = 12)
            let puzzle = build_puzzle_with_hole(N, 3, 12);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one large hole should classify successfully");
            assert_eq!(
                classification.0, 3,
                "n=5: Maximum hole should be classified as 3"
            );
        }
    }

    // n = 6 (non-planar): length = (6+1)*(6+2)/2 = 28, max_hole = 28 - (6+1) = 21.
    mod n6 {
        use super::*;
        const N: usize = 6;

        #[test]
        fn test_classification_class1_n6() {
            // Minimal hole (length = 1)
            let puzzle = build_puzzle_with_hole(N, 4, 1);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one small hole should classify successfully");
            assert_eq!(
                classification.0, 1,
                "n=6: Minimal hole should be classified as 1"
            );
        }

        #[test]
        fn test_classification_class2_n6() {
            // For n=6, choose a moderate hole length.
            // Calculations indicate that a hole length around 15 gives a relative complexity in the [0.5, 1) range.
            let puzzle = build_puzzle_with_hole(N, 4, 15);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one moderate hole should classify successfully");
            assert_eq!(
                classification.0, 2,
                "n=6: Moderate hole should be classified as 2"
            );
        }

        #[test]
        fn test_classification_class3_n6() {
            // Maximum hole (length = 21)
            let puzzle = build_puzzle_with_hole(N, 4, 21);
            let classification = classify_puzzle(&puzzle)
                .expect("Puzzle with one large hole should classify successfully");
            assert_eq!(
                classification.0, 3,
                "n=6: Maximum hole should be classified as 3"
            );
        }
    }
}
