#[cfg(test)]
mod tests {
    use domino_lib::functionalities::{generate::generate_puzzle, solve::solve_puzzle};

    #[test]
    fn generate() {     
        for n in 3..=12 {
            let puzzle = generate_puzzle(n, false);
            if n % 2 == 0 {
                assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
            } else {
                assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
            }
        }
    }

    #[test]
    fn planar_solve() {
        for n in 2..=3 {
            let puzzle = generate_puzzle(n, false);
            println!("{puzzle:?}");
            let solution = solve_puzzle(&puzzle).unwrap();
            println!("{puzzle:?}\n{solution:?}");
            assert_eq!(solution.len(), puzzle.len());                
        }
    }

}