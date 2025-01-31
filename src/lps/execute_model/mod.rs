mod parser;
mod adapter;
mod translator;

use adapter::Adapter;
use good_lp::ResolutionError;
use parser::ModelParser;
pub use translator::ResultTranslator;

pub struct Model {
}

impl Model {

  pub fn execute(model: String) -> Result<ResultTranslator, ResolutionError> {
    let mut parser = ModelParser::new();
    parser.parse(&model);
    let adapter = Adapter::new();
    let solved_problem = Adapter::adapt(adapter, &parser);
    solved_problem
  }

}

