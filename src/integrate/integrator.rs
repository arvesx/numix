

pub struct DxIntegratorArgs{

    //function: Fn(f64)->f64;
    //a:f64;
    //b:f64;
    n:usize

}

impl Default for DxIntegratorArgs{

    fn default() -> Self {
        DxIntegratorArgs { n:1000 }
}
}

// does not give number of points
impl From<()> for DxIntegratorArgs {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

//gives number of points
impl From<usize> for DxIntegratorArgs { 
    fn from(n: usize) -> Self {
        Self {
            n:n,
            ..Self::default()
        }
    }
}


pub fn composite_trapezoid<F,N>(function: F, a: f64, b: f64, arg_like:N) -> f64


where F: Fn(f64) -> f64, N: Into<DxIntegratorArgs>,
{

    let args= arg_like.into();
    let size=args.n;
    //declare step size
    let h = (b - a) / size as f64;

    let mut result = 0.5 * (function(a) + function(b));

    for i in 1..size {
        let x = a + i as f64 * h;
        result += function(x);
    }

    result *= h;
    return result;



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