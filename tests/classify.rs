#[cfg(test)]
mod tests {
    use domino_lib::graph_models::classify_puzzle::classify_puzzle;
    use domino_lib::graph_models::generate_sequence::generate_solution;

    #[test]
    fn classify1() {
        for n in 2..=12 {
            let sequence = generate_solution(n, false);
            let mut puzzle = sequence
                .into_iter()
                .map(|tile| Some(tile))
                .collect::<Vec<Option<(usize, usize)>>>();
            puzzle[0] = None;
            let difficulty = classify_puzzle(&puzzle);
            assert_eq!(difficulty, 1);
        }
    }

    #[test]
    fn classify2() {
        for n in 2..=12 {
            let sequence = generate_solution(n, false);
            let mut puzzle = sequence
                .into_iter()
                .map(|tile| Some(tile))
                .collect::<Vec<Option<(usize, usize)>>>();
            for i in 0..n {
                puzzle[i] = None;
            }
            let difficulty = classify_puzzle(&puzzle);
            assert_eq!(difficulty, 2);
        }
    }

    #[test]
    fn classify3() {
        for n in 2..=12 {
            let sequence = generate_solution(n, false);
            let mut puzzle = sequence
                .into_iter()
                .map(|tile| Some(tile))
                .collect::<Vec<Option<(usize, usize)>>>();
            for i in 0..2 {
                for j in 0..n {
                    puzzle[(n + 1) * i + j] = None;
                }
            }
            let difficulty = classify_puzzle(&puzzle);
            assert_eq!(difficulty, 3);
        }
    }
}
