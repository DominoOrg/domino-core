use crate::lp_models::classify_puzzle_model::Puzzle;

fn empty_positions_bounds(puzzle: &Puzzle, bounds: &mut Vec<String>, puzzle_l: usize) {
    for j in 0..puzzle_l {
        let value = if puzzle[j].is_some() { 0 } else { 1 };
        let bound = format!("y{}={}", j, value);
        bounds.push(bound);
    }
}

fn one_empty_bound(bounds: &mut Vec<String>, puzzle_l: usize) {
    let mut sum = String::new();
    for j in 0..puzzle_l {
        sum += &format!("y{}+", j);
    }
    sum.pop();
    let bound = format!("{}-{}a0<=0", sum, puzzle_l);
    bounds.push(bound);
}

fn empty_filled_nplets_bounds(bounds: &mut Vec<String>, puzzle_l: usize, n: usize) {
    let nplet_size = n;
    for j in 0..puzzle_l {
        let mut nplet = String::new();
        for offset in 0..nplet_size {
            let bound = format!("w{}-y{}<=0", j, (j + offset) % nplet_size);
            bounds.push(bound);
            nplet += &format!("y{}+", (j + offset) % puzzle_l);
        }
        nplet.pop();
        let bound = format!("w{}+y{}<=1", j, (j + nplet_size) % puzzle_l);
        bounds.push(bound);
        nplet += &format!("-y{}", (j + nplet_size) % puzzle_l);
        let bound = format!("{}-w{}<={}", nplet, j, n - 1);
        bounds.push(bound);
    }
}

fn one_empty_filled_nplets_bound(bounds: &mut Vec<String>, puzzle_l: usize) {
    let mut sum = String::new();
    for j in 0..puzzle_l {
        sum += &format!("w{}+", j);
    }
    sum.pop();
    let bound = format!("{}-{}a1<=0", sum, puzzle_l);
    bounds.push(bound);
}

fn two_empty_filled_nplets_bound(bounds: &mut Vec<String>, puzzle_l: usize) {
    let mut sum = "".to_owned();
    for j in 0..puzzle_l {
        sum += &format!("w{}+", j);
    }
    sum.pop();
    let bound = format!("{}-{}a2<=1", sum, puzzle_l);
    bounds.push(bound.clone());
}

fn populate_bounds(puzzle: &Puzzle, bounds: &mut Vec<String>, puzzle_l: usize, n: usize) {
    empty_positions_bounds(puzzle, bounds, puzzle_l);
    one_empty_bound(bounds, puzzle_l);
    empty_filled_nplets_bounds(bounds, puzzle_l, n);
    one_empty_filled_nplets_bound(bounds, puzzle_l);
    two_empty_filled_nplets_bound(bounds, puzzle_l);
}

pub fn bounds(puzzle: &Puzzle, puzzle_l: usize, n: usize) -> Vec<String> {
    // Date le variabili xij dove i è l'indice di una tessera nel tileset e j una positione nella sequenza finale
    // Devo classificare il puzzle in base a quali sono le x_ij la cui somma per ogni j è zero (la posizione nella sequenza è vuota)
    // traccio le posizioni vuote con variabili y_j dove per ogni j sum(x_ij) <= 1 - y_j
    // se c'è almeno una posizione vuota classifico come lvl1 con il vincolo sum(y_j) <= a0
    // traccio le coppie di posizioni vuote contigue con variabili z_j dove per ogni j > 0 vale y_j + y_j-1 <= z_j
    // se c'è almeno una coppia vuota contigua classifico come lvl2 con il vincolo sum(w_j) <= a1
    // traccio le coppie di posizioni vuoto pieno con variabili w_j dove per ogni j > 0 vale y_j + 1 - y_j-1 >= 2 * w_j
    // se ci sono almeno 2 coppie vuoto pieno classifico come lvl3 con il vincolo sum(w_j) <= 2 * a2
    let mut bounds = vec![];
    populate_bounds(puzzle, &mut bounds, puzzle_l, n);
    bounds
}
