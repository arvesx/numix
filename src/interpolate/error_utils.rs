pub enum InterpolationError {
    DuplicateXValuesError,
}

pub enum ParametricCurveError {
    InvalidBSplineConfiguration,
}

impl std::fmt::Display for ParametricCurveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBSplineConfiguration => write!(f, "The number of control points n must be greater than the spline degree p by at least 1. Please provide a valid configuration.")
        }
    }
}
impl fmt::Debug for ParametricCurveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBSplineConfiguration => write!(f, "The number of control points n must be greater than the spline degree p by at least 1. Please provide a valid configuration."),
        }
    }
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
