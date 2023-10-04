use super::{
    b_spline::{BSpline, Nurbs},
    cubic_bezier::CubicBezierCurve,
};

/// Constructs a new cubic Bezier curve.
///
/// # Parameters
///
/// - `p0, p1, p2, p3: (f64, f64)`: The four control points that define the cubic Bezier curve.
///
/// # Returns
///
/// - `CubicBezierCurve`: A new cubic Bezier curve object.
pub fn cubic_bezier(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> CubicBezierCurve {
    CubicBezierCurve::new(p0, p1, p2, p3)
}

/// Constructs a new cubic B-spline curve.
///
/// # Parameters
///
/// - `ctrl_pts: &[(f64, f64)]`: A slice of control points that define the B-spline curve.
///
/// # Returns
///
/// - `Result<BSpline, super::error_utils::ParametricCurveError>`: A new cubic B-spline curve object or an error.
pub fn cubic_bspline(
    ctrl_pts: &[(f64, f64)],
) -> Result<BSpline, super::error_utils::ParametricCurveError> {
    BSpline::new(ctrl_pts, 3)
}

/// Constructs a new B-spline curve of degree `p`.
///
/// # Parameters
///
/// - `ctrl_pts: &[(f64, f64)]`: A slice of control points that define the B-spline curve.
/// - `p: usize`: The degree of the B-spline curve.
///
/// # Returns
///
/// - `Result<BSpline, super::error_utils::ParametricCurveError>`: A new B-spline curve object or an error.
pub fn bspline(
    ctrl_pts: &[(f64, f64)],
    p: usize,
) -> Result<BSpline, super::error_utils::ParametricCurveError> {
    BSpline::new(ctrl_pts, p)
}

pub fn nurbs(
    ctrl_pts: &[(f64, f64)],
    p: usize,
) -> Result<Nurbs, super::error_utils::ParametricCurveError> {
    Nurbs::new(ctrl_pts, p, None, None)
}

pub fn nurbs_advanced(
    ctrl_pts: &[(f64, f64)],
    p: usize,
    weights: Option<&[f64]>,
    knot_vector: Option<&[f64]>,
) -> Result<Nurbs, super::error_utils::ParametricCurveError> {
    Nurbs::new(ctrl_pts, p, weights, knot_vector)
}
