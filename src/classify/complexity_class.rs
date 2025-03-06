use std::fmt::Display;

use crate::DominoError;

use super::NUMBER_OF_CLASSES;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ComplexityClass(pub usize);

impl ComplexityClass {
    pub fn new(class: usize) -> Result<ComplexityClass, DominoError> {
        if class == 0 || class > NUMBER_OF_CLASSES {
            let err_msg = format!("The complexity class provided is not valid: {}.\nIt should be in the range [1, {}]", class, NUMBER_OF_CLASSES);
            return Err(DominoError::InvalidClass(err_msg));
        }

        Ok(Self(class))
    }
}

impl Display for ComplexityClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0).as_str())
    }
}

impl Into<f32> for ComplexityClass {
    fn into(self) -> f32 {
        self.0 as f32
    }
}
