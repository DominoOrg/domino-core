#[cfg(feature = "lpmodels")]
use std::collections::HashMap;

#[cfg(feature = "lpmodels")]
use good_lp::{
    default_solver, variable, Constraint, Expression, ProblemVariables, ResolutionError, Solution,
    SolverModel, Variable, VariableDefinition,
};

#[cfg(feature = "lpmodels")]
use super::parser::{BoundType, ModelParser, ObjectiveType};

#[cfg(feature = "lpmodels")]
pub struct Adapter {
    vars: ProblemVariables,
    pub variable_map: HashMap<String, (VariableDefinition, Variable)>,
    var_names: HashMap<Variable, String>,
    objective_direction: ObjectiveType,
    objective: Expression,
    bounds: Vec<Constraint>,
}

#[cfg(feature = "lpmodels")]
impl Adapter {
    pub fn new() -> Adapter {
        Adapter {
            vars: ProblemVariables::new(),
            variable_map: HashMap::new(),
            var_names: HashMap::new(),
            objective_direction: ObjectiveType::Minimize,
            objective: Expression::from_other_affine(0.0),
            bounds: Vec::new(),
        }
    }

    pub fn adapt(mut self, parser: &ModelParser) -> Result<HashMap<String, f64>, ResolutionError> {
        self = self
            .adapt_variables(parser)
            .adapt_objective(parser)
            .adapt_bounds(parser);
        let mut problem = if self.objective_direction == ObjectiveType::Minimize {
            self.vars.minimise(self.objective.clone())
        } else {
            self.vars.maximise(self.objective.clone())
        }
        .using(default_solver);
        self.bounds.into_iter().for_each(|constraint| {
            problem.add_constraint(constraint);
        });
        let model = problem.solve();
        if let Ok(solution) = model {
            let mut map = HashMap::new();
            self.variable_map
                .values()
                .map(|(_var_def, var)| {
                    let name = self.var_names.get(var).unwrap();
                    (name, var)
                })
                .for_each(|(name, var)| {
                    let value = solution.value(*var);
                    map.insert(name.clone(), value);
                });
            Ok(map)
        } else {
            Err(model.err().unwrap())
        }
    }

    fn adapt_variables(mut self, parser: &ModelParser) -> Self {
        for variable_name in parser.binary_variables.clone() {
            let def_variable = variable().binary().name(variable_name.clone());
            let variable = self.vars.add(def_variable.clone());
            self.variable_map
                .insert(variable_name.clone(), (def_variable, variable));
            self.var_names.insert(variable, variable_name);
        }
        self
    }

    fn adapt_objective(mut self, parser: &ModelParser) -> Self {
        let objective = parser.objective.clone();
        self.objective_direction = objective.objective_type;
        self.objective = Expression::from_other_affine(0.0);
        for term in objective.terms.0 {
            if let Some(variable) = self.variable_map.get(&term.1) {
                self.objective
                    .add_mul(term.0, Expression::from_other_affine(variable.1));
            }
        }
        self.objective += objective.terms.1;
        self
    }

    fn adapt_bounds(mut self, parser: &ModelParser) -> Self {
        for bound in &parser.bounds {
            let mut lhs_expr = Expression::from_other_affine(0.0);
            bound.lhs.0.iter().for_each(|(coef, var_name)| {
                if let Some(variable) = self.variable_map.get(var_name) {
                    lhs_expr.add_mul(*coef, variable.1);
                }
            });
            lhs_expr += bound.lhs.1;
            let mut rhs_expr = Expression::from_other_affine(0.0);
            bound.rhs.0.iter().for_each(|(coef, var_name)| {
                if let Some(variable) = self.variable_map.get(var_name) {
                    rhs_expr.add_mul(*coef, variable.1);
                }
            });
            rhs_expr += bound.rhs.1;

            let constraint = match bound.bound_type {
                BoundType::Equality => lhs_expr.eq(rhs_expr),
                BoundType::LessThanOrEqual => lhs_expr.leq(rhs_expr),
                BoundType::GreaterThanOrEqual => lhs_expr.geq(rhs_expr),
            };
            self.bounds.push(constraint);
        }
        self
    }
}
