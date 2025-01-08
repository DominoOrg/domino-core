mod bounds;
mod variables;

use variables::variables;

use bounds::bounds;

use crate::lp_models::classify_puzzle_model::Puzzle;

fn objective() -> String {
    String::from("a0+a1+a2")
}

pub fn compute_model(puzzle: &Puzzle, n: usize) -> String {
    let puzzle_l = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1).pow(2) / 2
    };

    let mut model = String::from("Minimize\n  obj: ");
    model += (objective() + "\nSubject To\n").as_str();
    for (i, bound) in bounds(puzzle, puzzle_l, n).into_iter().enumerate() {
        model += format!("  c{}: {}\n", i, bound).as_str();
    }
    model += "Binary\n";
    for variable in variables(puzzle_l) {
        model += &format!("  {}\n", variable);
    }
    model += "End";
    model
}
