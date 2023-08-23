



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



/* 
macro_rules! composite_trapezoid {
    ($function:expr , $a:expr, $b:expr) => {
        composite_trapezoid($function,$a, $b, 1000);
    };

    ($function:expr, $a:expr, $b:expr, $n:expr) => {
        composite_trapezoid($function,$a, $b, $n);
    };
}
*/