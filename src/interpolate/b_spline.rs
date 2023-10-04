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
        return Err(ParametricCurveError::BSplineConfiguration);
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
/// * `t: f64` - The parameter value.
/// * `knot_vector: &[f64]` - A reference to the knot vector array.
///
/// # Returns
///
/// * `Option<usize>` - An `Option` containing either:
///     - `Some(usize)` with the index of the knot span where `t` lies, or
///     - `None` if `t` is outside the range of the knot vector or if the knot vector is invalid.
///
/// # Special Cases
///
/// * For `t` equal to the first knot, returns the index of the first non-zero knot.
/// * For `t` equal to the last knot, returns the index of the last non-one knot.
///
fn find_knot_span(t: f64, knot_vector: &[f64]) -> Option<usize> {
    let t_min = *knot_vector.first()?;
    let t_max = *knot_vector.last()?;

    if t < t_min || t > t_max {
        return None;
    }

    // Special case for t = t_min
    if t == t_min {
        return Some(knot_vector.iter().position(|&x| x != t_min)? - 1);
    }

    // Special case for t = t_max
    if t == t_max {
        return knot_vector.iter().rposition(|&x| x != t_max);
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

pub struct Nurbs {
    ctrl_pts: Vec<(f64, f64)>,
    weights: Vec<f64>,
    p: usize,
    knot_vector: Vec<f64>,
}

impl Nurbs {
    pub(crate) fn new(
        ctrl_pts: &[(f64, f64)],
        p: usize,
        weights: Option<&[f64]>,
        knot_vector: Option<&[f64]>,
    ) -> Result<Self, ParametricCurveError> {
        // Invalid Degree: The degree p should be less than the number of control points n. If p>=n, that's an error.
        if p >= ctrl_pts.len() {
            return Err(ParametricCurveError::NURBSConfiguration(
                "The number of control points n must be greater than the spline degree p by at least 1. Please provide a valid configuration.".to_string(),
            ));
        }

        if let Some(knot_vector) = knot_vector {
            // Mismatched Lengths: If the length of the knot vector doesn't match the expected size based on the number of control points and the degree of the curve, throw an error.
            if knot_vector.len() != ctrl_pts.len() + p + 1 {
                return Err(ParametricCurveError::NURBSConfiguration(
                    "knot_vector.len() should be equal to ctrl_pts.len() + p + 1".to_string(),
                ));
            }
            // Non-Ascending Knot Vector: The values in the knot vector should be non-decreasing. If you find a value that's smaller than the previous one, throw an error.
            if !knot_vector.windows(2).all(|w| w[0] <= w[1]) {
                return Err(ParametricCurveError::NURBSConfiguration(
                    "Knot vector is not in non-descending order.".to_string(),
                ));
            }

            // Invalid Multiplicity at Start or End: The first and last knots should appear p+1 times for a clamped B-spline. If they don't, that's an error.
            let first_knot = knot_vector[0];
            let last_knot = *knot_vector.last().unwrap();
            let first_multiplicity = knot_vector.iter().take_while(|&&x| x == first_knot).count();
            let last_multiplicity = knot_vector
                .iter()
                .rev()
                .take_while(|&&x| x == last_knot)
                .count();

            if first_multiplicity < p + 1 || last_multiplicity < p + 1 {
                return Err(ParametricCurveError::NURBSConfiguration(
                    "Invalid Multiplicity at Start or End: The first and last knots should appear p+1 times for a clamped B-spline.".to_string(),
                ));
            }

            // Internal Knot Multiplicity Exceeds Degree: For internal knots (knots that are not at the start or end of the knot vector), the multiplicity (number of times the knot value appears) should not exceed the degree p of the curve.
            let mut prev_knot = knot_vector[0];
            let mut count = 1;

            for &current_knot in &knot_vector[1..] {
                if current_knot == prev_knot {
                    count += 1;
                } else {
                    if count > p {
                        return Err(ParametricCurveError::NURBSConfiguration(
                            "Internal knot multiplicity exceeds degree.".to_string(),
                        ));
                    }
                    count = 1;
                    prev_knot = current_knot;
                }
            }

            // Check the last knot's multiplicity
            if count > p {
                return Err(ParametricCurveError::NURBSConfiguration(
                    "Internal knot multiplicity exceeds degree.".to_string(),
                ));
            }
        }

        if let Some(weights) = weights {
            if weights.len() != ctrl_pts.len() {
                return Err(ParametricCurveError::NURBSConfiguration(
                    "In a NURBS curve, each control point must have an associated weight."
                        .to_string(),
                ));
            }

            // Non-Positive Weights: All weights should be positive. If any weight is zero or negative, throw an error.
            if weights.iter().any(|&x| x < 0.0) {
                return Err(ParametricCurveError::NURBSConfiguration(
                    "Negative weight has been encountered. Be sure to have non negative values in weights vector.".to_string(),
                ));
            }
        }

        let ctrl_pts_copy = ctrl_pts.to_vec();
        let weights_copy = weights.unwrap_or(&vec![1.0; ctrl_pts_copy.len()]).to_vec();
        let knot_vector_copy = knot_vector
            .unwrap_or(&construct_uniform_knot_vector(ctrl_pts_copy.len(), p)?)
            .to_vec();

        Ok(Self {
            ctrl_pts: ctrl_pts_copy,
            weights: weights_copy,
            p,
            knot_vector: knot_vector_copy,
        })
    }

    pub fn eval(&self, t: f64) -> Option<(f64, f64)> {
        let i = find_knot_span(t, &self.knot_vector)?;

        // Initialize local control points and weights
        let local_ctrl_pts = self.ctrl_pts[i - self.p..=i].to_vec();
        let mut local_weights = self.weights[i - self.p..=i].to_vec(); // Make it mutable

        // Initialize weighted control points for De Boor's Algorithm
        let mut weighted_ctrl_pts: Vec<(f64, f64)> = local_ctrl_pts
            .iter()
            .zip(local_weights.iter())
            .map(|(&(x, y), &w)| (x * w, y * w))
            .collect();

        // De Boor's Algorithm
        for r in 1..=self.p {
            for j in (r..=self.p).rev() {
                let alpha = (t - self.knot_vector[i + j - self.p])
                    / (self.knot_vector[i + j + 1 - r] - self.knot_vector[i + j - self.p]);

                // Update weighted control points
                weighted_ctrl_pts[j].0 =
                    (1.0 - alpha) * weighted_ctrl_pts[j - 1].0 + alpha * weighted_ctrl_pts[j].0;
                weighted_ctrl_pts[j].1 =
                    (1.0 - alpha) * weighted_ctrl_pts[j - 1].1 + alpha * weighted_ctrl_pts[j].1;

                // Update weights
                local_weights[j] = (1.0 - alpha) * local_weights[j - 1] + alpha * local_weights[j];
            }
        }

        // Convert from homogeneous to Cartesian coordinates
        let final_weight = local_weights[self.p];
        let final_point = (
            weighted_ctrl_pts[self.p].0 / final_weight,
            weighted_ctrl_pts[self.p].1 / final_weight,
        );

        Some(final_point)
    }

    pub fn set_knot_at(&mut self, index: usize, value: f64) -> Result<(), ParametricCurveError> {
        // Check for out-of-bounds index
        if index >= self.knot_vector.len() {
            return Err(ParametricCurveError::KnotValue);
        }

        // Check for clamping condition
        let p = self.p;
        if index <= p || index >= self.knot_vector.len() - p - 1 {
            return Err(ParametricCurveError::KnotValue);
        }

        // Check for non-decreasing condition
        if value < self.knot_vector[index - 1] || value > self.knot_vector[index + 1] {
            return Err(ParametricCurveError::KnotValue);
        }

        self.knot_vector[index] = value;
        Ok(())
    }

    pub fn set_weight_at(&mut self, index: usize, value: f64) -> Result<(), &'static str> {
        if index < self.weights.len() {
            self.weights[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
}
