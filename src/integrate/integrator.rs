use core::fmt;
use std::collections::VecDeque;

static DEFAULT_NODES:usize=10000;
static DEFAULT_TOL:f64=1e-11;

///Output characteristics for evaluating an one dimensional integral .
///Consists of an output message , the number of nodes evaluated and the result in f64
/// ## Attributes
/// - msg:String
/// - nodes:usize
/// - integral:f64
pub struct IntegralChar{
    pub msg:String,
    pub nodes:usize,
    pub integral:f64

}
impl fmt::Display for IntegralChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f,"{}\n Nodes={} Result={}\n",
            self.msg, self.nodes,self.integral
        )
    }
}

/// Possible errors that will occured in the integration process.
pub enum IntegralError{
    None,
    IntervalError,
    UnacceptableTolearanceError(IntegralChar),
    IterationLimitExceededError(IntegralChar),
}

impl fmt::Display for IntegralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegralError::None=>{
                write!(f, "The parameters are valid\n")}
            IntegralError::IntervalError => {
                write!(f, "The interval is infinite\n")
            }
            IntegralError::UnacceptableTolearanceError(integral_char) => {
                
                write!(f,"Derivative became zero during computation.\n{}",
                    integral_char
                )
            }
            IntegralError::IterationLimitExceededError(integral_char) => {
                write!(f, "Initial guesses x0 and x1 cannot be identical.\n{}",integral_char)
            }
        }
    }
}

/// # Composite Trapezoid Integration
/// Structure that handles input parameters and runs the composite trapezoid method
/// for a number of nodes.

/// ## Methods
/// - Initialize the struct
/// - Change the Nodes 
/// - Run and Compute the integrals
/// 
pub struct CompositeTrapezoid{
    f:fn(f64)->f64,
    a: f64,
    b: f64,
    nodes: usize

}
impl CompositeTrapezoid{

    ///A method that will initialize the integration struct
    pub fn initialize(f:fn(f64)->f64,a:f64,b:f64)-> Self {
        
        Self { 

            f,
            a,
            b,
            nodes:DEFAULT_NODES  

        }
    }

    ///A method that changes the number of nodes that will be evaluated in the interval
    pub fn nodes (mut self, nodes:usize) -> Self {

        self.nodes=nodes;
        self 

    }

    pub fn run (self)->Result<IntegralChar,IntegralError> {
        

        let mut integral_char=IntegralChar{
            
            msg: String::from(""),
            nodes: self.nodes,
            integral:f64::NAN,
        };


        if !(self.a.is_finite() || self.b.is_finite()) {

            return Err(IntegralError::IntervalError)
        }

        let h = (self.b - self.a) / self.nodes as f64;

        if h <=DEFAULT_TOL{

            return Err(IntegralError::IntervalError);
        }

        let mut result = 0.5 * (self.f)(self.a) + (self.f)(self.b);

        for i in 1..self.nodes {
            let x = self.a + i as f64 * h;
            result += (self.f)(x);
        }

        result *= h;

        integral_char.msg="Integration Completed".to_string();
        integral_char.integral=result;

        return  Ok(integral_char)
    }
   

}

/// # Simpson Rule Integration
/// Structure that handles input parameters and runs the simpson 1/3 rule 
/// for a number of nodes.

/// ## Methods
/// - Initialize the struct
/// - Change the Nodes 
/// - Run and Compute the integrals
///
pub struct Simpson{

    f:fn(f64)->f64,
    a: f64,
    b: f64,
    nodes: usize
}

impl Simpson{

    ///A method that will initialize the integration struct
    pub fn initialize(f:fn(f64)->f64,a:f64,b:f64)-> Self {
        
        Self { 

            f,
            a,
            b,
            nodes:DEFAULT_NODES  

        }
    }

    ///A method that changes the number of nodes that will be evaluated in the interval
    pub fn nodes (mut self, nodes:usize) -> Self {

        self.nodes=nodes;
        self 

    }

    /// A method that runs the numerical integration and returns the result.
    pub fn run (self)->Result<IntegralChar,IntegralError> {

        let mut integral_char=IntegralChar{
            
            msg: String::from(""),
            nodes: self.nodes,
            integral:f64::NAN,
        };


        if !(self.a.is_finite() || self.b.is_finite()) {

            return Err(IntegralError::IntervalError)
        }

        let h = (self.b - self.a) / self.nodes as f64;

        if h <=DEFAULT_TOL{

            return Err(IntegralError::IntervalError);
        }

        let mut result = (self.f)(self.a) + (self.f)(self.b);

        for i in 1..self.nodes {
            let x = self.a + i as f64 * h;
            result += if i % 2 == 0 { 2.0 * (self.f)(x) } else { 4.0 * (self.f)(x) };
        }

        result *= h/ 3.0;

        integral_char.integral=result;

        return  Ok(integral_char)
    }


}




/// # Romberg Integration
/// Structure that handles input parameters and runs the romberg integration 
/// for a number of nodes what is a power of two.
/// The estimated values of the integral are stored in the struct and can be extended
/// for more precision.
/// ## Methods
/// - Initialize the struct
/// - Extend function that computes the integral
/// - Run function that returns the integral
///
pub struct Romberg<F>{

    function:F,
    a:f64,
    h:f64,
    r:VecDeque<f64>,
    size:u32,
    error_type:IntegralError

}
impl<F> Romberg<F>

where
    F:Fn(f64)->f64,
{
    ///A method that will initialize the integration struct
    pub fn initialize(function:F,a:f64,b:f64)-> Self {

        let mut rcopy:VecDeque<f64>=VecDeque::new();

        if !(a.is_finite() || b.is_finite()) {
            rcopy.push_front(0.0);
            Self{

                function:function,
                a:a,
                h:b-a,
                r:rcopy,
                size:1,
                error_type:IntegralError::IntervalError
            } 
        }
        else{

            //first evalutation
            rcopy.push_front( 0.5 * (b-a) *  ((function)(a) + (function)(b)));

            Self{

                function:function,
                a:a,
                h:b-a,
                r:rcopy,
                size:1,
                error_type:IntegralError::None
            } 
            }
    }

    /// A method that takes as input the struct parameters and a degree that determines how many times the interval points are sub divided. 
    pub fn extend(mut self,size:u32)->Self{


    let extention_size=size+1;
    


    for n in self.size..self.size+extention_size{

        let hn: f64 =self.h/(2_i32.pow(n)  as f64);
        let mut sum =0.0;
        for k in 1..(2_i32.pow(n-1)+1){
            sum += (self.function)(self.a + (2.0 * k as f64 - 1.0) * hn);
            
        }

        
        self.r.push_front( 0.5 * self.r[0] + hn * sum);

        for m in 1..=n as usize {
            self.r[m]= self.r[m-1]+ (self.r[m-1]-self.r[m])/(4.0_f64.powi(m as i32)-1.0);
            
        }
    }
    self.size+=extention_size;
    self
    
    }

    /// A method that returns the result.
    pub fn run(self)->Result<IntegralChar,IntegralError>{
        let mut int_char=IntegralChar{
            msg:"".to_string(),
            nodes:self.size.pow(2) as usize,
            integral:f64::NAN

        };

        match self.error_type{
            IntegralError::IntervalError=>{
                return Err(IntegralError::IntervalError)
            }
            _=>{
                int_char.msg="Completed Integration".to_string();
                int_char.integral=*self.r.back().unwrap();
                return Ok(int_char)
            }

        }
   
    }
}


