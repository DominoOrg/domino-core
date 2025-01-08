use crate::lp_models::generate_sequence_model::model::variables::create_tileset;
use itertools::Itertools;
use std::collections::HashMap;

pub fn parse_to_sequence(
    variables_values: HashMap<String, f64>,
    n: usize,
) -> anyhow::Result<Vec<(usize, usize)>> {
    let tileset_digits = (((n + 1).pow(2) as f32).log10().floor() + 1.0) as usize;
    let sequence_len = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1).pow(2) / 2
    };
    let sequence_digits = ((sequence_len as f32).log10().floor() + 1.0) as usize;
    // Parsing the result of the solver execution
    let tileset = create_tileset(n);
    let sequence = variables_values
        .iter()
        .filter(|(_name, &value)| value == 1.0)
        .map(|(name, _)| {
            let index = name[1..(1 + tileset_digits)].parse::<usize>().unwrap();
            let position = name[(1 + tileset_digits)..(1 + tileset_digits + sequence_digits)]
                .parse::<usize>()
                .unwrap();
            (position, tileset[index])
        })
        .sorted_by_key(|(position_index, _)| *position_index)
        .map(|(_, tile)| tile)
        .collect::<Vec<(usize, usize)>>();
    Ok(sequence)
}
