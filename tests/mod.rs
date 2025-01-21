#[cfg(test)]
mod tests {
    use domino_lib::{functionalities::{classify::classify_puzzle, generate::generate_puzzle, solve::solve_puzzle, validate::validate_puzzle}, types::Tile};

    fn test_suite() -> Vec<usize> {
        return vec![3,6]
    }

    #[test]
    fn generate() {     
        for n in test_suite() {
            let puzzle = generate_puzzle(n, false);
            if n % 2 == 0 {
                assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
            } else {
                assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
            }
        }

        for n in test_suite() {
            let puzzle = generate_puzzle(n, true);
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
            let puzzle = generate_puzzle(n, false);
            let solution = solve_puzzle(&puzzle).unwrap();
            assert_eq!(solution.len(), puzzle.len());                
        }

        for n in test_suite() {
            let puzzle = generate_puzzle(n, true);
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
            let puzzle = generate_puzzle(n, false);
            assert!(validate_puzzle(&puzzle).is_ok());                
        }

        // For each length an empty puzzle should result in not valid
        for n in test_suite() {
            let puzzle = vec![None; if n % 2 == 0 { (n + 1) * (n + 2) / 2 } else { (n + 1) * (n + 1) / 2 }];
            assert!(validate_puzzle(&puzzle).is_err());                
        }
    }

    #[test]
    fn classify() {
        let threshold1 = 4.0 * (1.0 / 7.0);
        let threshold2 = threshold1 + 2.0 * (1.0 / 7.0);
        for n in test_suite() {
            let max_len = 2 * n + 1;
            for desired_complexity in 1..=3 {
                let mut puzzle = if n == 6 {
                    vec![
                        Some(Tile(0,0)),
                        Some(Tile(0,1)),
                        Some(Tile(1,1)),
                        Some(Tile(1,2)),
                        Some(Tile(2,2)),
                        Some(Tile(2,3)),
                        Some(Tile(3,3)),
                        Some(Tile(3,4)),
                        Some(Tile(4,4)),
                        Some(Tile(4,5)),
                        Some(Tile(5,5)),
                        Some(Tile(5,6)),
                        Some(Tile(6,6)),
                        Some(Tile(6,0)),
                        Some(Tile(0,2)),
                        Some(Tile(2,5)),
                        Some(Tile(5,3)),
                        Some(Tile(3,6)),
                        Some(Tile(6,2)),
                        Some(Tile(2,4)),
                        Some(Tile(4,0)),
                        Some(Tile(0,3)),
                        Some(Tile(3,1)),
                        Some(Tile(1,4)),
                        Some(Tile(4,6)),
                        Some(Tile(6,1)),
                        Some(Tile(1,5)),
                        Some(Tile(5,0))
                    ]
                } else {
                    generate_puzzle(3, false)
                };
                match desired_complexity {
                    1 => {
                        let complexity = classify_puzzle(&puzzle);
                        let valid_result = validate_puzzle(&puzzle);
                        assert!(valid_result.is_ok());
                        assert!(complexity == 1)
                    },
                    2 => {
                        for i in 0..(max_len as f32 * threshold1).ceil() as usize  {
                            puzzle[i] = None;
                        }
                        let complexity = classify_puzzle(&puzzle);
                        let valid_result = validate_puzzle(&puzzle);
                        assert!(valid_result.is_ok());
                        assert!(complexity == 2)
                    },
                    3 => {
                        for i in 0..(max_len as f32 * threshold2).ceil() as usize {
                            puzzle[i] = None;
                        }
                        println!("{puzzle:?}");
                        let complexity = classify_puzzle(&puzzle);
                        let valid_result = validate_puzzle(&puzzle);
                        assert!(valid_result.is_ok());
                        assert!(complexity == 3)
                    },
                    _ => panic!("Invalid complexity")
                }
    
            }
        
        }
    }

    #[test]
    fn hardest_puzzle() {
        let puzzle = vec![
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Tile(6,0)),
            Some(Tile(0,2)),
            Some(Tile(2,5)),
            Some(Tile(5,3)),
            Some(Tile(3,6)),
            Some(Tile(6,2)),
            Some(Tile(2,4)),
            Some(Tile(4,0)),
            Some(Tile(0,3)),
            Some(Tile(3,1)),
            Some(Tile(1,4)),
            Some(Tile(4,6)),
            Some(Tile(6,1)),
            Some(Tile(1,5)),
            Some(Tile(5,0))
        ];
        let result = validate_puzzle(&puzzle);
        assert!(result.is_ok());
    }

}