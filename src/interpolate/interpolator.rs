use super::linear_spline::LinearSpline;

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
///
/// # Example
///
/// ```
/// let points = vec![(1.0, 2.0), (2.0, 3.0), (3.0, 4.0)];
/// let spline = linear_spline(&points);
/// ```
pub fn linear_spline(pts: &[(f64, f64)]) -> LinearSpline {
    let mut pts_clone = pts.to_owned();
    LinearSpline::new(&mut pts_clone)
}

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
///
/// # Example
///
/// ```
/// let mut points = vec![(3.0, 4.0), (1.0, 2.0), (2.0, 3.0)];
/// let spline = linear_spline_in_place(&mut points);
/// ```
pub fn linear_spline_in_place(pts: &mut Vec<(f64, f64)>) -> LinearSpline {
    LinearSpline::new(pts)
}
