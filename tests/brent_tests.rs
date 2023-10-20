#[cfg(test)]
mod brent_tests {
    use numix::optimize::root_finding::Brent;

    #[test]
    fn test1() {
        let root1 = Brent::initialize(|x| x * x - 4.0, -3.0, -1.0)
            .tol(1e-5)
            .run();

        let root2 = Brent::initialize(|x| x * x - 4.0, 1.0, 3.0).tol(1e-5).run();

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
    fn test2() {
        let root1: Result<
            numix::optimize::root_finding::AlgoMetrics,
            numix::optimize::root_finding::RootFindingError,
        > = Brent::initialize(|x| x * x - 4.0, -3.0, -1.0)
            .tol(1e-10)
            .iter(10000)
            .run();
        let root2 = Brent::initialize(|x| x * x - 4.0, 1.0, 3.0)
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
    fn test3() {
        let root1 = Brent::initialize(|x| x.sin(), -1.0, 1.0).tol(1e-5).run();
        let root2 = Brent::initialize(|x| x.sin(), 2.0, 4.0).tol(1e-5).run();

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
    fn test4() {
        let root1 = Brent::initialize(|x| x.sin(), -1.0, 1.0)
            .tol(1e-10)
            .iter(10000)
            .run();
        let root2 = Brent::initialize(|x| x.sin(), 2.0, 4.0)
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
    fn test5() {
        let root = Brent::initialize(|x| x.exp() - 2.0, 0.0, 1.0)
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
    fn test6() {
        let root = Brent::initialize(|x| x.exp() - 2.0, 0.0, 1.0)
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
    fn test7() {
        let root = Brent::initialize(|x| x.powi(3) - 2.0 * x.powi(2) + x.sin() - 1.0, 0.0, 3.0)
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
    fn test8() {
        let root = Brent::initialize(|x| (-x).exp() + x.powi(2) - x.cos() - 1.0, -2.0, 0.0)
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
    fn test9() {
        let root = Brent::initialize(|x| x, -1.0, 1.0).tol(1e-5).run();

        match root {
            Ok(root) => {
                assert!(root.est_x.abs() < 1e-5);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test10() {
        let root = Brent::initialize(|x| (x + 1.0).ln() - x.powi(2) + 2.0 * x, -0.5, 2.0)
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
