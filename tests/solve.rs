#[cfg(test)]
mod test {

    use domino_lib::domino_types::puzzle::Puzzle;
    use domino_lib::graph_models::generate_sequence::generate_solution;
    use domino_lib::graph_models::solve_puzzle::solve;

    #[test]
    fn solve_test() {
        for n in 2..=12 {
            let sequence = generate_solution(n, false);
            let mut puzzle = sequence
                .clone()
                .into_iter()
                .map(|tile| {
                    Some((
                        i32::from_str_radix(&tile.0, 10).unwrap() as usize,
                        i32::from_str_radix(&tile.1, 10).unwrap() as usize,
                    ))
                })
                .collect::<Vec<Option<(usize, usize)>>>();
            puzzle[0] = None;
            let puzzle = Puzzle::from(puzzle);
            let solved = solve(&puzzle).unwrap();
            assert_eq!(solved, sequence);
        }
    }
}
