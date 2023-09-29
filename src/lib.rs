pub mod optimize {
    pub mod root_finding;
}

pub mod interpolate {
    mod cubic_spline;
    mod error_utils;
    pub mod interpolator;
    mod linear_spline;
    pub mod parametric_interpolator;
}
pub mod integrate {
    pub mod integrator;
}
