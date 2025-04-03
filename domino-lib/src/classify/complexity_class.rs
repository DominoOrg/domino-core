//! This module defines the `ComplexityClass` struct, which represents a class of computational complexity.
//!
//! The class is validated to ensure it falls within the defined range `[1, NUMBER_OF_CLASSES]`.
//! It also implements `Display` for formatted output and `Into<f32>` for numeric conversion.

use super::NUMBER_OF_CLASSES;
use crate::DominoError;
use std::cmp::Ordering;
use std::fmt::Display;

/// Represents a complexity class as an integer value.
///
/// This struct enforces that the class value is within a valid range and provides
/// conversion methods for display and numerical operations.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<f32> for ComplexityClass {
    /// Converts the `ComplexityClass` into a floating-point number.
    fn into(self) -> f32 {
        self.0 as f32
    }
}

// Implement PartialEq to allow comparison with f32
impl PartialEq<f32> for ComplexityClass {
    fn eq(&self, other: &f32) -> bool {
        (self.0 as f32) == *other
    }
}

// Implement PartialOrd to allow ordering comparisons with f32
impl PartialOrd<f32> for ComplexityClass {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        (self.0 as f32).partial_cmp(other)
    }
}

// Implement PartialEq to allow comparison with usize
impl PartialEq<usize> for ComplexityClass {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

// Implement PartialOrd to allow ordering comparisons with usize
impl PartialOrd<usize> for ComplexityClass {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}
