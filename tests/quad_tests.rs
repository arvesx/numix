#[cfg(test)]
mod general_test{
    use snt::integrate::quad::{Quad, QuadError};
    use snt::common::functions::precision_equals;
    use std::time::Instant;
    
    


    #[test]
    fn test_quad_finite(){
        //done
        let results=Quad::initialize(
            |x| x.exp() + x.ln(), 2.0, 3.0).run();       
        
        match results{
            Ok(results)=>{
                println!("{}", results);
                assert!(precision_equals(results.integral,13.60602332914145596,1e-11,0.0));
            }
            Err(results)=>{

                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }

        }
    }




    #[test]
    fn test_quad_finite2(){
        //BENCHMARK TEST 0.0309 
        //done
        let now=Instant::now();
        let results=Quad::initialize(
            |x| x.powi(3) - 2.0 * x.powi(2) + x.sin() - 1.0, -1.0, 4.0).run();       
        let elapsed=now.elapsed();
        println!("Elapsed: {:.2?}",elapsed);
        match results{
            Ok(results)=>{
                println!("{}", results);
                assert!(precision_equals(results.integral,16.610612593398418,1e-11,0.0));
            }
            Err(results)=>{

                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }
        }

    }
 




    #[test]
    fn test_quad_conv_singularity_conv(){
        // tolerance 1e-5 to default interval limit
        let now=Instant::now();
        let results=Quad::initialize(
            |x| 1.0/(3.0-x).sqrt(), 0.0, 3.0).change_tolerance(1e-5).run();       
        let elapsed=now.elapsed();
        println!("Elapsed: {:.2?}",elapsed);

        match results{
            Ok(results)=>{
                println!("{}", results);
                assert!(precision_equals(results.integral,3.464101615137754587054892683,1e-5,0.0));
            }
            Err(results)=>{

                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }

        }

    }
    #[test]
    fn test_quad_singularity2_div(){
        
        let results=Quad::initialize(
            |x| 1.0/(3.0-x).sqrt(), f64::NEG_INFINITY, 3.0).change_tolerance(1e-6).run();       
        
        match results{
            Ok(results)=>{
                println!("{}", results);
                panic!("Test failed due to error returning a value when it is divergence")
            }
            Err(results)=>{
                match results{
                    QuadError::Divergence=>{
                        println!("{}", results);
                        println!("Test passed!")
                    }
                    _=>{
                        println!("{}", results);
                        panic!("Test failed due to incorrect returning error")
                    }
                }
                
            }
        }
    }

    #[test]
    fn test_quad_posinfinite(){
        //benchmark 0.1365ms new 0.1060ms new 0.0882ms
        //done
        let now=Instant::now();
        let results=Quad::initialize(
            |x| (-x).exp()*x.sin(), 0.0, f64::INFINITY).run();       
        
        let elapsed=now.elapsed();
        println!("Elapsed: {:.2?}",elapsed);

        match results{
            Ok(results)=>{
                println!("{}", results);
                assert!(precision_equals(results.integral,0.5,1e-11,0.0));
            }
            Err(results)=>{

                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }

        }

    }
    #[test]
    fn test_quad_posinfinite2(){
        //done
        let results=Quad::initialize(
            |x| (1.0/x.powi(2)), 1.0, f64::INFINITY).run();       
        
        match results{
            Ok(results)=>{
                println!("{}", results);
                assert!(precision_equals(results.integral,1.0,1e-7,0.0));
            }
            Err(results)=>{

                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }

        }
    }
    #[test]
    fn test_quad_posinfinite3_div(){
        
        let results=Quad::initialize(
            |x| (1.0/x.powi(2)), 0.0, f64::INFINITY).change_tolerance(1e-14).run();       
        
        match results{
            Ok(results)=>{
                println!("{}", results);
                panic!("Test failed due to error returning a value when it is divergence")
            }
            Err(results)=>{
                match results{
                    QuadError::Divergence=>{
                        println!("{}", results);
                        println!("Test passed!")
                    }
                    _=>{
                        println!("{}", results);
                        panic!("Test failed due to incorrect returning error")
                    }
                }
                
            }
        }
    }
    #[test]
    fn test_quad_posinfinite4_div(){
        
        let results=Quad::initialize(
            |x| x.sin(), -2.0, f64::INFINITY).change_tolerance(1e-14).run();       
        
        match results{
            Ok(results)=>{
                println!("{}", results);
                panic!("Test failed due to error returning a value when is divergence")
            }
            Err(results)=>{
                match results{
                    QuadError::Divergence=>{
                        println!("{}", results);
                        println!("Test passed!")
                    }
                    _=>{
                        println!("{}", results);
                        panic!("Test failed due to incorrect returning error")
                    }
                }
                
            }
        }
    }
    #[test]
    fn test_quad_neginfinite(){
        //Benchmark 0.1087ms new 0.083ms
        //done
        let now=Instant::now();
        let results=Quad::initialize(
            |x| (x).exp()*x.sin(), f64::NEG_INFINITY,2.0 ).run();       
            let elapsed=now.elapsed();
            println!("Elapsed: {:.2?}",elapsed);
        match results{
            Ok(results)=>{
                println!("{}", results);
                assert!(precision_equals(results.integral,4.8968910090338044191,1e-11,0.0));
            }
            Err(results)=>{

                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }

        }
    }
    #[test]
    fn test_quad_neginfinite2_div(){

    }
    #[test]
    fn test_quad_bilateralinfinite(){
        
        let results=Quad::initialize(
            |x| x*(-x*x).exp(), f64::NEG_INFINITY, f64::INFINITY).run();       
        
        match results{
            Ok(results)=>{
                println!("{}", results);
                assert!(precision_equals(results.integral,0.0,1e-11,0.0));
            }
            Err(results)=>{

                println!("{}", results);
                panic!("Test failed due to error: {}", results)
            }

        }
    }
}