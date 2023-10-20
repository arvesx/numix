#[cfg(test)]
mod general_test {
    use numix::arithmetic::binomial::binomial;
    use numix::common::functions::precision_equals_vectors;
    use numix::special::polynomials::bessel_polynomials;
    use numix::special::polynomials::chebyshev_first_kind_polynomials;
    use numix::special::polynomials::chebyshev_second_kind_polynomials;
    use numix::special::polynomials::hermite_polynomials;
    use numix::special::polynomials::laguerre_polynomials;
    use numix::special::polynomials::legendre_polynomials;

    #[test]

    fn test_binomial() {
        let res1 = binomial(3, 1);
        println!("{}", res1);
        let res2 = binomial(5, 3);
        println!("{}", res2);
        let res3 = binomial(7, 4);
        println!("{}", res3);
    }

    #[test]
    fn test_bessel() {
        //done
        let coef1 = bessel_polynomials(3);

        let coef2 = bessel_polynomials(4);

        let coef3 = bessel_polynomials(5);

        println!("{:?}", coef1);
        assert!(precision_equals_vectors(
            &coef1,
            &vec![1.0, 6.0, 15.0, 15.0],
            1e-8,
            0.0
        ));

        println!("{:?}", coef2);
        assert!(precision_equals_vectors(
            &coef2,
            &vec![1.0, 10.0, 45.0, 105.0, 105.0],
            1e-8,
            0.0
        ));

        println!("{:?}", coef3);
        assert!(precision_equals_vectors(
            &coef3,
            &vec![1.0, 15.0, 105.0, 420.0, 945.0, 945.0],
            1e-8,
            0.0
        ));
    }
    #[test]
    fn test_legendre() {
        let coef1 = legendre_polynomials(4);

        let coef2 = legendre_polynomials(5);

        let coef3 = legendre_polynomials(6);

        println!("{:?}", coef1);
        assert!(precision_equals_vectors(
            &coef1,
            &vec![0.375, 0.0, -3.750, 0.0, 4.375],
            1e-8,
            0.0
        ));

        println!("{:?}", coef2);
        assert!(precision_equals_vectors(
            &coef2,
            &vec![0.0, 1.875, 0.0, -8.75, 0.0, 7.875],
            1e-8,
            0.0
        ));

        println!("{:?}", coef3);
        assert!(precision_equals_vectors(
            &coef3,
            &vec![-0.3125, 0.0, 6.5625, 0.0, -19.6875, 0.0, 14.4375],
            1e-8,
            0.0
        ));
    }
    #[test]
    fn test_laguerre() {
        //done
        let coef1 = laguerre_polynomials(5);

        let coef2 = laguerre_polynomials(6);

        let coef3 = laguerre_polynomials(7);

        println!("{:?}", coef1);
        assert!(precision_equals_vectors(
            &coef1,
            &vec![
                1.0,
                -5.0,
                5.0,
                -1.6666666666666667,
                0.20833333333333334,
                -0.008333333333333333
            ],
            1e-8,
            0.0
        ));

        println!("{:?}", coef2);
        assert!(precision_equals_vectors(
            &coef2,
            &vec![
                1.0,
                -6.0,
                7.5,
                -3.3333333333333335,
                0.625,
                -0.05,
                0.001388888888888889
            ],
            1e-8,
            0.0
        ));

        println!("{:?}", coef3);
        assert!(precision_equals_vectors(
            &coef3,
            &vec![
                1.0,
                -7.0,
                10.5,
                -5.833333333333333,
                1.4583333333333333,
                -0.175,
                0.009722222222222222,
                -0.0001984126984126984
            ],
            1e-8,
            0.0
        ));
    }
    #[test]
    fn test_hermite() {
        //done
        let coef1 = hermite_polynomials(8);

        let coef2 = hermite_polynomials(9);

        let coef3 = hermite_polynomials(10);

        println!("{:?}", coef1);
        assert!(precision_equals_vectors(
            &coef1,
            &vec![1680.0, 0.0, -13440.0, 0.0, 13440.0, 0.0, -3584.0, 0.0, 256.0],
            1e-8,
            0.0
        ));

        println!("{:?}", coef2);
        assert!(precision_equals_vectors(
            &coef2,
            &vec![0.0, 30240.0, 0.0, -80640.0, 0.0, 48384.0, 0.0, -9216.0, 0.0, 512.0],
            1e-8,
            0.0
        ));

        println!("{:?}", coef3);
        assert!(precision_equals_vectors(
            &coef3,
            &vec![
                -30240.0, 0.0, 302400.0, 0.0, -403200.0, 0.0, 161280.0, 0.0, -23040.0, 0.0, 1024.0
            ],
            1e-8,
            0.0
        ));
    }
    #[test]
    fn test_chebyshev_one() {
        let coef1 = chebyshev_first_kind_polynomials(3);

        let coef2 = chebyshev_first_kind_polynomials(4);

        let coef3 = chebyshev_first_kind_polynomials(5);

        println!("{:?}", coef1);
        assert_eq!(coef1, vec![0.0, -3.0, 0.0, 4.0]);

        println!("{:?}", coef2);
        assert_eq!(coef2, vec![1.0, 0.0, -8.0, 0.0, 8.0]);

        println!("{:?}", coef3);
        assert_eq!(coef3, vec![0.0, 5.0, 0.0, -20.0, 0.0, 16.0]);
    }
    #[test]
    fn test_chebyshev_two() {
        let coef1 = chebyshev_second_kind_polynomials(3);

        let coef2 = chebyshev_second_kind_polynomials(5);

        let coef3 = chebyshev_second_kind_polynomials(6);

        println!("{:?}", coef1);
        assert_eq!(coef1, vec![0.0, -4.0, 0.0, 8.0]);

        println!("{:?}", coef2);
        assert_eq!(coef2, vec![0.0, 6.0, 0.0, -32.0, 0.0, 32.0]);

        println!("{:?}", coef3);
        assert_eq!(coef3, vec![-1.0, 0.0, 24.0, 0.0, -80.0, 0.0, 64.0]);
    }
}
