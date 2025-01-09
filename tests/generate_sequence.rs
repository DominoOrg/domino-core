#[cfg(test)]
mod tests {
    use domino_lib::graph_models::generate_sequence::generate_solution;

    #[test]
    fn generate_sequence() {
        for n in 2..=12 {
            let sequence = generate_solution(n, false);
            if n % 2 == 0 {
                assert_eq!(sequence.len(), (n + 1) * (n + 2) / 2);
            } else {
                assert_eq!(sequence.len(), (n + 1) * (n + 1) / 2);
            }
        }
    }
}
