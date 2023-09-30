use super::cubic_bezier::CubicBezierCurve;

pub fn cubic_bezier(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> CubicBezierCurve {
    CubicBezierCurve::new(p0, p1, p2, p3)
}
