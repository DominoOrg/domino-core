#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg(feature = "lpmodels")]
pub enum ObjectiveType {
    #[default]
    Minimize,
    Maximize,
}

#[cfg(feature = "lpmodels")]
#[derive(Debug)]
pub enum BoundType {
    Equality,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[cfg(feature = "lpmodels")]
#[derive(Debug)]
pub struct Bound {
    _name: String,
    pub lhs: (Vec<(f64, String)>, f64), // Coefficient and variable pairs for the left-hand side
    pub rhs: (Vec<(f64, String)>, f64), // Right-hand side constant
    pub bound_type: BoundType,
}

#[cfg(feature = "lpmodels")]
#[derive(Debug, Default, Clone)]
pub struct Objective {
    pub objective_type: ObjectiveType,
    pub terms: (Vec<(f64, String)>, f64), // Coefficient and variable pairs
}

#[cfg(feature = "lpmodels")]
#[derive(Debug)]
pub struct ModelParser {
    pub objective: Objective,
    pub bounds: Vec<Bound>,
    pub binary_variables: Vec<String>,
}

#[cfg(feature = "lpmodels")]
impl ModelParser {
    pub fn new() -> Self {
        ModelParser {
            objective: Objective::default(),
            bounds: Vec::new(),
            binary_variables: Vec::new(),
        }
    }

    // Parses the text input into the ModelParser structure
    pub fn parse(&mut self, text: &str) {
        let mut lines = text.lines();
        let mut mode = "";
        let mut current_objective_terms = Vec::new();
        let mut constant = 0.0;

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            } else if line.starts_with("Minimize") || line.starts_with("Maximize") {
                mode = "objective";
                let objective_type = if line.starts_with("Minimize") {
                    ObjectiveType::Minimize
                } else {
                    ObjectiveType::Maximize
                };
                self.objective = Objective {
                    objective_type,
                    terms: (Vec::new(), 0.0),
                };
            } else if line.starts_with("Subject To") {
                mode = "constraints";
                // Store any objective terms collected so far
                self.objective.terms = (current_objective_terms.clone(), constant);
            } else if line.starts_with("Binary") {
                mode = "binary";
            } else if line.starts_with("End") {
                break;
            } else {
                match mode {
                    "objective" => {
                        // Collects terms for the objective function across multiple lines
                        let obj_line = line
                            .split_whitespace()
                            .skip(1)
                            .collect::<Vec<&str>>()
                            .join(" ");
                        let expr = self.parse_expression(&obj_line);
                        current_objective_terms.extend(expr.0);
                        constant = expr.1;
                    }
                    "constraints" => {
                        // Parses each constraint line and adds it to the constraints vector
                        self.bounds.push(self.parse_constraint(line));
                    }
                    "binary" => {
                        // Adds each binary variable name to the binary_variables vector
                        self.binary_variables.push(line.to_string());
                    }
                    _ => {}
                }
            }
        }

        // Ensure any remaining terms for the objective are stored if they were parsed
        self.objective.terms = (current_objective_terms, constant);
    }

    fn parse_constraint(&self, line: &str) -> Bound {
        let name_split: Vec<&str> = line.split(':').collect();
        let _name = name_split[0].trim().to_string();
        let rest = name_split[1].trim();

        let (lhs, bound_type, rhs) = if rest.contains("<=") {
            let sides: Vec<&str> = rest.split("<=").collect();
            (
                self.parse_expression(sides[0]),
                BoundType::LessThanOrEqual,
                self.parse_expression(sides[1]),
            )
        } else if rest.contains(">=") {
            let sides: Vec<&str> = rest.split(">=").collect();
            (
                self.parse_expression(sides[0]),
                BoundType::GreaterThanOrEqual,
                self.parse_expression(sides[1]),
            )
        } else if rest.contains("=") {
            let sides: Vec<&str> = rest.split('=').collect();
            (
                self.parse_expression(sides[0]),
                BoundType::Equality,
                self.parse_expression(sides[1]),
            )
        } else {
            panic!("Unrecognized constraint format");
        };

        Bound {
            _name,
            lhs,
            rhs,
            bound_type,
        }
    }

    fn parse_expression(&self, expr: &str) -> (Vec<(f64, String)>, f64) {
        let mut terms = Vec::new();
        let mut constant = 0.0;
        let mut current_sign = 1.0;
        let mut buffer = String::new();

        for (i, c) in expr.chars().enumerate() {
            match c {
                ' ' => {
                    if !buffer.is_empty() {
                        self.parse_and_add_term(
                            &mut buffer,
                            &mut terms,
                            &mut constant,
                            current_sign,
                        );
                        buffer.clear();
                    }
                }
                '+' => {
                    if !buffer.is_empty() {
                        self.parse_and_add_term(
                            &mut buffer,
                            &mut terms,
                            &mut constant,
                            current_sign,
                        );
                        buffer.clear();
                    }
                    current_sign = 1.0;
                }
                '-' => {
                    if !buffer.is_empty() {
                        self.parse_and_add_term(
                            &mut buffer,
                            &mut terms,
                            &mut constant,
                            current_sign,
                        );
                        buffer.clear();
                    }
                    current_sign = -1.0;
                }
                _ => buffer.push(c), // Accumulate characters into buffer
            }

            // If it's the last character, parse the remaining buffer
            if i == expr.len() - 1 && !buffer.is_empty() {
                self.parse_and_add_term(&mut buffer, &mut terms, &mut constant, current_sign);
            }
        }

        (terms, constant)
    }

    // Helper function to parse buffer content as either a constant or variable term
    fn parse_and_add_term(
        &self,
        buffer: &mut String,
        terms: &mut Vec<(f64, String)>,
        constant: &mut f64,
        sign: f64,
    ) {
        // Check if buffer is empty (nothing to parse)
        if buffer.is_empty() {
            return;
        }

        // Case 1: If the buffer is purely numeric, treat it as a constant
        if buffer.chars().all(|c| c.is_digit(10) || c == '.') {
            *constant += sign * buffer.parse::<f64>().unwrap();

        // Case 2: If buffer starts with a digit and contains letters, it's a variable with a coefficient
        } else if buffer.chars().next().unwrap().is_digit(10) {
            let mut coefficient_str = String::new();

            // Extract leading numeric characters as the coefficient until we reach a letter
            while let Some(c) = buffer.chars().next() {
                if c.is_alphabetic() {
                    break;
                }
                coefficient_str.push(buffer.remove(0)); // Add to coefficient and remove from buffer
            }

            // Parse coefficient and add term
            let coefficient = sign * coefficient_str.parse::<f64>().unwrap();
            terms.push((coefficient, buffer.clone())); // Remainder of buffer is the variable name

        // Case 3: If buffer starts with a letter, treat it as a variable with implicit coefficient 1
        } else {
            terms.push((sign, buffer.clone()));
        }

        // Clear buffer after parsing
        buffer.clear();
    }
}
