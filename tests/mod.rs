pub mod benches;

use domino_lib::{
    classify_puzzle, generate_puzzle, solve_puzzle, validate_puzzle, ComplexityClass,
};

fn test_suite() -> Vec<usize> {
    // todo!("Add more lengths to test suite");
    return vec![3, 4];
}

#[test]
fn test_generate() {
    test_suite().into_iter().for_each(|n| {
        let puzzle = generate_puzzle(n, 1, false);
        if n % 2 == 0 {
            assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
        } else {
            assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
        }

        let puzzle = generate_puzzle(n, 1, true);
        if n % 2 == 0 {
            assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
        } else {
            assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
        }
    });
}

#[test]
fn test_solve() {
    test_suite().into_iter().for_each(|n| {
        let puzzle = generate_puzzle(n, 1, false);
        println!("Puzzle: {puzzle:?}");
        let solution = solve_puzzle(&puzzle).unwrap();
        println!("Solution: {solution:?}");
        assert_eq!(solution.len(), puzzle.len());

        let puzzle = generate_puzzle(n, 1, true);
        println!("Puzzle: {puzzle:?}");
        let solution = solve_puzzle(&puzzle);
        println!("Solution: {solution:?}");
        if let Ok(solution) = solution {
            assert_eq!(solution.len(), puzzle.len());
        }
    });
}

#[test]
fn test_validate() {
    test_suite().into_iter().for_each(|n| {
        // For each length a puzzle with a single tile missing is always valid
        let puzzle = generate_puzzle(n, 1, false);
        let solution = solve_puzzle(&puzzle).unwrap();
        let result = validate_puzzle(&puzzle, &solution);
        assert!(result.is_ok());

        // For each length an empty puzzle should result in not valid
        let puzzle = vec![
            None;
            if n % 2 == 0 {
                (n + 1) * (n + 2) / 2
            } else {
                (n + 1) * (n + 1) / 2
            }
        ];
        let solution = solve_puzzle(&puzzle).unwrap();
        let result = validate_puzzle(&puzzle, &solution);
        assert!(result.is_err());
    });
}

#[test]
fn test_classify() {
    test_suite().into_iter().for_each(|n| {
        let l = if n % 2 == 0 {
            (n + 1) * (n + 2) / 2
        } else {
            (n + 1) * (n + 1) / 2
        };
        let max_hole: usize = if n >= 4 { n + 1 } else { (n + 1) * 2 - 1 };
        (1..=3)
            .into_iter()
            .map(|c| ComplexityClass::new(c).unwrap())
            .for_each(|expected_complexity| {
                let log_factor = match expected_complexity.0 {
                    1 => 1.0 / l as f32,
                    2 => 4.0 / 7.0,
                    3 => 6.0 / 7.0,
                    _ => 0.0,
                };
                let minimum_removals = (max_hole as f32 * log_factor.sqrt()).ceil() as usize;
                let puzzle = generate_puzzle(n, minimum_removals, false);
                let computed_complexity =
                    classify_puzzle(&puzzle).expect("Failed to classify puzzle: {puzzle:?}");
                assert_eq!(computed_complexity, expected_complexity);
            })
    });
}

#[test]
fn test_all() {
    test_suite().into_iter().for_each(|n| {
        let l = if n % 2 == 0 {
            (n + 1) * (n + 2) / 2
        } else {
            (n + 1) * (n + 1) / 2
        };
        let max_hole: usize = if n >= 4 { n + 1 } else { (n + 1) * 2 - 1 };

        (1..=3)
            .into_iter()
            .map(|c| ComplexityClass::new(c).unwrap())
            .for_each(|expected_complexity| {
                let log_factor = match expected_complexity.0 {
                    1 => 1.0 / l as f32,
                    2 => 4.0 / 7.0,
                    3 => 6.0 / 7.0,
                    _ => 0.0,
                };
                let minimum_removals = (max_hole as f32 * log_factor.sqrt()).ceil() as usize;
                let puzzle = generate_puzzle(n, minimum_removals, false);
                solve_puzzle(&puzzle)
                    .ok()
                    .filter(|solution| validate_puzzle(&puzzle, solution).is_ok())
                    .map(|_solution| {
                        let computed_complexity = classify_puzzle(&puzzle)
                            .expect("Failed to classify puzzle: {puzzle:?}");
                        assert_eq!(computed_complexity, expected_complexity);
                    });
            });
    });
}
