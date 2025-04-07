
// pub mod benches;

mod tests {

  use domino_lib::{
      classify_puzzle, generate_puzzle, solve_puzzle, validate_puzzle, ComplexityClass,
  };

  fn test_suite() -> Vec<usize> {
      // todo!("Add more lengths to test suite");
      return vec![3, 6, 9];
  }

  #[test]
  fn test_generate() {
      test_suite().into_iter().for_each(|n| {
        (1..=3).for_each(|c| {
          let puzzle = generate_puzzle(n, c);
          if n % 2 == 0 {
              assert_eq!(puzzle.0.len(), (n + 1) * (n + 2) / 2);
          } else {
              assert_eq!(puzzle.0.len(), (n + 1) * (n + 1) / 2);
          }
        });
      });
  }

  #[test]
  fn test_solve() {
      test_suite().into_iter().for_each(|n| {
        (1..=3).for_each(|c| {
          let puzzle = generate_puzzle(n, c);
          println!("Puzzle: {puzzle:?}");
          let solution = solve_puzzle(&puzzle).unwrap();
          println!("Solution: {solution:?}");
          assert_eq!(solution.len(), puzzle.0.len());
      });
    });
  }

  #[test]
  fn test_validate() {
      test_suite().into_iter().for_each(|n| {
        (1..=3).for_each(|c| {
          // For each length a puzzle with a single tile missing is always valid
          let puzzle = generate_puzzle(n, c);
          println!("Puzzle: {}",serde_json::to_string(&puzzle).unwrap());
          let solution = solve_puzzle(&puzzle).unwrap();
          let result = validate_puzzle(&puzzle, &solution);
          assert!(result.is_ok());
      });
    });
  }

  #[test]
  fn test_classify() {
      test_suite().into_iter().for_each(|n| {
          (1..=3)
              .into_iter()
              .map(|c| ComplexityClass::new(c).unwrap())
              .for_each(|expected_complexity| {
                  let puzzle = generate_puzzle(n, expected_complexity.0);
                  println!("Puzzle: {puzzle:?}");
                  let computed_complexity =
                      classify_puzzle(&puzzle).expect("Failed to classify puzzle: {puzzle:?}");
                  assert_eq!(computed_complexity, expected_complexity);
              })
      });
  }

  #[test]
  fn test_all() {
      test_suite().into_iter().for_each(|n| {
          (1..=3)
              .into_iter()
              .map(|c| ComplexityClass::new(c).unwrap())
              .for_each(|expected_complexity| {
                  let puzzle = generate_puzzle(n, expected_complexity.0);
                  let solution = solve_puzzle(&puzzle).unwrap();
                  let _ = validate_puzzle(&puzzle, &solution)
                      .map(|_solution| {
                          let computed_complexity = classify_puzzle(&puzzle)
                              .expect("Failed to classify puzzle: {puzzle:?}");
                          assert_eq!(computed_complexity, expected_complexity);
                      });
              });
      });
  }

}
