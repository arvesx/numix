#[cfg(test)]
mod nurbs_tests {
    use snt::interpolate::parametric_curve::Nurbs;
    use snt::interpolate::parametric_curve::{nurbs_curve, nurbs_curve_advanced};

    /// Endpoint Testing: This test verifies that the curve starts and ends at the correct control points.
    #[test]
    fn test_nurbs_endpoints() -> Result<(), Box<dyn std::error::Error>> {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let nurbs = nurbs_curve(&ctrl_pts, 3).unwrap();

        let start = nurbs.eval(0.0).unwrap();
        let end = nurbs.eval(1.0).unwrap();

        assert_eq!(start, ctrl_pts[0]);
        assert_eq!(end, *ctrl_pts.last().unwrap());

        Ok(())
    }

    ///Weight Invariance: This test verifies that for a NURBS curve with all weights set to 1,
    /// the curve should be identical to a B-spline with the same control points and knot vector.
    ///
    #[test]
    fn test_nurbs_weight_invariance() -> Result<(), Box<dyn std::error::Error>> {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];

        // Create a NURBS curve with all weights set to 1
        let nurbs =
            nurbs_curve_advanced(&ctrl_pts, 3, Some(&vec![1.0; ctrl_pts.len()]), None).unwrap();

        // Create a B-spline with the same control points
        let bspline = nurbs_curve(&ctrl_pts, 3).unwrap();

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let nurbs_point = nurbs.eval(t).unwrap();
            let bspline_point = bspline.eval(t).unwrap();

            assert_eq!(nurbs_point, bspline_point);
        }

        Ok(())
    }

    /// Knot Vector Invariance: This test verifies that if you set a uniform knot vector manually,
    /// the curve should be identical to a NURBS curve with the default uniform knot vector.
    #[test]
    fn test_nurbs_knot_vector_invariance() -> Result<(), Box<dyn std::error::Error>> {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let uniform_knot_vector = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];

        let nurbs_default = nurbs_curve(&ctrl_pts, 3).unwrap();
        let nurbs_manual =
            nurbs_curve_advanced(&ctrl_pts, 3, None, Some(&uniform_knot_vector)).unwrap();

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let point_default = nurbs_default.eval(t).unwrap();
            let point_manual = nurbs_manual.eval(t).unwrap();

            assert_eq!(point_default, point_manual);
        }

        Ok(())
    }

    /// Control Point Invariance: This test verifies that if you set a control point to a
    /// new value and then set it back to the original value, the curve should remain
    /// unchanged.
    #[test]
    fn test_nurbs_control_point_invariance() -> Result<(), Box<dyn std::error::Error>> {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let mut nurbs = nurbs_curve(&ctrl_pts, 3).unwrap();

        let original_point = ctrl_pts[1];
        let new_point = (1.5, 1.5);

        // Modify a control point
        nurbs.set_control_point_at(1, new_point).unwrap();

        // Set it back to the original value
        nurbs.set_control_point_at(1, original_point).unwrap();

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let point_original = nurbs_curve(&ctrl_pts, 3).unwrap().eval(t).unwrap();
            let point_modified = nurbs.eval(t).unwrap();

            assert_eq!(point_original, point_modified);
        }

        Ok(())
    }

    // Weight Invariance: If all weights are equal, the NURBS curve should be identical to the corresponding B-spline curve.
    #[test]
    fn test_weight_invariance() {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let nurbs =
            nurbs_curve_advanced(&ctrl_pts, 3, Some(&vec![1.0; ctrl_pts.len()]), None).unwrap();
        let bspline_curve = nurbs_curve(&ctrl_pts, 3).unwrap();

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let nurbs_point = nurbs.eval(t).unwrap();
            let bspline_point = bspline_curve.eval(t).unwrap();
            assert!((nurbs_point.0 - bspline_point.0).abs() < 1e-6);
            assert!((nurbs_point.1 - bspline_point.1).abs() < 1e-6);
        }
    }

    // Parameter Range Invariance: The curve should not produce any points for parameter values outside the range defined by the knot vector.
    #[test]
    fn test_parameter_range_invariance() {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let nurbs_curve = nurbs_curve_advanced(&ctrl_pts, 3, None, None).unwrap();

        // Assuming your knot vector starts at 0 and ends at 1
        assert!(nurbs_curve.eval(-0.1).is_none());
        assert!(nurbs_curve.eval(1.1).is_none());
    }

    /// Affine Invariance: The curve should be invariant under affine transformations.
    /// This means that transforming the control points and then constructing the
    /// curve should be the same as constructing the curve and then transforming it.
    #[test]
    fn test_affine_invariance() {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let nurbs_curve = nurbs_curve_advanced(&ctrl_pts, 3, None, None).unwrap();

        // Apply an affine transformation to the control points
        let transformed_ctrl_pts: Vec<(f64, f64)> =
            ctrl_pts.iter().map(|&(x, y)| (x + 1.0, y * 2.0)).collect();

        let transformed_nurbs_curve =
            nurbs_curve_advanced(&transformed_ctrl_pts, 3, None, None).unwrap();

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let point = nurbs_curve.eval(t).unwrap();
            let transformed_point = transformed_nurbs_curve.eval(t).unwrap();

            // Check if the transformation holds
            assert!((transformed_point.0 - (point.0 + 1.0)).abs() < 1e-6);
            assert!((transformed_point.1 - (point.1 * 2.0)).abs() < 1e-6);
        }
    }

    /// Homogeneous Coordinate Invariance: Multiplying all weights by a constant should not change the curve.
    #[test]
    fn test_homogeneous_coordinate_invariance() {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let weights = vec![1.0, 2.0, 1.0, 1.5];
        let nurbs = nurbs_curve_advanced(&ctrl_pts, 3, Some(&weights), None).unwrap();

        let scaled_weights: Vec<f64> = weights.iter().map(|&w| w * 2.0).collect();
        let scaled_nurbs = nurbs_curve_advanced(&ctrl_pts, 3, Some(&scaled_weights), None).unwrap();

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let point = nurbs.eval(t).unwrap();
            let scaled_point = scaled_nurbs.eval(t).unwrap();

            assert!((scaled_point.0 - point.0).abs() < 1e-6);
            assert!((scaled_point.1 - point.1).abs() < 1e-6);
        }
    }

    /// Reversal Invariance: Reversing the control points and knot vector should produce a curve that is the "reverse" of the original curve.
    #[test]
    fn test_reversal_invariance() {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let nurbs = nurbs_curve_advanced(&ctrl_pts, 3, None, None).unwrap();

        let reversed_ctrl_pts: Vec<(f64, f64)> = ctrl_pts.iter().rev().cloned().collect();
        let reversed_nurbs = nurbs_curve_advanced(&reversed_ctrl_pts, 3, None, None).unwrap();

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let point = nurbs.eval(t).unwrap();
            let reversed_point = reversed_nurbs.eval(1.0 - t).unwrap();

            assert!((reversed_point.0 - point.0).abs() < 1e-6);
            assert!((reversed_point.1 - point.1).abs() < 1e-6);
        }
    }

    #[test]
    fn test_normalization_invariance() {
        let ctrl_pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)];
        let weights = vec![1.0, 2.0, 1.0, 1.5];
        let nurbs = nurbs_curve_advanced(&ctrl_pts, 3, Some(&weights), None).unwrap();

        let sum_weights: f64 = weights.iter().sum();
        let normalized_weights: Vec<f64> = weights.iter().map(|&w| w / sum_weights).collect();
        let normalized_nurbs =
            nurbs_curve_advanced(&ctrl_pts, 3, Some(&normalized_weights), None).unwrap();

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let point = nurbs.eval(t).unwrap();
            let normalized_point = normalized_nurbs.eval(t).unwrap();

            assert!((normalized_point.0 - point.0).abs() < 1e-6);
            assert!((normalized_point.1 - point.1).abs() < 1e-6);
        }
    }

    #[test]
    pub fn test_weight_modification() -> Result<(), Box<dyn std::error::Error>> {
        let ctrl_pts = vec![
            (0.0, 0.0),
            (5.0, 2.0),
            (2.0, 1.0),
            (7.0, 3.0),
            (4.0, 0.0),
            (1.0, 2.0),
            (6.0, 1.0),
            (3.0, 3.0),
            (8.0, 0.0),
            (9.0, 2.0),
        ];

        let mut bspline = nurbs_curve(&ctrl_pts, 3).unwrap();

        // Plot before weight modification
        plot_nurbs(&bspline, "before_weight_modification.png")?;

        // Modify weight
        bspline.set_weight_at(7, 3.4).unwrap();

        // Plot after weight modification
        plot_nurbs(&bspline, "after_weight_modification.png")?;

        Ok(())
    }

    #[test]
    pub fn test_knot_modification() -> Result<(), Box<dyn std::error::Error>> {
        let ctrl_pts = vec![
            (0.0, 0.0), // P0
            (1.0, 2.0), // P1
            (2.0, 3.0), // P2
            (4.0, 3.0), // P3
            (6.0, 1.0), // P4
            (7.0, 2.0), // P5
            (8.0, 1.0), // P6
            (9.0, 0.0), // P7
        ];
        let mut bspline = nurbs_curve(&ctrl_pts, 3).unwrap();
        bspline
            .get_knot_vector()
            .iter()
            .for_each(|&x| print!("{} ", x));
        // Plot before knot modification
        plot_nurbs(&bspline, "before_knot_modification.png")?;

        bspline.set_knot_at(4, 0.01)?;
        bspline.set_knot_at(5, 0.6)?;

        // Plot after knot modification
        plot_nurbs(&bspline, "after_knot_modification.png")?;

        Ok(())
    }
    fn plot_nurbs(bspline: &Nurbs, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        use plotters::prelude::*;

        let root_drawing_area = BitMapBackend::new(filename, (2056, 1400)).into_drawing_area();
        root_drawing_area.fill(&WHITE)?;

        let mut chart =
            ChartBuilder::on(&root_drawing_area).build_cartesian_2d(-1f64..10f64, -1f64..5f64)?;

        let curve_points: Vec<(f64, f64)> = (0..=1000)
            .map(|i| i as f64 / 1000.0)
            .filter_map(|t| bspline.eval(t))
            .collect();

        chart.draw_series(curve_points.windows(2).map(|ps| {
            PathElement::new(ps.to_vec(), Into::<ShapeStyle>::into(RED).stroke_width(13))
        }))?;

        chart.draw_series(
            bspline
                .ctrl_pts
                .iter()
                .map(|&point| Circle::new(point, 5, BLUE)),
        )?;

        Ok(())
    }
}
