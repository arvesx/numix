///A function that computes the positive integer factorials 
pub fn factorial(n: u64) -> u64 {
    let mut result =1;
    for i in 1..=n {
        result *= i;
    }
    result
}
///A function that computes binomial coefficients
pub fn binomial(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let min=std::cmp::min(n-k,k);
    
    let mut result:u64=1;

    for i in 0..min {

        result*=n-i;
        result /= i + 1;
    }
    return result;
}