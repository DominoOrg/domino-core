use std::collections::HashMap;

// Struct representing a Variable with label, tile, and position.
#[derive(Debug, Clone)]
pub struct Variable {
    pub label: String,
    pub tile: (usize, usize),
    pub position: usize,
}

// Struct representing the Variables collection, with HashMaps for lookup by label, tile, and position.
#[derive(Debug, Clone)]
pub struct Variables {
    pub vars: Vec<Variable>,
    pub by_label: HashMap<String, Variable>,
    pub by_tile: HashMap<(usize, usize), Vec<Variable>>,
    pub by_position: HashMap<usize, Vec<Variable>>,
}

impl Variables {
    fn new() -> Self {
        Variables {
            vars: Vec::new(),
            by_label: HashMap::new(),
            by_tile: HashMap::new(),
            by_position: HashMap::new(),
        }
    }

    fn insert(&mut self, variable: Variable) {
        // Helper function to insert or update a map entry.
        fn insert_or_update<K: std::cmp::Eq + std::hash::Hash, V: Clone>(
            map: &mut HashMap<K, Vec<V>>,
            key: K,
            value: V,
        ) {
            map.entry(key).or_insert_with(Vec::new).push(value);
        }
        self.vars.push(variable.clone());
        self.by_label
            .insert(variable.label.clone(), variable.clone());
        insert_or_update(&mut self.by_tile, variable.tile, variable.clone());
        insert_or_update(&mut self.by_position, variable.position, variable);
    }
}

// Function to create a tileset based on N
pub fn create_tileset(n: usize) -> Vec<(usize, usize)> {
    let length: usize = (n + 1).pow(2);
    let mut tileset: Vec<(usize, usize)> = (0..length)
        .map(|i| (i / (n + 1), i % (n + 1)))
        .collect::<Vec<(usize, usize)>>();

    if n % 2 == 1 {
        tileset.retain(|&(i, j)| {
            !(i <= j && j == i + (n + 1) / 2) && !(i > j && i == j + (n + 1) / 2)
        });
    }

    tileset
}

// Function to generate combinations of tiles and positions into Variables.
fn generate_combinations(tileset: Vec<(usize, usize)>, n: usize) -> Vec<Variable> {
    let sequence_length: usize = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1).pow(2) / 2
    };
    let tileset_digits = ((tileset.len() - 1) as f32).log10().floor() as usize + 1;
    let sequence_digits = ((sequence_length - 1) as f32).log10().floor() as usize + 1;
    let positions: Vec<usize> = (0..sequence_length).collect::<Vec<usize>>();
    tileset
        .iter()
        .enumerate()
        .flat_map(|(tile_index, &tile)| {
            positions.iter().map(move |&position| {
                let label: String = format!(
                    "x{}{}",
                    format_on_n_digits(tile_index as usize, tileset_digits),
                    format_on_n_digits(position, sequence_digits)
                );
                let variable = Variable {
                    label,
                    tile,
                    position,
                };
                variable
            })
        })
        .collect::<Vec<Variable>>()
}

// Helper function to format numbers with padding.
fn format_on_n_digits(number: usize, digits: usize) -> String {
    // format!("{:01}", number) // Modify as needed for digit padding.
    return format!("{:0width$}", number, width = digits);
}

// Function to initialize Variables with combinations, filtered by randomness.
fn init_variables(combinations: Vec<Variable>) -> Variables {
    let mut vars = Variables::new();

    for el in combinations {
        vars.insert(el);
    }

    vars
}

// Main function to create variables based on N and random flag.
pub fn variables(n: usize) -> Variables {
    let tileset = create_tileset(n);
    let mapped_variables = generate_combinations(tileset, n);
    init_variables(mapped_variables)
}
