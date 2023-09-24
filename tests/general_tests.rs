#[cfg(test)]
mod general_tests {
    use snt::optimize::root_finding::precision_equals;

    #[test]
    fn test_exact_equals() {
        let x1 = 42.0;
        let x2 = 42.0;
        let tol = 1e-5;
        let rtol = 1e-5;
        assert!(precision_equals(&x1, &x2, &tol, &rtol));
    }

    #[test]
    fn test_close_enough() {
        let x1 = 42.00001;
        let x2 = 42.0;
        let tol = 1e-5;
        let rtol = 1e-5;
        assert!(precision_equals(&x1, &x2, &tol, &rtol));
    }

    #[test]
    fn test_not_close() {
        let x1 = 42.1;
        let x2 = 42.0;
        let tol = 1e-5;
        let rtol = 1e-5;
        assert!(!precision_equals(&x1, &x2, &tol, &rtol));
    }

    #[test]
    fn test_with_zero() {
        let x1 = 0.0;
        let x2 = 0.0;
        let tol = 1e-5;
        let rtol = 1e-5;
        assert!(precision_equals(&x1, &x2, &tol, &rtol));
    }

    #[test]
    fn test_with_negative_numbers() {
        let x1 = -42.0;
        let x2 = -42.00001;
        let tol = 1e-5;
        let rtol = 1e-5;
        assert!(precision_equals(&x1, &x2, &tol, &rtol));
    }
}
