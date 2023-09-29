use ndarray::{Array1, Array2};
use ndarray_linalg::SolveTridiagonal;
use std::cmp::Ordering;

use super::error_utils::InterpolationError;

pub struct CubicSpline {
    segments: Vec<(f64, f64, f64, f64)>,
    coefficients: Vec<(f64, f64, f64, f64)>,
}

impl CubicSpline {
    /// # Natural Cubic Spline Constructor
    ///
    /// Constructs a natural cubic spline based on the given set of points.
    /// The function sorts the points by their x-values and then calculates the cubic coefficients
    /// for each segment between adjacent points. It also checks for duplicate x-values and throws an error if found.
    ///
    /// # Arguments
    ///
    /// * `pts` - A mutable reference to a vector of tuples, where each tuple represents a point `(x, y)`.
    ///
    /// # Returns
    ///
    /// * `Result<Self, InterpolationError>` - Returns a `CubicSpline` object containing the segments and coefficients
    ///   for interpolation, or an `InterpolationError` if duplicate x-values are found.
    ///
    /// # Errors
    ///
    /// * `InterpolationError::DuplicateXValuesError` - Thrown when two points have the same x-value.
    ///
    pub(crate) fn new(pts: &mut Vec<(f64, f64)>) -> Result<Self, InterpolationError> {
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut segments = Vec::new();
        let mut coefficients = Vec::new();
        let mut h = Vec::new();
        let n = pts.len();

        // Initialize m vector with zeros, including endpoints
        let mut m = Array1::<f64>::zeros(n);

        for i in 0..(n - 1) {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[i + 1];

            if x1 == x2 {
                return Err(InterpolationError::DuplicateXValuesError);
            }

            h.push(x2 - x1);
            segments.push((x1, y1, x2, y2));
        }

        let mut b = Array1::<f64>::zeros(n - 2);
        for i in 0..(n - 2) {
            let (_, y0) = pts[i];
            let (_, y1) = pts[i + 1];
            let (_, y2) = pts[i + 2];
            b[i] = 6.0 * ((y2 - y1) / h[i + 1] - (y1 - y0) / h[i]);
        }

        let mut a = Array2::<f64>::zeros((n - 2, n - 2));
        for i in 0..(n - 2) {
            a[[i, i]] = 2.0 * (h[i] + h[i + 1]);
        }

        for i in 0..(n - 3) {
            a[[i, i + 1]] = h[i + 1];
            a[[i + 1, i]] = h[i + 1];
        }

        // Solve the tridiagonal system
        let m_inner = a.solve_tridiagonal_into(b).unwrap();

        // Fill in the inner m values
        for i in 1..(n - 1) {
            m[i] = m_inner[i - 1];
        }

        for i in 0..(n - 1) {
            let (_x_i, y_i) = pts[i];
            let (_, y_ipp) = pts[i + 1];
            let h_i = h[i];
            let m_i = m[i];
            let m_ipp = m[i + 1];

            let a_i = y_i;
            let b_i = (y_ipp - y_i) / h_i - h_i * (m_ipp + 2.0 * m_i) / 6.0;
            let c_i = m_i / 2.0;
            let d_i = (m_ipp - m_i) / (6.0 * h_i);

            coefficients.push((a_i, b_i, c_i, d_i));
        }

        Ok(CubicSpline {
            segments,
            coefficients,
        })
    }

    /// # Evaluate Cubic Spline at a Point
    ///
    /// Evaluates the cubic spline at a given point `x`. The function uses binary search to find the
    /// appropriate segment that contains `x`, and then evaluates the cubic polynomial for that segment.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate where the cubic spline will be evaluated.
    ///
    /// # Returns
    ///
    /// * `Option<f64>` - Returns the y-coordinate corresponding to `x` if `x` is within the domain of the spline.
    ///   Returns `None` if `x` is outside the domain.
    ///
    pub fn eval(&self, x: f64) -> Option<f64> {
        let idx = self.segments.binary_search_by(|&(x1, _, x2, _)| {
            if x < x1 {
                Ordering::Greater
            } else if x > x2 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        match idx {
            Ok(i) => {
                let (x1, _, _, _) = self.segments[i];
                let (a_i, b_i, c_i, d_i) = self.coefficients[i];
                let dx = x - x1;
                Some(a_i + b_i * dx + c_i * dx.powi(2) + d_i * dx.powi(3))
            }
            Err(_) => None,
        }
    }
}
