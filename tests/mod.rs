#[cfg(test)]
mod tests {
    use domino_lib::{classify_puzzle, generate_puzzle, solve_puzzle, validate_puzzle, Puzzle, Tile};

    fn test_suite() -> Vec<usize> {
        todo!("Add more lengths to test suite");
        return vec![6]
    }

    #[test]
    fn generate() {
        for n in test_suite() {
            let puzzle = generate_puzzle(n, 1, false);
            if n % 2 == 0 {
                assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
            } else {
                assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
            }
        }

        for n in test_suite() {
            let puzzle = generate_puzzle(n, 1, true);
            if n % 2 == 0 {
                assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
            } else {
                assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
            }
        }
    }

    #[test]
    fn solve() {
        for n in test_suite() {
            let puzzle = generate_puzzle(n, 1, false);
            let solution = solve_puzzle(&puzzle).unwrap();
            assert_eq!(solution.len(), puzzle.len());
        }

        for n in test_suite() {
            let puzzle = generate_puzzle(n, 1, true);
            let solution = solve_puzzle(&puzzle);
            if let Ok(solution) = solution {
                assert_eq!(solution.len(), puzzle.len());
            }
        }
    }

    #[test]
    fn validate() {
        // For each length a puzzle with a single tile missing is always valid
        for n in test_suite() {
            let puzzle = generate_puzzle(n, 1, false);
            let solution = solve_puzzle(&puzzle).unwrap();
            assert!(validate_puzzle(&puzzle, &solution).is_ok());
        }

        // For each length an empty puzzle should result in not valid
        for n in test_suite() {
            let puzzle = vec![None; if n % 2 == 0 { (n + 1) * (n + 2) / 2 } else { (n + 1) * (n + 1) / 2 }];
            let solution = solve_puzzle(&puzzle).unwrap();
            assert!(validate_puzzle(&puzzle, &solution).is_err());
        }
    }

    fn mock_puzzle(n: usize, c: usize) -> Puzzle {
      let l = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
      let mut puzzle: Puzzle = vec![Some(Tile::from((0,0))); l];
      let max_hole = (l as f32 - (n as f32 / 2.0).floor()) as usize;
      let log_factor: f32 = match c {
        1 => 1.0 / puzzle.len() as f32,
        2 => 4.0 / 7.0,
        3 => 6.0 / 7.0,
        _ => 0.0
      };
      for i in 0..(max_hole as f32  * log_factor.sqrt()).ceil() as usize {
        puzzle[i] = None;
      }
      puzzle
    }

    #[test]
    fn classify() {
      for mock_c in 1..=3 {
        let puzzle = mock_puzzle(6, mock_c);
        let c = classify_puzzle(&puzzle);
        assert_eq!(c, mock_c);
      }
    }

    // #[test]
    // fn hardest_puzzle() {
    //     let puzzle = vec![
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         Some(Tile(6,0)),
    //         Some(Tile(0,2)),
    //         Some(Tile(2,5)),
    //         Some(Tile(5,3)),
    //         Some(Tile(3,6)),
    //         Some(Tile(6,2)),
    //         Some(Tile(2,4)),
    //         Some(Tile(4,0)),
    //         Some(Tile(0,3)),
    //         Some(Tile(3,1)),
    //         Some(Tile(1,4)),
    //         Some(Tile(4,6)),
    //         Some(Tile(6,1)),
    //         Some(Tile(1,5)),
    //         Some(Tile(5,0))
    //     ];
    //     let solution = solve_puzzle(&puzzle).unwrap();
    //     let result = validate_puzzle(&puzzle, &solution);
    //     assert!(result.is_ok());
    // }

    #[test]
    fn all() {
        for n in test_suite() {
            println!("n: {}", n);
            let l = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
            let minimum_tiles = (n as f32/2 as f32).floor();
            let max_hole = (l as f32 - minimum_tiles) as usize;

            for c in 1..=3 {
                let minimum_removals = match c {
                    1 => 1,
                    2 => 4 * max_hole / 7,
                    3 => 6 * max_hole / 7,
                    _ => 0
                };
                let puzzle = generate_puzzle(n, minimum_removals, false);
                if let Ok(solution) = solve_puzzle(&puzzle) {
                    if validate_puzzle(&puzzle, &solution).is_ok() {
                        assert_eq!(c, classify_puzzle(&puzzle));
                    }
                }
            }
        }
    }
}
