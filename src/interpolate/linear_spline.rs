use std::cmp::Ordering;
pub struct LinearSpline {
    segments: Vec<(f64, f64, f64, f64)>, // (x1, y1, x2, y2) for each segment
}

impl LinearSpline {
    pub(crate) fn new(pts: &mut Vec<(f64, f64)>) -> Self {
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut segments = Vec::new();
        for i in 0..pts.len() - 1 {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[i + 1];
            segments.push((x1, y1, x2, y2));
        }

        LinearSpline { segments }
    }

    /// Evaluates the linear spline at a given point `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The point at which to evaluate the spline.
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
    /// * The function does not panic but returns `None` if `x` is outside the domain.
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
