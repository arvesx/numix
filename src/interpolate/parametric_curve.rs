use super::cubic_bezier::CubicBezierCurve;

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

/// Represents a Non-Uniform Rational B-Spline (NURBS) curve.
///
/// A NURBS curve is defined by its control points, weights, degree, and a knot vector.
///
/// # Fields
///
/// * `ctrl_pts: Vec<(f64, f64)>` - The control points that define the shape of the curve.
///   Each control point is a tuple `(x, y)` where `x` and `y` are the coordinates of the point.
///
/// * `weights: Vec<f64>` - The weights associated with each control point.
///   The weights influence how much the curve is attracted to each control point.
///   Must have the same length as `ctrl_pts`.
///
/// * `p: usize` - The degree of the curve. Determines the smoothness and complexity of the curve.
///   Must be less than the number of control points.
///
/// * `knot_vector: Vec<f64>` - The knot vector that defines the parameterization of the curve.
///   Must be a non-decreasing sequence and its length must be `ctrl_pts.len() + p + 1`.
///
pub struct Nurbs {
    pub ctrl_pts: Vec<(f64, f64)>,
    pub(super) weights: Vec<f64>,
    pub(super) p: usize,
    pub(super) knot_vector: Vec<f64>,
}

/// Creates a NURBS curve with the given control points and degree.
///
/// This function serves as a simplified API for creating a NURBS curve. It only requires the control points and the degree of the curve, using default values for the weights and knot vector.
///
/// # Parameters
///
/// * `ctrl_pts: &[(f64, f64)]` - An array of control points for the curve.
/// * `p: usize` - The degree of the curve.
///
/// # Returns
///
/// * `Result<Nurbs, super::error_utils::ParametricCurveError>` - Returns a `Nurbs` object if the curve is successfully created. Otherwise, returns an `Err` with a `ParametricCurveError` detailing the reason for the failure.
///
/// # Examples
///
/// ```
/// use snt::interpolate::parametric_curve::nurbs_curve;
/// let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
/// let p = 2;
/// let curve = nurbs_curve(&ctrl_pts, p);
/// ```
pub fn nurbs_curve(
    ctrl_pts: &[(f64, f64)],
    p: usize,
) -> Result<Nurbs, super::error_utils::ParametricCurveError> {
    Nurbs::new(ctrl_pts, p, None, None)
}

/// Creates a NURBS curve with advanced options.
///
/// This function provides a more advanced API for creating a NURBS curve, allowing for custom weights and knot vectors in addition to the control points and degree.
///
/// # Parameters
///
/// * `ctrl_pts: &[(f64, f64)]` - An array of control points for the curve.
/// * `p: usize` - The degree of the curve.
/// * `weights: Option<&[f64]>` - An optional array of weights for the control points.
/// * `knot_vector: Option<&[f64]>` - An optional knot vector for the curve.
///
/// # Returns
///
/// * `Result<Nurbs, super::error_utils::ParametricCurveError>` - Returns a `Nurbs` object if the curve is successfully created. Otherwise, returns an `Err` with a `ParametricCurveError` detailing the reason for the failure.
///
/// # Examples
///
/// ```
/// use snt::interpolate::parametric_curve::nurbs_curve_advanced;
/// let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
/// let p = 2;
/// let weights = vec![1.0, 0.5, 1.0];
/// let knot_vector = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
/// let curve = nurbs_curve_advanced(&ctrl_pts, p, Some(&weights), Some(&knot_vector));
/// ```
pub fn nurbs_curve_advanced(
    ctrl_pts: &[(f64, f64)],
    p: usize,
    weights: Option<&[f64]>,
    knot_vector: Option<&[f64]>,
) -> Result<Nurbs, super::error_utils::ParametricCurveError> {
    Nurbs::new(ctrl_pts, p, weights, knot_vector)
}
