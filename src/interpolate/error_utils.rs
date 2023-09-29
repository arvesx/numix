pub enum InterpolationError {
    DuplicateXValuesError,
}

impl std::fmt::Display for InterpolationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InterpolationError::DuplicateXValuesError => write!(
                f,
                "Duplicate x-values found. Interpolation requires unique x-values."
            ),
        }
    }
}

use std::fmt;

impl fmt::Debug for InterpolationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpolationError::DuplicateXValuesError => write!(
                f,
                "Duplicate x-values found. Interpolation requires unique x-values."
            ),
        }
    }
}
