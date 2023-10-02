#[cfg(test)]
mod b_spline_tests {
    use snt::interpolate::parametric_curve::cubic_bspline;

    #[test]
    pub fn bspline_test() -> Result<(), Box<dyn std::error::Error>> {
        use plotters::prelude::*;
        let root_drawing_area =
            BitMapBackend::new("roller_coaster.png", (2056, 1400)).into_drawing_area();
        root_drawing_area.fill(&WHITE)?;

        let mut chart =
            ChartBuilder::on(&root_drawing_area).build_cartesian_2d(-1f64..15f64, -1f64..8f64)?;

        let ctrl_pts = vec![
            (0.0, 0.0),
            (3.0, 2.0),
            (1.0, 4.0),
            (4.0, 6.0),
            (2.0, 5.0),
            (5.0, 4.0),
            (6.0, 1.0),
        ];

        let bspline = cubic_bspline(&ctrl_pts).unwrap();

        let curve_points: Vec<(f64, f64)> = (0..=1000)
            .map(|i| i as f64 / 1000.0)
            .filter_map(|t| bspline.eval(t))
            .collect();

        chart.draw_series(curve_points.windows(2).map(|ps| {
            PathElement::new(ps.to_vec(), Into::<ShapeStyle>::into(RED).stroke_width(13))
        }))?;

        chart.draw_series(ctrl_pts.iter().map(|&point| Circle::new(point, 5, BLUE)))?;

        Ok(())
    }
}
