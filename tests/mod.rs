#[cfg(test)]
mod tests {
    use domino_lib::functionalities::{generate::generate_puzzle, solve::solve_puzzle, validate::validate_puzzle};

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
}