// Global constant defining the default number of classes (thresholds)
const NUMBER_OF_CLASSES: usize = 3; // Default value set to 3

/// Computes the adjusted Fibonacci number at index `n`.
///
/// This function ensures that the Fibonacci sequence starts as:
/// ```text
/// F'(1) = 1, F'(2) = 2, F'(3) = 3, F'(4) = 5, ...
/// ```
///
/// # Arguments
///
/// * `n` - The index in the translated Fibonacci sequence (1-based).
///
/// # Returns
///
/// The `n`-th number in the translated Fibonacci sequence.
///
/// # Examples
///
/// ```
/// let fib = translated_fibonacci(4);
/// assert_eq!(fib, 5);
/// ```
fn translated_fibonacci(n: usize) -> usize {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut a = 1;
    let mut b = 2;
    for _ in 3..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

/// Computes the denominator for threshold calculations.
///
/// The denominator is the sum of the first `NUMBER_OF_CLASSES` translated Fibonacci numbers:
///
/// ```text
/// D(NUMBER_OF_CLASSES) = F'(1) + F'(2) + ... + F'(NUMBER_OF_CLASSES)
/// ```
///
/// # Returns
///
/// The sum of the first `NUMBER_OF_CLASSES` Fibonacci-translated numbers.
///
/// # Examples
///
/// ```
/// let denom = compute_denominator();
/// assert_eq!(denom, 6); // For NUMBER_OF_CLASSES = 3: (1 + 2 + 3)
/// ```
fn compute_denominator() -> usize {
    (1..=NUMBER_OF_CLASSES)
        .map(|i| translated_fibonacci(i))
        .sum()
}

/// Computes an intermediate value for the threshold calculation.
///
/// This corresponds to an individual term in the summation:
///
/// ```text
/// S(k, NUMBER_OF_CLASSES) = F'(k) / D(NUMBER_OF_CLASSES)
/// ```
///
/// # Arguments
///
/// * `k` - The index of the intermediate value (1-based).
/// * `denominator` - The precomputed sum of the first `NUMBER_OF_CLASSES` Fibonacci-translated numbers.
///
/// # Returns
///
/// A tuple containing:
/// - `numerator` - The `k`-th translated Fibonacci number.
/// - `denominator` - The total sum of the first `NUMBER_OF_CLASSES` Fibonacci-translated numbers.
///
/// # Examples
///
/// ```
/// let (num, denom) = compute_intermediate_value(2, 6);
/// assert_eq!(num, 2);
/// assert_eq!(denom, 6);
/// ```
fn compute_intermediate_value(k: usize, denominator: usize) -> (usize, usize) {
    let numerator = translated_fibonacci(k);
    (numerator, denominator)
}

/// Computes the threshold value for a given `k`.
///
/// This is the cumulative sum of the first `k` intermediate values:
///
/// ```text
/// Threshold(k, NUMBER_OF_CLASSES) = (F'(1) + F'(2) + ... + F'(k)) / D(NUMBER_OF_CLASSES)
/// ```
///
/// # Arguments
///
/// * `k` - The threshold index (1-based).
///
/// # Returns
///
/// The computed threshold as an `f32`, representing a fraction of the total.
///
/// # Examples
///
/// ```
/// let threshold = compute_threshold(3);
/// assert_eq!(threshold, 6.0 / 6.0);
/// ```
pub(super) fn compute_threshold(k: usize) -> f32 {
    let denominator = compute_denominator() as f32;
    let mut num_sum = 0;

    for i in 1..=k {
        let (num, _) = compute_intermediate_value(i, denominator as usize);
        num_sum += num;
    }

    num_sum as f32 / denominator
}

/// Recursively finds the index `k` at which the computed threshold surpasses a given `value`.
///
/// The function starts from `k = 1` and increases `k` until:
/// ```text
/// Threshold(k, NUMBER_OF_CLASSES) > value
/// ```
///
/// # Arguments
///
/// * `value` - The floating-point value to compare thresholds against.
///
/// # Returns
///
/// The first index `k` where the threshold surpasses `value`.
///
/// # Examples
///
/// ```
/// let index = find_threshold_index(0.5);
/// assert_eq!(index, 3);
/// ```
pub(super) fn find_threshold_index(value: f32) -> usize {
    let mut k = 1;
    while k <= NUMBER_OF_CLASSES {
        let threshold = compute_threshold(k);
        if threshold > value {
            return k;
        }
        k += 1;
    }
    NUMBER_OF_CLASSES // Return the last index if value is not surpassed
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests `translated_fibonacci` function with deterministic inputs.
    /// Verifies the computed Fibonacci-like sequence for fixed values.
    #[test]
    fn test_formula_translated_fibonacci() {
        let test_values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected_values = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

        for (n, expected) in test_values.iter().zip(expected_values.iter()) {
            assert_eq!(translated_fibonacci(*n), *expected);
        }
    }

    /// Tests `compute_denominator` by comparing its output against the expected sum
    /// of the first `NUMBER_OF_CLASSES` Fibonacci-translated numbers.
    #[test]
    fn test_formula_compute_denominator() {
        let expected: usize = (1..=NUMBER_OF_CLASSES).map(translated_fibonacci).sum();
        assert_eq!(compute_denominator(), expected);
    }

    /// Tests `compute_intermediate_value` with deterministic `k` values.
    /// It verifies that the function correctly returns the Fibonacci-translated number and denominator.
    #[test]
    fn test_formula_compute_intermediate_value() {
        let denominator = compute_denominator();
        let test_values = [1, 2, 3];

        for &k in &test_values {
            let expected = (translated_fibonacci(k), denominator);
            assert_eq!(compute_intermediate_value(k, denominator), expected);
        }
    }

    /// Tests `compute_threshold` with deterministic `k` values.
    /// Verifies that the computed threshold matches the expected ratio.
    #[test]
    fn test_formula_compute_threshold() {
        let test_values = [1, 2, 3];
        let denominator = compute_denominator() as f32;

        for &k in &test_values {
            let num_sum = (1..=k).map(translated_fibonacci).sum::<usize>() as f32;
            assert_eq!(compute_threshold(k), num_sum / denominator);
        }
    }

    /// Tests `find_threshold_index` with deterministic values between 0.0 and 1.0.
    /// Ensures that the function returns the correct index where the threshold surpasses `value`.
    #[test]
    fn test_formula_find_threshold_index() {
        let test_values = [0.0, 0.1, 0.3, 0.6, 1.0];
        let expected_indices = [1, 1, 2, 3, 3];

        for (value, expected) in test_values.iter().zip(expected_indices.iter()) {
            assert_eq!(find_threshold_index(*value), *expected);
        }
    }
}
