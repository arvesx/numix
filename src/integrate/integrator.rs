use std::collections::VecDeque;

pub struct CompositeTrapezoid<F>
{
    function: F,
    a: f64,
    b: f64,
    
    size: usize

}

impl<F> CompositeTrapezoid<F>
where
    F:Fn(f64)->f64,
{
    pub fn initialize(function:F,a:f64,b:f64)-> Self {
        Self { 
    
            function,
            a,
            b,
            size:10000 }

    }

    pub fn steps (mut self, size:usize) -> Self {

        self.size=size;
        self 

    }

    pub fn run (self)->f64 {


        let h = (self.b - self.a) / self.size as f64;

        let mut result = 0.5 * (self.function)(self.a) + (self.function)(self.b);

        for i in 1..self.size {
            let x = self.a + i as f64 * h;
            result += (self.function)(x);
        }

        result *= h;
        return  result;
    }
   

}

pub struct Romberg<F>{

    function:F,
    a:f64,
    b:f64,
    h:f64,
    r:VecDeque<f64>,
    size:u32

}
impl<F> Romberg<F>

where
    F:Fn(f64)->f64,
{
    pub fn initialize(function:F,a:f64,b:f64)-> Self {

        let mut rcopy:VecDeque<f64>=VecDeque::new();
        rcopy.push_front( 0.5 * (b-a) *  ((function)(a) + (function)(b)));

        Self{

            function:function,
            a:a,
            b:b,
            h:b-a,
            r:rcopy,
            size:1


        }
        
        
        
        

    }
pub fn extend(mut self,size:u32)->Self{


    let mut extentionSize=size+1;


    for n in self.size..self.size+extentionSize{

        let mut hn: f64 =self.h/(2_i32.pow(n)  as f64);
        let mut sum =0.0;
        for k in 1..(2_i32.pow(n-1)+1){
            sum += (self.function)(self.a + (2.0 * k as f64 - 1.0) * hn);
            
        }

        
        self.r.push_front( 0.5 * self.r[0] + hn * sum);

        for m in 1..=n as usize {
            self.r[m]= self.r[m-1]+ (self.r[m-1]-self.r[m])/(4.0_f64.powi(m as i32)-1.0);
            
        }
    }

    self



    
}

pub fn run(self)->f64{

     *self.r.back().unwrap()
   
    
}
}

pub struct GaussQuad<F>{



    function:F,

    lowerbound:f64,
    upperbound:f64,

    number_of_nodes:u32,
    tolerance:f64,

    integral_type:String,
    singularities:Vec<f64>,
    integration_method:String,

    terminal_condition:String,
    exit_condition:String

}

/*
integral types: bounded non-singular
                bounded singularites
                infinite interval non-singular
                infinite interval singularites
                fourier transform
                oscillatory integral
                none aka trivial solution=0
*/

/*
integral methods: Gauss

*/

/*
terminal conditions: initial tolerance 
                     numberofnodes
                     demanded tolerance
                     
                     
                    
*/
/*
exitcondition:  noexit
                tolerance satisfied
                numberofnodes exhausted
                iterations exhausted
                divergence
                field error
                overflow error

*/

impl <F> GaussQuad<F> 
where
    F:Fn(f64)->f64
{



pub fn initialize(function:F,lowerbound:f64,upperbound:f64)->Self{

    //categorizing integral

    let integral_type:String;
    let integration_method:String;
    let singularities:Vec<f64>;
    let terminal_condition:String="initial_tolerance";
    
    
    if lowerbound==upperbound {

        integral_type="none";

        integration_method="gauss";
        

   
    }

    singularities=singularites(function, lowerbound, upperbound);
    if singularities.is_empty() {

        if upperbound.is_infinite() || lowerbound.is_infinite() {

            integral_type="infinite interval";

            integration_method="gauss";

        } else{

            integral_type="bounded";
            
            integration_method="gauss";

        }
    }
    else{

        if upperbound.is_infinite() || lowerbound.is_infinite() {

            integral_type="infinite interval singularities";

            integration_method="gauss";

        }else{

            integral_type="bounded singularities";
            
            integration_method="gauss";

        }

    }
        

    Self{

        function:function,

        lowerbound:lowerbound,
        upperbound:upperbound,

        number_of_nodes:100,
        tolerance:10.0_f64.powf(-6.0),

        integral_type:integral_type,
        integration_method,
        singularities:singularities,

        terminal_condition:terminal_condition,
        exit_condition:"no exit"



    }


    

}

pub fn change_tolerance(mut self, tol:f64)->Self{

    self.tolerance=tol;
    self.terminal_condition="demanded tolerance";
    self
        


}
pub fn oscillatory(mut self)->Self{


    self.integral_type="oscilatory";
    self
}



pub fn run(mut self)->f64{

    let mut solution:f64=0.0;
    self.exit_condition="tolerance satisfied";

    return solution;



}

pub fn exit_condition(self)->String{
    
    self.exit_condition


}

}


pub fn singularites<F>(function:F,lowerbound:f64,upperbound:f64)->Vec<f64>{
    if lowerbound>=upperbound {

        panic!("Irrational Bounds");
    }

    return Vec::new();


}



pub fn laplace_transform<F>(fucntion:F, tolerance:f64)->bool{

    /*
    norma4
    bilateral
     */
    return true


}

pub fn fourier_transform<F>(function:F,transformtype:String, tolerance:f64)->bool{

    /* 
        normalised sine
        non-normalised sine
        normalised cosine
        non-normalised cosine

        normalised fft
        non-normalised fft
     */
    return true




}

