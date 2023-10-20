use super::error_utils::InterpolationError;
use std::cmp::Ordering;
pub struct LinearSpline {
    segments: Vec<(f64, f64, f64, f64)>, // (x1, y1, x2, y2) for each segment
}

impl LinearSpline {
    /// # Constructor for `LinearSpline`
    ///
    /// Initializes a `LinearSpline` object by sorting the input points and creating segments.
    ///
    /// # Arguments
    ///
    /// * `pts` - A mutable reference to a vector of tuples `(x, y)` representing the data points.
    ///
    /// # Returns
    ///
    /// * `Result<Self, InterpolationError>` - Returns an `Ok` variant containing the `LinearSpline` object if successful, or an `Err` variant containing an `InterpolationError` if duplicate x-values are found.
    ///
    /// # Errors
    ///
    /// * `InterpolationError::DuplicateXValuesError` - Thrown when two points have the same x-value.
    pub(crate) fn new(pts: &mut Vec<(f64, f64)>) -> Result<Self, InterpolationError> {
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut segments = Vec::new();
        for i in 0..pts.len() - 1 {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[i + 1];

            if x1 == x2 {
                return Err(InterpolationError::DuplicateXValuesError);
            }

            segments.push((x1, y1, x2, y2));
        }

        Ok(LinearSpline { segments })
    }

    /// Evaluates the linear spline at a given point `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The point at which to evaluate the spline. Evaluates the cubic spline at a given
    /// point `x`. The function uses binary search to find the appropriate segment that contains `x`.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - The value of the spline at `x` if `x` is within the domain of the spline.
    /// * `None` - If `x` is outside the domain of the spline.
    ///
    ///
    ///
    /// # Notes
    ///
    /// * The function assumes that the segments are sorted by their `x` values, which of course happens when creating the spline. Caller does NOT have to sort the data.
    /// * Uses binary search for efficient lookup, making the operation O(log n).
    ///
    /// # Panics
    ///
    /// * The function does NOT panic but returns `None` if `x` is outside the domain.
    ///
    pub fn eval(&self, x: f64) -> Option<f64> {
        // Assuming self.segments is sorted by x1
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
                let (x1, y1, x2, y2) = self.segments[i];
                Some(y1 + (y2 - y1) / (x2 - x1) * (x - x1))
            }
            Err(_) => None,
        }
    }
}
