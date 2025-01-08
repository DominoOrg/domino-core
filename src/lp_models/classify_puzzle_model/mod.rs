mod model;

use crate::model_execution_lib::execute;
use model::compute_model;

type Tile = (usize, usize);
type Puzzle = Vec<Option<Tile>>;

pub fn classify_puzzle(puzzle: &Puzzle, n: usize) -> Result<usize, String> {
    let model = compute_model(&puzzle, n);
    println!("{}", model);
    let result = execute(model);
    if let Ok(variables) = result {
        println!("{:?}", variables);
        let difficulty = variables
            .iter()
            .filter(|(name, _)| name.starts_with("a"))
            .map(|(_, &value)| value)
            .reduce(|acc, el| acc + el)
            .unwrap_or_default();
        Ok(difficulty as usize)
    } else {
        Err(result.err().unwrap())
    }
}

#[cfg(test)]
mod tests {

    use crate::lp_models::generate_sequence_model::generate_sequence;

    use crate::lp_models::classify_puzzle_model::classify_puzzle;

    #[test]
    fn classify1() {
        for n in 2..=4 {
            let sequence = generate_sequence(n, false).unwrap();
            let mut puzzle: Vec<Option<(usize, usize)>> = sequence
                .clone()
                .into_iter()
                .map(|tile| Some(tile))
                .collect();
            puzzle[0] = None;
            let difficulty = classify_puzzle(&puzzle, n).unwrap();
            assert_eq!(difficulty, 1);
        }
    }

    #[test]
    fn classify2() {
        for n in 2..=4 {
            let sequence = generate_sequence(n, false).unwrap();
            let mut puzzle: Vec<Option<(usize, usize)>> = sequence
                .clone()
                .into_iter()
                .map(|tile| Some(tile))
                .collect();
            for i in 0..n {
                puzzle[i] = None;
            }
            let difficulty = classify_puzzle(&puzzle, n).unwrap();
            assert_eq!(difficulty, 2);
        }
    }

    #[test]
    fn classify3() {
        for n in 2..=4 {
            let sequence = generate_sequence(n, false).unwrap();
            let mut puzzle: Vec<Option<(usize, usize)>> = sequence
                .clone()
                .into_iter()
                .map(|tile| Some(tile))
                .collect();
            for i in 0..2 {
                for j in 0..n {
                    puzzle[(n + 1) * i + j] = None;
                }
            }
            let difficulty = classify_puzzle(&puzzle, n).unwrap();
            assert_eq!(difficulty, 3);
        }
    }
}
