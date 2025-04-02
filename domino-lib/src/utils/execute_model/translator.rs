use std::collections::HashMap;

use good_lp::{solvers::highs::HighsSolution, Expression, Solution, Variable, VariableDefinition};

#[derive(Debug)]
pub struct ResultTranslator {
    objective: Expression,
    _var_names: HashMap<Variable, String>,
    _variable_map: HashMap<String, (VariableDefinition, Variable)>,
    solution: HighsSolution,
}

impl ResultTranslator {
    pub fn new(
        objective: Expression,
        _var_names: HashMap<Variable, String>,
        _variable_map: HashMap<String, (VariableDefinition, Variable)>,
        solution: HighsSolution,
    ) -> Self {
        ResultTranslator {
            objective,
            _var_names,
            _variable_map,
            solution,
        }
    }

    pub fn _get_variables(&self) -> HashMap<String, f64> {
        let mut map = HashMap::new();
        self._variable_map
            .values()
            .map(|(_var_def, var)| {
                let name = self._var_names.get(var).unwrap();
                (name, var)
            })
            .for_each(|(name, var)| {
                let value = self.solution.value(*var);
                map.insert(name.clone(), value);
            });
        map
    }

    pub fn get_objective(&self) -> f64 {
        self.solution.eval(self.objective.clone())
    }
}
