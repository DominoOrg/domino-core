use crate::model_execution_lib::execute;
use model::compute_model;
use parse::parse_to_sequence;
mod model;
mod parse;

pub fn generate_sequence(n: usize, random: bool) -> Result<Vec<(usize, usize)>, String> {
    let model = compute_model(n, random);
    let result = execute(model);
    if let Ok(variables) = result {
        let result = parse_to_sequence(variables, n);
        let sequence: Vec<(usize, usize)> =
            result.expect("Couldn't parse the result of the solver");
        println!("Model produced the sequence: {:?}", sequence);
        Ok(sequence)
    } else {
        Err(result.err().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::lp_models::generate_sequence_model::generate_sequence;

    #[test]
    fn integration() {
        for n in 2..=4 {
            let result = generate_sequence(n, false);
            let sequence = result.as_ref().unwrap();
            assert!(result.is_ok());
            let expected_len = if n % 2 == 0 {
                (n + 1) * (n + 2) / 2
            } else {
                (n + 1).pow(2) / 2
            };
            assert_eq!(sequence.len(), expected_len);
        }
    }
}
