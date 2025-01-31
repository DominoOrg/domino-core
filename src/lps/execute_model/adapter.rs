use std::collections::HashMap;

use good_lp::{default_solver, variable, Constraint, Expression, ProblemVariables, ResolutionError, SolverModel, Variable, VariableDefinition};

use super::{parser::{BoundType, ObjectiveType}, translator::ResultTranslator, ModelParser};


pub struct Adapter {
  vars: ProblemVariables,
  pub variable_map: HashMap<String, (VariableDefinition, Variable)>,
  var_names: HashMap<Variable, String>,
  objective_direction: ObjectiveType,
  objective: Expression,
  bounds: Vec<Constraint>
}

impl Adapter {

  pub fn new() -> Adapter {
    Adapter { vars: ProblemVariables::new(), variable_map: HashMap::new(), var_names: HashMap::new(), objective_direction: ObjectiveType::Minimize, objective: Expression::from_other_affine(0.0), bounds: Vec::new() }
  }

  pub fn adapt(mut self, parser: &ModelParser) -> Result<ResultTranslator, ResolutionError> {
    self.adapt_variables(parser);
    self.adapt_objective(parser);
    self.adapt_bounds(parser);
    let mut problem = if self.objective_direction == ObjectiveType::Minimize {
      self.vars.minimise(self.objective.clone())
    } else {
      self.vars.maximise(self.objective.clone())
    }.using(default_solver);
    self.bounds.into_iter().for_each(|constraint| {
      problem.add_constraint(constraint);
    });
    problem.solve().map(|solution| ResultTranslator::new(self.objective, self.var_names, self.variable_map.clone(), solution))
  }

  fn adapt_variables(&mut self, parser: &ModelParser) {
    for variable_name in parser.binary_variables.clone() {
      let def_variable = variable().binary().name(variable_name.clone());
      let variable = self.vars.add(def_variable.clone());
      self.variable_map.insert(variable_name.clone(), (def_variable, variable));
      self.var_names.insert(variable, variable_name);
    }
  }

  fn adapt_objective(&mut self, parser: &ModelParser) {
    let objective = parser.objective.clone();
    self.objective_direction = objective.objective_type;
    self.objective = Expression::from_other_affine(0.0);
    for term in objective.terms.0 {
      if let Some(variable) = self.variable_map.get(&term.1) {
        self.objective.add_mul(term.0, Expression::from_other_affine(variable.1));
      }
    }
    self.objective += objective.terms.1;
  }

  fn adapt_bounds(&mut self, parser: &ModelParser) {
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
          BoundType::Equality => {
            lhs_expr.eq(rhs_expr)
          },
          BoundType::LessThanOrEqual => {
            lhs_expr.leq(rhs_expr)
          },
          BoundType::GreaterThanOrEqual => {
            lhs_expr.geq(rhs_expr)
          },
      };
      self.bounds.push(constraint);
    }
  }
}