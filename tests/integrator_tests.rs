#[cfg(test)]
mod general_test {
    use numix::common::functions::precision_equals;
    use numix::integrate::integrator::CompositeTrapezoid;
    use numix::integrate::integrator::IntegralChar;
    use numix::integrate::integrator::IntegralError;
    use numix::integrate::integrator::Romberg;
    use numix::integrate::integrator::Simpson;
    use std::time::Instant;

    #[test]
    fn test_composite_rule() {
        let res1 = CompositeTrapezoid::initialize(|x| x.exp() + x.ln(), 2.0, 3.0)
            .nodes(100000000)
            .run();

        match res1 {
            Ok(res1) => {
                println!("{}", res1);
                assert!(precision_equals(
                    res1.integral,
                    13.60602332914145596,
                    1e-2,
                    0.0
                ));
            }
            Err(res1) => {
                println!("{}", res1);
                panic!("Test failed due to error: {}", res1)
            }
        }
    }

    #[test]
    fn test_composite_rule2() {
        //BENCHMARK TEST tol 1e-5 at nodes 1e+7 0.98s

        let now = Instant::now();
        let results = CompositeTrapezoid::initialize(
            |x| x.powi(3) - 2.0 * x.powi(2) + x.sin() - 1.0,
            -1.0,
            4.0,
        )
        .nodes(10000000)
        .run();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        match results {
            Ok(results) => {
                println!("{}", results);
                assert!(precision_equals(
                    results.integral,
                    16.610612593398418,
                    1e-5,
                    0.0
                ));
            }
            Err(results) => {
                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }
        }
    }

    #[test]
    fn test_simpson1() {
        let results: Result<IntegralChar, IntegralError> =
            Simpson::initialize(|x| x.powi(2) * x.sin() - 2.0, -3.0, 2.0)
                .nodes(10000)
                .run();

        match results {
            Ok(results) => {
                println!("{}", results);
                assert!(precision_equals(
                    results.integral,
                    -13.307184144165309977926262457,
                    1e-12,
                    0.0
                ));
            }
            Err(results) => {
                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }
        }
    }

    #[test]
    fn test_simpson2() {
        //Benchmark tol 1e-11 nodes 806 0.101 ms

        let now = Instant::now();
        let results =
            Simpson::initialize(|x| x.powi(3) - 2.0 * x.powi(2) + x.sin() - 1.0, -1.0, 4.0)
                .nodes(806)
                .run();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        match results {
            Ok(results) => {
                println!("{}", results);
                assert!(precision_equals(
                    results.integral,
                    16.610612593398418,
                    1e-11,
                    0.0
                ));
            }
            Err(results) => {
                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }
        }
    }

    #[test]
    fn test_romberg1() {
        let results = Romberg::initialize(|x| x.powi(2) * x.sin() - 2.0, -3.0, 2.0)
            .extend(5)
            .extend(2)
            .run();
        let dresult = Romberg::initialize(|x| x.powi(2) * x.sin() - 2.0, -3.0, 2.0)
            .extend(7)
            .run();
        match results {
            Ok(results) => {
                println!("{}", results);
                assert!(precision_equals(
                    results.integral,
                    -13.307184144165309977926262457,
                    1e-11,
                    0.0
                ));
            }
            Err(results) => {
                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }
        }
        match dresult {
            Ok(dresult) => {
                println!("DResult: {}", dresult.integral);
                assert!(precision_equals(
                    dresult.integral,
                    -13.307184144165309977926262457,
                    1e-11,
                    0.0
                ));
            }
            Err(dresult) => {
                println!("Error: {}", dresult);
                panic!("Test failed due to error: {}", dresult);
            }
        }
    }

    #[test]
    fn test_romberg2() {
        //benchmark tol 1e-11 size=6 nodes=65
        let now = Instant::now();
        let results =
            Romberg::initialize(|x| x.powi(3) - 2.0 * x.powi(2) + x.sin() - 1.0, -1.0, 4.0)
                .extend(5)
                .run();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        match results {
            Ok(results) => {
                println!("{}", results);
                assert!(precision_equals(
                    results.integral,
                    16.610612593398418,
                    1e-11,
                    0.0
                ));
            }
            Err(results) => {
                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }
        }
    }
}
