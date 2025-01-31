use std::collections::HashMap;

use good_lp::{solvers::highs::HighsSolution, Expression, Solution, Variable, VariableDefinition};

#[derive(Debug)]
pub struct ResultTranslator {
    objective: Expression,
    var_names: HashMap<Variable, String>,
    variable_map: HashMap<String, (VariableDefinition, Variable)>,
    solution: HighsSolution
}

impl ResultTranslator {
    pub fn new(objective: Expression, var_names: HashMap<Variable, String>, variable_map: HashMap<String, (VariableDefinition, Variable)>, solution: HighsSolution) -> Self {
        ResultTranslator {
            objective,
            var_names,
            variable_map,
            solution
        }
    }

    pub fn get_variables(&self) -> HashMap<String, f64> {
        let mut map = HashMap::new();
        self.variable_map.values().map(|(_var_def, var)| {
            let name = self.var_names.get(var).unwrap();
            (name, var)
        }).for_each(|(name, var)| {
            let value = self.solution.value(*var);
            map.insert(name.clone(), value);
        });
        map      
      }
    
      pub fn get_objective(&self) -> f64 {
        self.solution.eval(self.objective.clone())
      }


}