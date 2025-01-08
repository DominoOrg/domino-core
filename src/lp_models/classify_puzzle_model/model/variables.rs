use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Variables {
    vars: HashSet<String>,
}

impl Variables {
    pub fn new() -> Variables {
        Variables::default()
    }

    pub fn add_variable(&mut self, name: String) {
        self.vars.insert(name);
    }

    pub fn to_vec(self) -> Vec<String> {
        self.vars.into_iter().collect::<Vec<String>>()
    }
}

fn y_variables(vars: &mut Variables, puzzle_l: usize) {
    for j in 0..puzzle_l {
        let name = format!("y{}", j);
        vars.add_variable(name);
    }
}

fn w_variables(vars: &mut Variables, puzzle_l: usize) {
    for j in 0..puzzle_l {
        let name = format!("w{}", j);
        vars.add_variable(name);
    }
}

fn a_variables(vars: &mut Variables) {
    for i in 0..3 {
        let name = format!("a{}", i);
        vars.add_variable(name);
    }
}

fn populate_variables(vars: &mut Variables, puzzle_l: usize) {
    y_variables(vars, puzzle_l);
    w_variables(vars, puzzle_l);
    a_variables(vars);
}

pub fn variables(puzzle_l: usize) -> Vec<String> {
    let mut vars = Variables::new();
    populate_variables(&mut vars, puzzle_l);
    let mut vec = vars.to_vec();
    vec.sort();
    vec
}
