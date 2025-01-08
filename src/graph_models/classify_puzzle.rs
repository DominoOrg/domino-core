fn get_n(puzzle: &Vec<Option<(String, String)>>) -> usize {
    let l = puzzle.len();
    let n_p = (-3.0 + (1.0 + 8.0 * (l as f64)).sqrt()) / 2.0;
    let n_d = (-2.0 + (8.0 * (l as f64)).sqrt()) / 2.0;
    let n = if (n_p - n_p.floor()).abs() == 0.0 {
        n_p.floor() as usize
    } else {
        n_d.floor() as usize
    };
    n
}

fn is_overlap(nplet_bounds1: &(usize, usize), nplet_bounds2: &(usize, usize)) -> bool {
    nplet_bounds1.0 <= nplet_bounds2.1 && nplet_bounds2.0 <= nplet_bounds1.1
}

pub fn classify_puzzle(puzzle: &Vec<Option<(String, String)>>) -> usize {
    let n = get_n(puzzle);
    let mut indicators = vec![false, false, false];
    let nplet_size = n + 1;
    if puzzle.iter().any(|tile| tile.is_none()) {
        indicators[0] = true;
    }
    let enumerated_puzzle = puzzle
        .iter()
        .enumerate() // Gives index and value for each element
        .collect::<Vec<_>>();
    let nplets = enumerated_puzzle // Collect into a Vec to make a slice with (index, value) pairs
        .windows(nplet_size)
        .filter(|nplet| {
            nplet[0..=nplet_size - 2]
                .iter()
                .all(|(_, tile)| tile.is_none())
                && nplet[nplet_size - 1].1.is_some()
        })
        .map(|nplet| nplet.to_vec())
        .collect::<Vec<Vec<(usize, &Option<(String, String)>)>>>();
    let nplets_bounds = nplets
        .clone()
        .iter()
        .map(|nplet| {
            let start = nplet.into_iter().map(|&(index, _)| index).min().unwrap();
            let end = nplet.into_iter().map(|&(index, _)| index).max().unwrap();
            (start, end)
        })
        .collect::<Vec<(usize, usize)>>();
    if nplets.len() >= 1 {
        indicators[1] = true;
    }
    let mut disjoint_nplets = false;
    for (i, nplet_bounds1) in nplets_bounds.iter().enumerate() {
        for (j, nplet_bounds2) in nplets_bounds.iter().enumerate() {
            if i != j && !is_overlap(nplet_bounds1, nplet_bounds2) {
                for index in nplet_bounds1.1..=nplet_bounds2.0 {
                    if puzzle[index].is_some() {
                        disjoint_nplets = true;
                    }
                }
            }
        }
    }
    if nplets.len() >= 2 && disjoint_nplets {
        indicators[2] = true;
    }
    indicators
        .into_iter()
        .map(|indicator| if indicator { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::graph_models::classify_puzzle::classify_puzzle;
    use crate::graph_models::generate_sequence::generate_solution;

    #[test]
    fn classify1() {
        for n in 2..=12 {
            let sequence = generate_solution(n, false);
            let mut puzzle = sequence
                .into_iter()
                .map(|tile| Some(tile))
                .collect::<Vec<Option<(String, String)>>>();
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
                .collect::<Vec<Option<(String, String)>>>();
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
                .collect::<Vec<Option<(String, String)>>>();
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
