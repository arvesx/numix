#[cfg(test)]
mod cubic_spline_tests {
    use snt::{interpolate::interpolator::cubic_spline, optimize::root_finding::precision_equals};

    #[test]
    fn test_cubic_spline() {
        // Define the data points for the spline
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0), (4.0, 0.0)];

        // Create the spline function
        let f = cubic_spline(&data).unwrap();

        // Test points within the range
        assert!(precision_equals(f.eval(0.01).unwrap(), 0.0171, 1e-4, 0.0));
        assert!(precision_equals(f.eval(0.1).unwrap(), 0.1707, 1e-4, 0.0));
        assert!(precision_equals(f.eval(0.5).unwrap(), 0.7678, 1e-4, 0.0));
        assert!(precision_equals(f.eval(1.3).unwrap(), 0.7210, 1e-4, 0.0));
        assert!(precision_equals(f.eval(1.5).unwrap(), 0.4464, 1e-4, 0.0));
        assert!(precision_equals(f.eval(2.5).unwrap(), 0.4464, 1e-4, 0.0));
        assert!(precision_equals(f.eval(3.5).unwrap(), 0.7678, 1e-4, 0.0));

        // // Test points exactly at the data points
        assert!(precision_equals(f.eval(0.0).unwrap(), 0.0, 1e-4, 0.0));
        assert!(precision_equals(f.eval(4.0).unwrap(), 0.0, 1e-4, 0.0));

        // Test points outside the range
        assert!(f.eval(-0.5).is_none()); // Below the range
        assert!(f.eval(4.5).is_none()); // Above the range
    }
    #[test]
    fn test_cubic_spline2() {
        // Define the data points for the spline
        let data = vec![(-2.5, 3.0), (1.2, 1.0), (2.0, 4.0), (2.8, -1.0), (4.0, 5.0)];

        // Create the spline function
        let f = cubic_spline(&data).unwrap();

        // Test points within the range
        assert!(precision_equals(f.eval(-2.0).unwrap(), 1.1770, 1e-4, 0.0));
        assert!(precision_equals(f.eval(-2.2).unwrap(), 1.8951, 1e-4, 0.0));
        assert!(precision_equals(f.eval(0.0).unwrap(), -2.6490, 1e-4, 0.0));
        assert!(precision_equals(f.eval(0.4).unwrap(), -2.1055, 1e-4, 0.0));
        assert!(precision_equals(f.eval(1.2).unwrap(), 1.0, 1e-4, 0.0));
        assert!(precision_equals(f.eval(1.4).unwrap(), 2.2087, 1e-4, 0.0));
        assert!(precision_equals(f.eval(2.0).unwrap(), 4.0, 1e-4, 0.0));
        assert!(precision_equals(f.eval(3.15).unwrap(), -1.1119, 1e-4, 0.0));

        // // Test points exactly at the data points
        assert!(precision_equals(f.eval(-2.5).unwrap(), 3.0, 1e-4, 0.0));
        assert!(precision_equals(f.eval(4.0).unwrap(), 5.0, 1e-4, 0.0));

        // Test points outside the range
        assert!(f.eval(-3.5).is_none()); // Below the range
        assert!(f.eval(4.5).is_none()); // Above the range
    }
}
