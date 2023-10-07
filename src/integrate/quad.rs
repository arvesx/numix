use core::fmt;

//Default Values and Mathmatical Parameters
static DEFAULT_TOL:f64=1e-11;
static DEFAULT_RTOL:f64=0.001;
static LIMIT_TOL:f64=4.0*f64::EPSILON;
static DEFAULT_SUBINTERVAL_LIMIT:usize=10000;

static  W:[f64;5]=[0.5688888888888889,0.4786286704993665,0.4786286704993665,0.2369268850561891,0.2369268850561891];
static  X:[f64;5]=[0.0,-0.5384693101056831,0.5384693101056831, 0.9061798459386639,-0.9061798459386639];


///Output characteristics for evaluating an one dimensional integral using Adaptive Gauss Quadrature.
///Consists of an output message , the number of intervals needed for the result was to be computed,
///an error estimate given by the sum of errors and the result in f64
/// ## Attributes
/// - msg:String
/// - number_of_intervals:usize
/// - error_estimate:f64
/// - integral:f64

pub struct QuadCharacteristics{
    pub msg:String,
    pub number_of_intervals:usize,
    pub error_estimate:f64,
    pub integral:f64

}
impl fmt::Display for QuadCharacteristics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f,"{}\nWith {} subintervals, the result is {} with error {}",
            self.msg, self.number_of_intervals,self.integral ,format!("{:.5e}",self.error_estimate)
        )
    }
}



///Consists of error that happen during exclusively in the run routine.
enum QuadProcessError{
    None,
    SubintervalLimitExceededError,
    Divergence

}
///Errors during integration that occur within every method in the Quad struct.
///## Types
/// - Invalid Input
/// - Interval is NaN 
/// - Divergence Occured
/// - Tolerance Requirments Not Met (Various Issues)
pub enum QuadError {
    None,
    InvalidInput(String),
    IntervalError,
    Divergence,
    UnacceptableTolearanceError(QuadCharacteristics),

}
impl fmt::Display for QuadError{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuadError::None=>{
                write!(f,"\n")
            }
            QuadError::InvalidInput(message)=>{
                write!(f,"The algorithm could not start due to {}\n",message)
            }
            QuadError::IntervalError=>{
                write!(f,"The interval is not valid\n")
            }
            QuadError::UnacceptableTolearanceError(char)=>{
                write!(f,"The algorithm has terminated without meeting the tolerance requirements.
                    The integral may differge or be irregular on some points\n,{}",char)
                
            }
            QuadError::Divergence=>{
                write!(f,"The integral is guarented to diverge\n")
            }


        }
    }


}

/// # Quadrature Integration
/// Structure that handles input parameters and runs the appropriate
/// quadrature type on specific interval and function type.
/// Stores the reference to the function, the interval endpoints, the tolerance attributes, the error type if occured,
/// the limit of the subintervals to be created and the final result. 
/// 
/// ## Methods
/// - Initialize the struct
/// - Change Tolerance Parameters
/// - Change in Integral Type
/// - Run and Compute the integrals
pub struct Quad{

    f:fn(f64)->f64,
    a: f64,
    b: f64,
    limit_subintevals: usize,
    tolerance:f64,
    relative_tolerance:f64,

    error_type:QuadError
    
    

}
impl Quad{

/// Initialize a new GaussQuad instance with the given function,
/// lower and upper bounds.
/// ## Parameters
/// 
/// - function: fn(f64)->f64
/// - a: f64, b: f64 : interval endpoints
/// 
/// ## Returns
/// - Initialized Quad Struct
pub fn initialize(function:fn(f64)->f64,a:f64,b:f64)->Self{

    Self{

        f:function,
        a: a,
        b: b,
        limit_subintevals: DEFAULT_SUBINTERVAL_LIMIT,
        
        tolerance:DEFAULT_TOL,
        relative_tolerance:DEFAULT_RTOL,
        
        error_type:QuadError::None
        

    }
}

///A method that changes the tolerance of the computation
pub fn change_tolerance(mut self, tol:f64)->Self{
    if tol<LIMIT_TOL {
        self.error_type=QuadError::InvalidInput("Invalid Tolerance\n".to_string());
    }
    self.tolerance=tol;
    self
}

///A method that changes the relative tolerance of the computation
pub fn change_relative_tolerance(mut self, rtol:f64)->Self{

    if rtol<LIMIT_TOL {
        self.error_type=QuadError::InvalidInput("Invalid Relative Tolerance\n".to_string());
    }

    self.relative_tolerance=rtol;
    self
}


/// A method that runs the numerical integration and return the result.
/// Firstly check for accumulated errors in the input, then decides 
/// which function to call and with which parameters to start the computation.
/// ## Underlying Computing Functions
/// - quad_finite
/// - quad_intinite
pub fn run(self)->Result<QuadCharacteristics,QuadError> {
    
    let mut quadchar=QuadCharacteristics { 
        msg:"".to_string(),
        number_of_intervals:1,
        error_estimate:0.0,
        integral:f64::NAN 
    };


    //Handles the errors accumulated before run method
    match self.error_type {
        QuadError::None=>{}
        _=>{
            return Err(self.error_type)
        }
    }
    let solution:f64;
    let mut error_type:QuadProcessError=QuadProcessError::None;
    
    
    //Splits the interval cases
    if self.a.is_finite() && self.b.is_finite(){

        solution=Self::quad_finite(&self.f,0.0,self.a,self.b, self.tolerance,self.relative_tolerance,
            self.limit_subintevals,&mut quadchar.number_of_intervals,&mut error_type,&mut quadchar.error_estimate)
    }
    else if self.a.is_finite() && self.b.is_infinite(){

        solution=Self::quad_infinite(self.f,self.a,1,self.tolerance,self.relative_tolerance,
            self.limit_subintevals,&mut quadchar.number_of_intervals,&mut error_type,&mut quadchar.error_estimate)
    }
    else if self.a.is_infinite() && self.b.is_finite(){
        solution=Self::quad_infinite(self.f,self.b,-1,self.tolerance,self.relative_tolerance,
            self.limit_subintevals,&mut quadchar.number_of_intervals,&mut error_type,&mut quadchar.error_estimate)

    }
    else if self.a.is_infinite() && self.b.is_infinite() {
        
        solution=Self::quad_infinite(self.f,0.0,-1,self.tolerance,self.relative_tolerance,
            self.limit_subintevals,&mut quadchar.number_of_intervals,&mut error_type,&mut quadchar.error_estimate)+
            Self::quad_infinite(self.f,0.0,1,self.tolerance,self.relative_tolerance,
                self.limit_subintevals,&mut quadchar.number_of_intervals,&mut error_type,&mut quadchar.error_estimate)

    }   
    else {
        return Err(QuadError::IntervalError);
    }
    
    
    match error_type {
        QuadProcessError::SubintervalLimitExceededError=>{
            quadchar.msg="Unacceptable Tolerance due to meating subintervals number limit\n".to_string();
            quadchar.integral=solution;
            return  Err(QuadError::UnacceptableTolearanceError(quadchar));
        }
        _=>{ 
            quadchar.msg="Completed Integration".to_string();   
            quadchar.integral=solution;
            return  Ok(quadchar)
        }

        
    }


}


///Computes the integral in an finite interval
///implemented with an adaptive 5 point gauss-legendre quadrature.
/// ## Parameters
/// - function: &Fn(f64)->f64 (reference)
/// - approx: f64 
/// - a: f64, b: f64 : integral endpoints 
/// - tolerance: f64, rtolerance:f64 : tolerance attributes
/// - limit_iter:usize : Limit of subintervals created
/// - iter: &mut usize : Starting with zero , passed as reference
/// - error_type: &mut QuadProcessError : Starting with None type
/// - error_estimate:&mut f64 : Sum of errors passed as reference
/// 
/// ## Returns
/// - solution:f64
fn quad_finite<F:Fn(f64)->f64>(function:&F,approx:f64 ,a: f64, b: f64, tolerance: f64,rtolerance:f64,limit_iter:usize,iter: &mut usize,error_type: &mut QuadProcessError, error_estimate:&mut f64)->f64{
    

    *iter+=1;

    let midpoint:f64 = a + (b-a) / 2.0;
    let center: f64=(midpoint-a)/2.0;
    

    let mut left_area:f64=0.0;
    let mut right_area:f64=0.0;

    for i in 0..W.len() {
        
        left_area += ((*function)((X[i]+1.0)*center+a))* W[i];
        right_area += ((*function)((X[i]+1.0)*center+midpoint))* W[i];
    }
    
    left_area= left_area * center;
    right_area= right_area * center;


    //Next Iteration Desicion Tree
    match *error_type {
        QuadProcessError::None=>{}
        _=>{
            return left_area+right_area;
        }
    }
    if  *iter >= limit_iter {

        *error_type=QuadProcessError::SubintervalLimitExceededError;
        return left_area + right_area;
    } 
    else if (approx - (left_area + right_area)).abs() <= tolerance {

        
        *error_estimate+=(approx - (left_area + right_area)).abs();
        return left_area + right_area;

    }else if (approx - (left_area + right_area)).abs()<1.0 {
        *error_type=QuadProcessError::Divergence;
        return left_area+right_area;
    }else {
        
        let left_result = Self::quad_finite(function, left_area, a, midpoint, tolerance / 2.0,rtolerance,limit_iter,iter,error_type,error_estimate);
        let right_result = Self::quad_finite(function,right_area, midpoint, b, tolerance / 2.0,rtolerance,limit_iter,iter,error_type,error_estimate);
        return left_result + right_result;
    }

}

///Computes the integral in an infinite interval by changing the variable
///and calling the quad_finite function for a finite interval.
/// ## Parameters
/// - function: fn(f64)->f64
/// - a: f64, b: f64 : integral endpoints 
/// - inf:i32 : Type of infinity (1 or -1)
/// - tolerance: f64, rtolerance:f64 : tolerance attributes
/// - limit_iter:usize : Limit of subintervals created
/// - iter: &mut usize : Starting with zero , passed as reference
/// - error_type: &mut QuadProcessError : Starting with None type
/// - error_estimate:&mut f64 : Sum of errors passed as reference
/// 
/// ## Returns
/// - solution:f64
fn quad_infinite(function:fn(f64)->f64 ,a: f64, inf: i32, tolerance: f64,rtolerance:f64,limit_iter:usize,iter: &mut usize,error_type: &mut QuadProcessError, error_estimate:&mut f64)->f64{

    let adjusted_function=|x:f64|->f64 {
        let result=(function)(1.0/x)/(x).powi(2);
        result
    };

    if inf==1{
        if a < 1.0{
            
            return Self::quad_finite(&function, 0.0, a,1.0 , tolerance,rtolerance,limit_iter,iter,error_type,error_estimate)
                -Self::quad_finite(&adjusted_function, 0.0, 1.0,0.0 , tolerance,rtolerance,limit_iter,iter,error_type,error_estimate);
        }
        else {
            return Self::quad_finite(&adjusted_function, 0.0, 0.0,1.0 /a, tolerance,rtolerance,limit_iter,iter,error_type,error_estimate);
        }
    }
    else {

        if a >-1.0{
            
            return Self::quad_finite(&function, 0.0, -1.0,a , tolerance,rtolerance,limit_iter,iter,error_type,error_estimate)
                +Self::quad_finite(&adjusted_function, 0.0, -1.0,0.0 , tolerance,rtolerance,limit_iter,iter,error_type,error_estimate);
        }
        else {
            return Self::quad_finite(&adjusted_function, 0.0, 0.0,1.0 /a, tolerance,rtolerance,limit_iter,iter,error_type,error_estimate);
        }
        
    }
    

}


    
}
