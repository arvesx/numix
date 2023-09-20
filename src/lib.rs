pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod bisection_tests {
    use crate::optimize::root_finding::Bisection;

    #[test]
    fn test_quadratic() {
        let root1 = Bisection::initialize(|x| x * x - 4.0, -3.0, -1.0)
            .tolerance(1e-5)
            .run();
        let root2 = Bisection::initialize(|x| x * x - 4.0, 1.0, 3.0)
            .tolerance(1e-5)
            .run();

        assert!((root1 + 2.0).abs() < 1e-5);
        assert!((root2 - 2.0).abs() < 1e-5);
    }
    #[test]
    fn test_quadratic_high_precision() {
        let root1 = Bisection::initialize(|x| x * x - 4.0, -3.0, -1.0)
            .tolerance(1e-10)
            .iterations(10000)
            .run();
        let root2 = Bisection::initialize(|x| x * x - 4.0, 1.0, 3.0)
            .tolerance(1e-10)
            .iterations(10000)
            .run();

        assert!((root1 + 2.0).abs() < 1e-10);
        assert!((root2 - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_sine() {
        let root1 = Bisection::initialize(|x| x.sin(), -1.0, 1.0)
            .tolerance(1e-5)
            .run();
        let root2 = Bisection::initialize(|x| x.sin(), 2.0, 4.0)
            .tolerance(1e-5)
            .run();

        assert!(root1.abs() < 1e-5);
        assert!((root2 - std::f64::consts::PI).abs() < 1e-5);
    }
    #[test]
    fn test_sine_high_precision() {
        let root1 = Bisection::initialize(|x| x.sin(), -1.0, 1.0)
            .tolerance(1e-10)
            .iterations(10000)
            .run();
        let root2 = Bisection::initialize(|x| x.sin(), 2.0, 4.0)
            .tolerance(1e-10)
            .iterations(10000)
            .run();

        assert!(root1.abs() < 1e-10);
        assert!((root2 - std::f64::consts::PI).abs() < 1e-10);
    }
    #[test]
    fn test_exponential() {
        let root = Bisection::initialize(|x| x.exp() - 2.0, 0.0, 1.0)
            .tolerance(1e-5)
            .run();
        assert!((root - 2.0f64.ln()).abs() < 1e-5);
    }

    #[test]
    fn test_linear() {
        let root = Bisection::initialize(|x| x, -1.0, 1.0)
            .tolerance(1e-5)
            .run();
        assert!(root.abs() < 1e-5);
    }
}

pub mod optimize {
    pub mod root_finding;
}

pub mod interpolate {
    pub mod polynomial;
}
pub mod integrate {
    pub mod integrator;
}
