pub(super) mod adapter;
pub(super) mod parser;

#[cfg(feature = "lpmodels")]
use std::collections::HashMap;
#[cfg(feature = "lpmodels")]
use adapter::Adapter;
#[cfg(feature = "lpmodels")]
use parser::ModelParser;

#[cfg(feature = "lpmodels")]
pub fn execute(model: String) -> Result<HashMap<String, f64>, String> {
    let mut parser = ModelParser::new();
    parser.parse(&model);
    let adapter = Adapter::new();
    let result = Adapter::adapt(adapter, &parser);
    result.map_err(|err| err.to_string())
}
