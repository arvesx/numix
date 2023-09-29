use super::{cubic_spline::CubicSpline, linear_spline::LinearSpline};

/// # Linear Spline Interpolation
///
/// A linear spline is a piecewise linear function that interpolates a set of given points.
/// It consists of segments connecting each pair of consecutive points in such a way that
/// the function is continuous. Unlike higher-order splines, a linear spline does not provide
/// smoothness at the junctions between segments.
/// Creates a `LinearSpline` based on the given points.
///
/// This function clones the input vector and then sorts it, ensuring that the original data remains unchanged (data integrity).
/// Sorting is essential for the spline interpolation algorithm to work correctly.
/// Ideal for scenarios where you need to retain the original data for other operations.
///
/// # Arguments
///
/// * `pts` - A reference to a vector of tuples, where each tuple represents a point `(x, y)`.
///
/// # Returns
///
/// * `LinearSpline` - A `LinearSpline` object that can be used for interpolation.
pub fn linear_spline(
    pts: &[(f64, f64)],
) -> Result<LinearSpline, super::error_utils::InterpolationError> {
    let mut pts_clone = pts.to_owned();
    LinearSpline::new(&mut pts_clone)
}
/// # Linear Spline Interpolation
///
/// A linear spline is a piecewise linear function that interpolates a set of given points.
/// It consists of segments connecting each pair of consecutive points in such a way that
/// the function is continuous. Unlike higher-order splines, a linear spline does not provide
/// smoothness at the junctions between segments.
/// Creates a `LinearSpline` based on the given points, modifying the input vector in-place.
///
/// This function sorts the input vector in-place, making it more memory-efficient but altering the original data.
/// Sorting is essential for the spline interpolation algorithm to work correctly.
/// Ideal for scenarios where memory efficiency is a priority and you don't need the original data.
///
/// # Arguments
///
/// * `pts` - A mutable reference to a vector of tuples, where each tuple represents a point `(x, y)`.
///
/// # Returns
///
/// * `LinearSpline` - A `LinearSpline` object that can be used for interpolation.
pub fn linear_spline_in_place(
    pts: &mut Vec<(f64, f64)>,
) -> Result<LinearSpline, super::error_utils::InterpolationError> {
    LinearSpline::new(pts)
}

/// # Natural Cubic Spline Interpolation
///
/// A cubic spline is a piecewise cubic polynomial function that interpolates a set of given points.
/// Unlike linear splines, cubic splines provide smoothness at the junctions between segments, ensuring
/// that both the first and second derivatives are continuous.
/// Creates a `CubicSpline` based on the given points.
///
/// This function clones the input vector and then sorts it, ensuring that the original data remains unchanged (data integrity).
/// Sorting is essential for the spline interpolation algorithm to work correctly.
/// Ideal for scenarios where you need to retain the original data for other operations.
///
/// # Arguments
///
/// * `pts` - A reference to a vector of tuples, where each tuple represents a point `(x, y)`.
///
/// # Returns
///
/// * `CubicSpline` - A `CubicSpline` object that can be used for interpolation.
///
pub fn cubic_spline(
    pts: &[(f64, f64)],
) -> Result<CubicSpline, super::error_utils::InterpolationError> {
    let mut pts_clone = pts.to_owned();
    CubicSpline::new(&mut pts_clone)
}
