#[cfg(test)]
mod bisection_tests {
    use numix::optimize::root_finding::Ridders;

    #[test]
    fn test_quadratic() {
        let root1 = Ridders::initialize(|x| x * x - 4.0, -3.0, -1.0)
            .tol(1e-5)
            .run();

        let root2 = Ridders::initialize(|x| x * x - 4.0, 1.0, 3.0)
            .tol(1e-5)
            .run();

        match root1 {
            Ok(root1) => {
                println!("{}", root1);
                assert!((root1.est_x + 2.0).abs() < 1e-5)
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
        match root2 {
            Ok(root2) => {
                println!("{}", root2);
                assert!((root2.est_x - 2.0).abs() < 1e-5)
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        };
    }
    #[test]
    fn test_quadratic_high_precision() {
        let root1 = Ridders::initialize(|x| x * x - 4.0, -3.0, -1.0)
            .tol(1e-10)
            .iter(10000)
            .run();
        let root2 = Ridders::initialize(|x| x * x - 4.0, 1.0, 3.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root1 {
            Ok(root1) => {
                assert!((root1.est_x + 2.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
        match root2 {
            Ok(root2) => {
                assert!((root2.est_x - 2.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        };
    }
    #[test]
    fn test_sine() {
        let root1 = Ridders::initialize(|x| x.sin(), -1.0, 1.0).tol(1e-5).run();
        let root2 = Ridders::initialize(|x| x.sin(), 2.0, 4.0).tol(1e-5).run();

        match root1 {
            Ok(root1) => {
                assert!(root1.est_x.abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
        match root2 {
            Ok(root2) => {
                assert!((root2.est_x - std::f64::consts::PI).abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        };
    }
    #[test]
    fn test_sine_high_precision() {
        let root1 = Ridders::initialize(|x| x.sin(), -1.0, 1.0)
            .tol(1e-10)
            .iter(10000)
            .run();
        let root2 = Ridders::initialize(|x| x.sin(), 2.0, 4.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root1 {
            Ok(root1) => {
                assert!(root1.est_x.abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
        match root2 {
            Ok(root2) => {
                assert!((root2.est_x - std::f64::consts::PI).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        };
    }
    #[test]
    fn test_exponential() {
        let root = Ridders::initialize(|x| x.exp() - 2.0, 0.0, 1.0)
            .tol(1e-5)
            .run();
        match root {
            Ok(root) => {
                assert!((root.est_x - 2.0f64.ln()).abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test_exponential_high_precision() {
        let root = Ridders::initialize(|x| x.exp() - 2.0, 0.0, 1.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root {
            Ok(root) => {
                assert!((root.est_x - f64::ln(2.0)).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test_complex_high_precision() {
        let root = Ridders::initialize(|x| x.powi(3) - 2.0 * x.powi(2) + x.sin() - 1.0, 0.0, 3.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root {
            Ok(root) => {
                // Validate the root within the tolerance
                let f_root = root.est_x.powi(3) - 2.0 * root.est_x.powi(2) + root.est_x.sin() - 1.0;
                assert!(f_root.abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }

    #[test]
    fn test_super_complex_high_precision() {
        let root = Ridders::initialize(|x| (-x).exp() + x.powi(2) - x.cos() - 1.0, -2.0, 0.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root {
            Ok(root) => {
                // Validate the root within the tolerance
                let f_root = (-root.est_x).exp() + root.est_x.powi(2) - root.est_x.cos() - 1.0;
                assert!(f_root.abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }

    #[test]
    fn test_linear() {
        let root = Ridders::initialize(|x| x, -1.0, 1.0).tol(1e-5).run();

        match root {
            Ok(root) => {
                assert!(root.est_x.abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test_log_poly_high_precision() {
        let root = Ridders::initialize(|x| (x + 1.0).ln() - x.powi(2) + 2.0 * x, -0.1, 2.0)
            .tol(1e-10)
            .iter(10000)
            .run();

        match root {
            Ok(root) => {
                // Validate the root within the tolerance
                let f_root = (root.est_x + 1.0).ln() - root.est_x.powi(2) + 2.0 * root.est_x;
                assert!(f_root.abs() < 1e-8);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
}
