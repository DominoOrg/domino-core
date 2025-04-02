use clap::Parser;
use domino_lib::{generate_puzzle, validate_puzzle, Puzzle, Tile};
use serde_json::Value;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    GeneratePuzzle {
        #[arg(short, long, default_value_t = 6)]
        n: u32,
        #[arg(short, long)]
        minimum_removals: u32,
        #[arg(short, long, default_value_t = true)]
        random: bool
    },
    ValidatePuzzle {
        #[arg(short, long)]
        puzzle: String,
        #[arg(short, long)]
        solution: String
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GeneratePuzzle { n, minimum_removals, random} => {
            let puzzle =  domino_lib::generate_puzzle(*n as usize, *minimum_removals as usize, *random);
            println!("{:?}", puzzle);
        },
        #[allow(unused_variables)]
        Commands::ValidatePuzzle { puzzle, solution } => {
            let puzzle = serialize_puzzle(puzzle.to_string());
            let result = validate_puzzle(&puzzle);
        }
    }
}

fn serialize_puzzle(puzzle: String) -> Puzzle {
  let result: Value = serde_json::from_str(&puzzle).unwrap();
  let mut tiles: Vec<Option<Tile>> = vec![];
  for tile in result.as_array().unwrap() {
    if tile.is_null() {
      tiles.push(None);
    } else {
      let left = tile.get(0).unwrap().as_i64().unwrap() as i32;
      let right = tile.get(1).unwrap().as_i64().unwrap() as i32;
      tiles.push(Some(Tile(left, right)));
    }
  }
  Puzzle(tiles)
}
