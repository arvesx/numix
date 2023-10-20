
///A function that assesses whether two values are equal within a given tolerance.
///
/// ## Parameters
/// - x1, x2 : values to be compared to
/// - tol: tolerance
/// - rtol: relative tolerance
/// ## Returns 
/// Boolean Value (True/False)
pub fn precision_equals(x1:f64, x2:f64, tol:f64, rtol:f64) -> bool {
    (x1 - x2).abs() <= tol + rtol * x2.abs()
}

///A function that assesse whether two vectors are equal within a given tolerance
/// 
/// ## Parameters
/// - x1, x2 : vectors to be compared to
/// - tol: tolerance
/// - rtol: relative tolerance
/// ## Returns 
/// Boolean Value (True/False)
pub fn precision_equals_vectors(x1:&Vec<f64>,x2:&Vec<f64>,tol:f64, rtol: f64)->bool{

    if x1.len()!=x2.len(){

        return false
    }
    for iter in 0..x1.len(){
        if(x1[iter] - x2[iter]).abs() > tol + rtol * x2[iter].abs(){
            println!("{}{}",x1[iter],x2[iter]);
            break
        }
        return true
    }
    return false
}

