//! This module defines the `ComplexityClass` struct, which represents a class of computational complexity.
//!
//! The class is validated to ensure it falls within the defined range `[1, NUMBER_OF_CLASSES]`.
//! It also implements `Display` for formatted output and `Into<f32>` for numeric conversion.

use std::fmt::Display;
use crate::DominoError;
use super::NUMBER_OF_CLASSES;

/// Represents a complexity class as an integer value.
///
/// This struct enforces that the class value is within a valid range and provides
/// conversion methods for display and numerical operations.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ComplexityClass(pub usize);

impl ComplexityClass {
    /// Creates a new `ComplexityClass` instance if the provided value is within a valid range.
    ///
    /// # Arguments
    ///
    /// * `class` - The complexity class value.
    ///
    /// # Returns
    ///
    /// * `Ok(ComplexityClass)` - If the provided class is in the range `[1, NUMBER_OF_CLASSES]`.
    /// * `Err(DominoError::InvalidClass)` - If the class is out of bounds.
    ///
    /// # Example
    ///
    /// ```rust
    /// let valid_class = ComplexityClass::new(3);
    /// assert!(valid_class.is_ok());
    ///
    /// let invalid_class = ComplexityClass::new(0);
    /// assert!(invalid_class.is_err());
    /// ```
    pub fn new(class: usize) -> Result<ComplexityClass, DominoError> {
        if class == 0 || class > NUMBER_OF_CLASSES {
            let err_msg = format!(
                "The complexity class provided is not valid: {}.\nIt should be in the range [1, {}]",
                class, NUMBER_OF_CLASSES
            );
            return Err(DominoError::InvalidClass(err_msg));
        }

        Ok(Self(class))
    }
}

impl Display for ComplexityClass {
    /// Formats the `ComplexityClass` as a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let class = ComplexityClass(5);
    /// assert_eq!(format!("{}", class), "5");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0).as_str())
    }
}

impl Into<f32> for ComplexityClass {
    /// Converts the `ComplexityClass` into a floating-point number.
    ///
    /// # Example
    ///
    /// ```rust
    /// let class = ComplexityClass(4);
    /// let class_as_float: f32 = class.into();
    /// assert_eq!(class_as_float, 4.0);
    /// ```
    fn into(self) -> f32 {
        self.0 as f32
    }
}
