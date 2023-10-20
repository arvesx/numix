#[cfg(test)]
mod linear_spline_tests {

    use numix::interpolate::interpolator::linear_spline;
    use numix::interpolate::interpolator::linear_spline_in_place;

    #[test]
    fn test_linear_spline() {
        // Define the data points for the spline
        let data = vec![(1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0)];

        // Create the spline function
        let f = linear_spline(&data).unwrap();

        // Test points within the range
        assert_eq!(f.eval(1.5).unwrap(), 2.5); // Midway between 1.0 and 4.0
        assert_eq!(f.eval(2.5).unwrap(), 6.5); // Midway between 4.0 and 9.0

        // Test points exactly at the data points
        assert_eq!(f.eval(1.0).unwrap(), 1.0); // First point
        assert_eq!(f.eval(4.0).unwrap(), 16.0); // Last point

        // Test points outside the range
        assert!(f.eval(0.5).is_none()); // Below the range
        assert!(f.eval(4.5).is_none()); // Above the range
    }

    #[test]
    fn test_linear_spline_flat_and_descending() {
        // Define the data points for a flat line
        let flat_data = vec![(1.0, 2.0), (2.0, 2.0), (3.0, 2.0)];

        // Create the spline function for the flat line
        let f_flat = linear_spline(&flat_data).unwrap();

        // Test points within the range
        assert_eq!(f_flat.eval(1.5).unwrap(), 2.0);
        assert_eq!(f_flat.eval(2.5).unwrap(), 2.0);

        // Define the data points for a descending line
        let desc_data = vec![(1.0, 3.0), (2.0, 2.0), (3.0, 1.0)];

        // Create the spline function for the descending line
        let f_desc = linear_spline(&desc_data).unwrap();

        // Test points within the range
        assert_eq!(f_desc.eval(1.5).unwrap(), 2.5);
        assert_eq!(f_desc.eval(2.5).unwrap(), 1.5);

        // Test points exactly at the data points
        assert_eq!(f_desc.eval(1.0).unwrap(), 3.0);
        assert_eq!(f_desc.eval(3.0).unwrap(), 1.0);
    }

    #[test]
    fn test_linear_spline_in_place() {
        // Define the data points for the spline
        let mut data = vec![(1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0)];

        // Create the spline function
        let f = linear_spline_in_place(&mut data).unwrap();

        // Test points within the range
        assert_eq!(f.eval(1.5).unwrap(), 2.5); // Midway between 1.0 and 4.0
        assert_eq!(f.eval(2.5).unwrap(), 6.5); // Midway between 4.0 and 9.0

        // Test points exactly at the data points
        assert_eq!(f.eval(1.0).unwrap(), 1.0); // First point
        assert_eq!(f.eval(4.0).unwrap(), 16.0); // Last point

        // Test points outside the range
        assert!(f.eval(0.5).is_none()); // Below the range
        assert!(f.eval(4.5).is_none()); // Above the range
    }

    #[test]
    fn test_linear_spline_flat_and_descending_in_place() {
        // Define the data points for a flat line
        let mut flat_data = vec![(1.0, 2.0), (2.0, 2.0), (3.0, 2.0)];

        // Create the spline function for the flat line
        let f_flat = linear_spline_in_place(&mut flat_data).unwrap();

        // Test points within the range
        assert_eq!(f_flat.eval(1.5).unwrap(), 2.0);
        assert_eq!(f_flat.eval(2.5).unwrap(), 2.0);

        // Define the data points for a descending line
        let mut desc_data = vec![(1.0, 3.0), (2.0, 2.0), (3.0, 1.0)];

        // Create the spline function for the descending line
        let f_desc = linear_spline_in_place(&mut desc_data).unwrap();

        // Test points within the range
        assert_eq!(f_desc.eval(1.5).unwrap(), 2.5);
        assert_eq!(f_desc.eval(2.5).unwrap(), 1.5);

        // Test points exactly at the data points
        assert_eq!(f_desc.eval(1.0).unwrap(), 3.0);
        assert_eq!(f_desc.eval(3.0).unwrap(), 1.0);
    }
    #[test]
    fn test_linear_spline_with_artificial_data() {
        let mut data: Vec<(f64, f64)> = Vec::new();

        // Generate artificial data
        for i in 0..500 {
            let x = i as f64;
            let y = i as f64;
            data.push((x, y));
        }

        // Create the spline function
        let f = linear_spline(&data).unwrap();

        // Test a few random points within the range
        let test_points = vec![
            10.5, 200.3, 150.2, 145.6, 32.7, 65.8, 300.7, 400.1, 10.5, 200.3, 150.2, 145.6, 32.7,
            65.8, 300.7, 400.1, 10.5, 200.3, 150.2, 145.6, 32.7, 65.8, 300.7, 400.1,
        ];
        for &x in &test_points {
            let _y = f.eval(x);
            // assert!(y.is_some(), "Failed to evaluate at x = {}", x);
            // println!("f({}) = {:?}", x, y.unwrap());
        }
    }
    #[test]
    fn test_linear_spline_with_artificial_data_in_place() {
        let mut data: Vec<(f64, f64)> = Vec::new();

        // Generate artificial data
        for i in 0..500 {
            let x = i as f64;
            let y = i as f64;
            data.push((x, y));
        }

        // Create the spline function
        let f = linear_spline_in_place(&mut data).unwrap();

        // Test a few random points within the range
        let test_points = vec![
            10.5, 200.3, 150.2, 145.6, 32.7, 65.8, 300.7, 400.1, 10.5, 200.3, 150.2, 145.6, 32.7,
            65.8, 300.7, 400.1, 10.5, 200.3, 150.2, 145.6, 32.7, 65.8, 300.7, 400.1,
        ];
        for &x in &test_points {
            let _y = f.eval(x);
            // assert!(y.is_some(), "Failed to evaluate at x = {}", x);
            // println!("f({}) = {:?}", x, y.unwrap());
        }
    }
}
