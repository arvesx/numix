pub struct CubicBezierCurve {
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
}

impl CubicBezierCurve {
    pub(crate) fn new(
        p0: (f64, f64),
        p1: (f64, f64),
        p2: (f64, f64),
        p3: (f64, f64),
    ) -> CubicBezierCurve {
        CubicBezierCurve { p0, p1, p2, p3 }
    }

    pub fn eval(&self, t: f64) -> Option<(f64, f64)> {
        if !(0.0..=1.0).contains(&t) {
            return None;
        }

        let x = (1.0 - t).powi(3) * self.p0.0
            + 3.0 * t * (1.0 - t).powi(2) * self.p1.0
            + 3.0 * t.powi(2) * (1.0 - t) * self.p2.0
            + t.powi(3) * self.p3.0;

        let y = (1.0 - t).powi(3) * self.p0.1
            + 3.0 * t * (1.0 - t).powi(2) * self.p1.1
            + 3.0 * t.powi(2) * (1.0 - t) * self.p2.1
            + t.powi(3) * self.p3.1;

        Some((x, y))
    }
}
