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

