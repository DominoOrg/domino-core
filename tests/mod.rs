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
    fn solve() {
        let puzzle = generate_puzzle(3, true);
        println!("{puzzle:?}");
        let solution = solve_puzzle(&puzzle).unwrap();
        println!("{puzzle:?}\n{solution:?}");
        assert_eq!(solution.len(), puzzle.len());
    }

}