#[cfg(test)]
mod tests {
    use domino_lib::functionalities::generate::generate_puzzle;

    #[test]
    fn generate() {     
        for n in 3..=12 {
            let puzzle = generate_puzzle(n, false);
            println!("{n}\n{puzzle:#?}");
            if n % 2 == 0 {
                assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
            } else {
                assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
            }
        }
    }


}