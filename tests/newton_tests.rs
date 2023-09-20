#[cfg(test)]
pub mod newton_tests {
    use snt::optimize::root_finding::Newton;

    #[test]
    fn test_newton_high_precision() {
        // Test case 1: Root near 0 for sin(x)
        let root1 = Newton::initialize(|x| x.sin(), 1.0)
            .f_prime(|x| x.cos())
            .tolerance(1e-10)
            .run();

        // Test case 2: Root near PI for sin(x)
        let root2 = Newton::initialize(|x| x.sin(), std::f64::consts::PI)
            .f_prime(|x| x.cos())
            .f_double_prime(|x| -x.sin())
            .tolerance(1e-10)
            .run();

        // Validate root1
        match root1 {
            Ok(root1) => {
                println!("{}", root1);
                assert!(root1.abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }

        // Validate root2
        match root2 {
            Ok(root2) => {
                println!("{}", root2);
                assert!((root2 - std::f64::consts::PI).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
    #[test]
    fn test_newton_cubic_high_precision() {
        // Test case 1: Root at x = 1 for the cubic equation
        let root1 = Newton::initialize(|x| x.powi(3) - 6.0 * x.powi(2) + 11.0 * x - 6.0, 0.5)
            .f_prime(|x| 3.0 * x.powi(2) - 12.0 * x + 11.0)
            .tolerance(1e-10)
            .run();

        // Test case 2: Root at x = 3 for the cubic equation
        let root2 = Newton::initialize(|x| x.powi(3) - 6.0 * x.powi(2) + 11.0 * x - 6.0, 3.5)
            .f_prime(|x| 3.0 * x.powi(2) - 12.0 * x + 11.0)
            .f_double_prime(|x| 6.0 * x - 12.0)
            .tolerance(1e-10)
            .run();

        // Validate root1
        match root1 {
            Ok(root1) => {
                println!("{}", root1);
                assert!((root1 - 1.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }

        // Validate root2
        match root2 {
            Ok(root2) => {
                println!("{}", root2);
                assert!((root2 - 3.0).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }

    #[test]
    fn test_newton_exponential_high_precision() {
        // Constants
        let ln_2: f64 = std::f64::consts::LN_2; // Natural log of 2

        // Test case 1: Root at x = ln(2) for the exponential equation
        let root1 = Newton::initialize(|x| x.exp() - 2.0, 0.5)
            .f_prime(|x| x.exp())
            .tolerance(1e-10)
            .run();

        // Test case 2: Same root, but now with second derivative for Halley's method
        let root2 = Newton::initialize(|x| x.exp() - 2.0, 0.5)
            .f_prime(|x| x.exp())
            .f_double_prime(|x| x.exp())
            .tolerance(1e-10)
            .run();

        // Validate root1
        match root1 {
            Ok(root1) => {
                println!("{}", root1);
                assert!((root1 - ln_2).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }

        // Validate root2
        match root2 {
            Ok(root2) => {
                println!("{}", root2);
                assert!((root2 - ln_2).abs() < 1e-10);
            }
            Err(e) => panic!("Test failed due to error: {}", e),
        }
    }
}
