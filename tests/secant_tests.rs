#[cfg(test)]
pub mod newton_tests {
    use snt::optimize::root_finding::Newton;

    #[test]
    fn test1() {
        // Test case 1: Root near 0 for sin(x)
        let root1 = Newton::initialize(|x| x.sin(), 1.0).tol(1e-10).run();

        // Test case 2: Root near PI for sin(x)
        let root2 = Newton::initialize(|x| x.sin(), std::f64::consts::PI)
            .tol(1e-10)
            .run();

        // Validate root1
        match root1 {
            Ok(root1) => {
                assert!(root1.abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }

        // Validate root2
        match root2 {
            Ok(root2) => {
                assert!((root2 - std::f64::consts::PI).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test2() {
        // Test case 1: Root at x = 1 for the cubic equation
        let root1 = Newton::initialize(|x| x.powi(3) - 6.0 * x.powi(2) + 11.0 * x - 6.0, 0.5)
            .tol(1e-10)
            .run();

        // Test case 2: Root at x = 3 for the cubic equation
        let root2 = Newton::initialize(|x| x.powi(3) - 6.0 * x.powi(2) + 11.0 * x - 6.0, 3.5)
            .tol(1e-10)
            .run();

        // Validate root1
        match root1 {
            Ok(root1) => {
                assert!((root1 - 1.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }

        // Validate root2
        match root2 {
            Ok(root2) => {
                assert!((root2 - 3.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }

    #[test]
    fn test3() {
        // Constants
        let ln_2: f64 = std::f64::consts::LN_2; // Natural log of 2

        // Test case 1: Root at x = ln(2) for the exponential equation
        let root = Newton::initialize(|x| x.exp() - 2.0, 0.5).tol(1e-10).run();

        // Validate root
        match root {
            Ok(root) => {
                assert!((root - ln_2).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test4() {
        // Test case 1: Root at x = 2 for f(x) = x^2 - 4
        let root1 = Newton::initialize(|x| x.powi(2) - 4.0, 1.0)
            .tol(1e-10)
            .run();

        // Test case 2: Root at x = -2 for f(x) = x^2 - 4
        let root2 = Newton::initialize(|x| x.powi(2) - 4.0, -1.0)
            .tol(1e-10)
            .run();

        // Validate root1
        match root1 {
            Ok(root1) => {
                assert!((root1 - 2.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }

        // Validate root2
        match root2 {
            Ok(root2) => {
                assert!((root2 + 2.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test5() {
        // Test case: Root at x = 4 for f(x) = x^3 - 4x^2 + 6x - 24
        let root = Newton::initialize(|x| x.powi(3) - 4.0 * x.powi(2) + 6.0 * x - 24.0, 1.0)
            .tol(1e-10)
            .run();

        // Validate root
        match root {
            Ok(root) => {
                assert!((root - 4.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test6() {
        // Test case: Root near 0.739 for f(x) = cos(x) - x
        let root = Newton::initialize(|x| x.cos() - x, 1.0).tol(1e-10).run();

        // Validate root
        match root {
            Ok(root) => {
                assert!((root - 0.7390851332).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }

    #[test]
    fn test7() {
        // Test case: Root at x = e for f(x) = ln(x) - 1
        let root = Newton::initialize(|x| x.ln() - 1.0, 2.0).tol(1e-10).run();

        // Validate root
        match root {
            Ok(root) => {
                assert!((root - std::f64::consts::E).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test8() {
        // Test case: Root near 0.567 for f(x) = e^{-x} - x
        let root = Newton::initialize(|x| (-x).exp() - x, 1.0).tol(1e-10).run();

        // Validate root
        match root {
            Ok(root) => {
                assert!((root - 0.5671432904).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
}
