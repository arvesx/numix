#[cfg(test)]
mod cubic_bezier_tests {
    use snt::{
        interpolate::parametric_curve::cubic_bezier, optimize::root_finding::precision_equals,
    };

    #[test]
    fn test_cubic_bezier() {
        let c = cubic_bezier((1.2, 1.6), (1.9, 5.4), (6.7, 3.8), (7.4, 6.6));

        // Test points within the range
        let (x, y) = c.eval(0.5).unwrap();
        assert!(precision_equals(x, 4.3, 1e-4, 0.0));
        assert!(precision_equals(y, 4.475, 1e-4, 0.0));

        let (x, y) = c.eval(0.2).unwrap();
        assert!(precision_equals(x, 2.0464, 1e-4, 0.0));
        assert!(precision_equals(y, 3.3104, 1e-4, 0.0));

        let (x, y) = c.eval(0.7).unwrap();
        assert!(precision_equals(x, 5.8844, 1e-4, 0.0));
        assert!(precision_equals(y, 5.0034, 1e-4, 0.0));

        // Test points exactly at the data points

        let (x, y) = c.eval(0.0).unwrap();
        assert!(precision_equals(x, 1.2, 1e-4, 0.0));
        assert!(precision_equals(y, 1.6, 1e-4, 0.0));

        let (x, y) = c.eval(1.0).unwrap();
        assert!(precision_equals(x, 7.4, 1e-4, 0.0));
        assert!(precision_equals(y, 6.6, 1e-4, 0.0));

        // Test points outside the range
        assert!(c.eval(-0.1).is_none()); // Below the range
        assert!(c.eval(1.2).is_none()); // Above the range
    }
}
