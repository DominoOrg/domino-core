mod parser;
mod adapter;

use std::collections::HashMap;

use adapter::Adapter;
use parser::ModelParser;

pub fn execute(model: String) -> Result<HashMap<String, f64>, String> {
  let mut parser = ModelParser::new();
  parser.parse(&model);
  let adapter = Adapter::new();
  let result = Adapter::adapt(adapter, &parser);
  result.map_err(|err| err.to_string())
}