use super::error_utils::ParametricCurveError;

/// Constructs a uniform knot vector for a B-spline curve.
///
/// # Arguments
///
/// * `n: usize` - The number of control points.
/// * `p: usize` - The degree of the B-spline.
///
/// # Returns
///
/// * `Result<Vec<f64>, ParametricCurveError>` - A `Result` type containing either:
///     - `Ok(Vec<f64>)` with the constructed uniform knot vector, or
///     - `Err(ParametricCurveError::InvalidBSplineConfiguration)` if the number of control points `n` is less than or equal to the degree `p`.
///
/// # Errors
///
/// Returns `ParametricCurveError::InvalidBSplineConfiguration` if `n <= p`, indicating an invalid B-spline configuration.
///
fn construct_uniform_knot_vector(n: usize, p: usize) -> Result<Vec<f64>, ParametricCurveError> {
    if n <= p {
        return Err(ParametricCurveError::InvalidBSplineConfiguration);
    }

    let knot_vector_size = n + p + 1;
    let mut knot_vector = vec![0.0; knot_vector_size];
    let segment_size = 1.0 / ((n - p) as f64);
    let mut j = 1.0;

    for item in knot_vector
        .iter_mut()
        .take(knot_vector_size - (p + 1))
        .skip(p + 1)
    {
        *item = j * segment_size;
        j += 1.0;
    }

    for item in knot_vector
        .iter_mut()
        .take(knot_vector_size)
        .skip(knot_vector_size - (p + 1))
    {
        *item = 1.0;
    }

    Ok(knot_vector)
}

/// Finds the knot span index for a given parameter value `t` within a given knot vector.
///
/// # Arguments
///
/// * `t: f64` - The parameter value, expected to be in the range [0.0, 1.0].
/// * `knot_vector: &[f64]` - A reference to the knot vector array.
///
/// # Returns
///
/// * `Option<usize>` - An `Option` containing either:
///     - `Some(usize)` with the index of the knot span where `t` lies, or
///     - `None` if `t` is outside the range [0.0, 1.0] or if the knot vector is invalid.
///
/// # Special Cases
///
/// * For `t = 0.0`, returns the index of the first non-zero knot.
/// * For `t = 1.0`, returns the index of the last non-one knot.
///
fn find_knot_span(t: f64, knot_vector: &[f64]) -> Option<usize> {
    if !(0.0..=1.0).contains(&t) {
        return None;
    }

    // Special case for t = 0.0
    if t <= *knot_vector.first()? {
        return Some(knot_vector.iter().position(|&x| x != knot_vector[0])? - 1);
    }

    // Special case for t = 1.0
    if t >= *knot_vector.last()? {
        return knot_vector
            .iter()
            .rposition(|&x| x != *knot_vector.last().unwrap());
    }

    // Binary search for all other cases
    knot_vector
        .windows(2)
        .position(|window| t >= window[0] && t < window[1])
}

pub struct BSpline {
    ctrl_pts: Vec<(f64, f64)>,
    p: usize,
    knot_vector: Vec<f64>,
}

impl BSpline {
    /// Constructs a new instance of a B-spline curve.
    ///
    /// # Parameters
    ///
    /// - `ctrl_pts: &[(f64, f64)]`: A slice of control points, each represented as a tuple `(x, y)`.
    ///   These points define the shape of the B-spline curve.
    /// - `p: usize`: The degree of the B-spline curve. This determines the smoothness and complexity of the curve.
    ///
    /// # Returns
    ///
    /// - `Result<Self, ParametricCurveError>`: Returns an `Ok` variant containing the newly created B-spline curve
    ///   if successful. If the function encounters an error, it returns an `Err` variant containing a `ParametricCurveError`.
    ///
    /// # Errors
    ///
    /// - `ParametricCurveError::InvalidBSplineConfiguration`: This error is returned if the number of control points
    ///   is insufficient for the given degree `p`.
    ///
    /// # Notes
    ///
    /// - The function internally clones the provided control points and constructs a uniform knot vector based
    ///   on the number of control points and the degree `p`.
    /// - The knot vector is stored as part of the B-spline object for future evaluations of the curve.
    pub(crate) fn new(ctrl_pts: &[(f64, f64)], p: usize) -> Result<Self, ParametricCurveError> {
        let ctrl_pts_copy = ctrl_pts.to_vec();
        let knot_vector = construct_uniform_knot_vector(ctrl_pts_copy.len(), p)?;
        Ok(Self {
            ctrl_pts: ctrl_pts_copy,
            p,
            knot_vector,
        })
    }

    /// Evaluates the B-spline curve at a given parameter `t` using De Boor's Algorithm.
    ///
    /// # Parameters
    /// - `t`: The parameter at which to evaluate the curve. Must be in the range [0, 1].
    ///
    /// # Returns
    /// - `Option<(f64, f64)>`: The point `(x, y)` on the curve corresponding to the parameter `t`.
    ///   Returns `None` if `t` is out of range or if the knot span cannot be found.
    ///
    /// # Algorithm
    /// 1. Finds the knot span index `i` for the given parameter `t` using the knot vector.
    /// 2. Extracts the local control points that influence the curve at `t`.
    /// 3. Applies De Boor's Algorithm to iteratively compute the point on the curve.
    ///
    /// # Notes
    /// - This function assumes that the knot vector and control points are already set.
    /// - The function uses a uniform knot vector.
    /// - The function is designed to work with non-rational B-splines.
    ///
    /// # Errors
    /// - Returns `None` if `t` is not in the range [0, 1] or if the knot span cannot be found.
    pub fn eval(&self, t: f64) -> Option<(f64, f64)> {
        let i = find_knot_span(t, &self.knot_vector)?;

        let mut local_ctrl_pts = self.ctrl_pts[i - self.p..=i].to_vec();

        // De Boor's Algorithm
        for r in 1..=self.p {
            for j in (r..=self.p).rev() {
                let alpha = (t - self.knot_vector[i + j - self.p])
                    / (self.knot_vector[i + j + 1 - r] - self.knot_vector[i + j - self.p]);
                local_ctrl_pts[j].0 =
                    (1.0 - alpha) * local_ctrl_pts[j - 1].0 + alpha * local_ctrl_pts[j].0;
                local_ctrl_pts[j].1 =
                    (1.0 - alpha) * local_ctrl_pts[j - 1].1 + alpha * local_ctrl_pts[j].1;
            }
        }

        Some(local_ctrl_pts[self.p])
    }
}
