use crate::arithmetic::binomial::{binomial, factorial};


///Bessel Polynomials are solutions to : (x^2)y''+2(x+1)y'-n(n+1)y=0
///and are orthogonal with respect to integral 0 -> 2pi {Pn(e^iθ)*Pm(e^iθ)*(i*e^iθ)}
///definition: Pn(x) = sum { (n+k)!/((n-k)!*k!*2^k) * x^k }
pub fn bessel_polynomials(degree:u64)->Vec<f64>{


    let mut coefficients: Vec<f64>= Vec::new();

    for iter in 0..=degree{
        
        let coef: f64 = (factorial(degree+iter) as f64)/(factorial(degree-iter) as f64)/(factorial(iter) as f64)/(2.0_f64.powi(iter as i32));
        coefficients.push(coef)
   
    }

    return coefficients
}    




///Laguerre Polynomials are solutions to : (1-x^2)y''-2xy'+n(n+1)y=0
///and are orthogonal with respect to integral -1 -> 1 {Pn*Pm}
///definition recursive form : n*Pn(x)=(2n-1)x*Pn-1-(n-1)*Pn-2
pub fn legendre_polynomials(n:u64)->Vec<f64>{
    if n==0{
        return vec![1.0];

    }
    else if n==1{
        return vec![0.0,1.0];
    }
    else{
        let uppermultiplier=((2*n-1) as f64)/(n as f64);
        let lowermultiplier=((n-1) as f64)/(n as f64);

        let mut upperlegendre=legendre_polynomials(n-1);
        upperlegendre.insert(0,0.0 );
        
        let scaleduppper:Vec<f64>=upperlegendre.iter().map(|&x| x * uppermultiplier).collect();
        
        let mut lowerlegendre=legendre_polynomials(n-2);
        lowerlegendre.push(0.0);
        lowerlegendre.push(0.0);
        
        
        let scaledlower:Vec<f64>=lowerlegendre.iter().map(|&x| x * lowermultiplier).collect();

        
        return scaleduppper.iter().zip(scaledlower.iter()).map(|(a, b)| a - b).collect();
    }

}


///Laguerre Polynomials are solutions to : xy''+(1-x)y'+ny=0
///and are orthogonal with respect to integral 0 -> inf {Pn*Pm*e^-x}
///definition closed form: Pn(x)= sum { C(n,k)*(-1)^k/k! * x^k }
pub fn laguerre_polynomials(degree:u64)->Vec<f64>{

    
    
    let mut coefficients: Vec<f64>= Vec::new();

    for iter in 0..=degree{

        let coef:f64=(binomial(degree, iter) as f64)*(alt_sign(iter) as f64)/(factorial(iter) as f64);
        coefficients.push(coef)    
    }
    
    return coefficients
         
}




///Chebyshev Polynomials of the first kind are solutions to :
/// y(cosθ)=cos(nΘ) and (1-x^2)y''-xy'+n^2y=0
///and are orthogonal with respect to integral -1 -> 1 {Pn*Pm/sqrt(1-x^2)}
///definition recursive form : Pn(x)=2x*Pn-1-Pn-2 
pub fn chebyshev_first_kind_polynomials(degree:u64)->Vec<f64>{

    return chebyshev_polynomials(degree, 1);
}


///Chebyshev Polynomials of the secpnd kind are solutions to :
/// y(cosθ)*sinθ=sin((n+1)θ) and (1-x^2)y''-3xy'+n(n+2)y=0
///and are orthogonal with respect to integral -1 -> 1 {Pn*Pm*sqrt(1-x^2)}
///definition recursive form : Pn(x)=2x*Pn-1-Pn-2

pub fn chebyshev_second_kind_polynomials(degree:u64)->Vec<f64>{
    let kind=2;
    return chebyshev_polynomials(degree, kind);

}

fn chebyshev_polynomials(n:u64,kind:u64)->Vec<f64>{
    if n==0{
        return vec![1.0];

    }
    else if n==1{
        if kind==1{
            return vec![0.0,1.0];
        }
        else{
            return vec![0.0,2.0];
        }
    }
    else{
        let mut higher_chebyshev=chebyshev_polynomials(n-1,kind);
        higher_chebyshev.insert(0,0.0 );
        
        let scaled_higher_chebyshev:Vec<f64>=higher_chebyshev.iter().map(|&x| x * 2.0).collect();
        
        let mut lower_chebyshev=chebyshev_polynomials(n-2,kind);
        lower_chebyshev.push(0.0);
        lower_chebyshev.push(0.0);
        
        
        return scaled_higher_chebyshev.iter().zip(lower_chebyshev.iter()).map(|(a, b)| a - b).collect();
    }
}




///Hermite Polynomials are solutions to : y''-2xy'+2ny=0
///and are orthogonal with respect to integral -inf -> inf {Pn*Pm*e^(-x^2)}
///definition "physicist's Hermite"  Pn(x)= n! * sum {(-1)^k/k!/(n-2k)! *(2x)^n-2m }
///the sum are from 0 to floor n/2 for even and odd integer separation.
pub fn hermite_polynomials(degree:u64)->Vec<f64>{


    let mut coefficients: Vec<f64>= Vec::new();

    for iter in 0..=degree/2{

        let coef:f64=(alt_sign(iter) as f64)*(2.0_f64.powi((degree-2*iter) as i32))/(factorial(iter) as f64)/(factorial(degree-2*iter) as f64);

        coefficients.push(coef*(factorial(degree)as f64));
        coefficients.push(0.0)
    }
    if degree & 1 == 0{
        coefficients.pop();
    }
    coefficients.reverse();
    coefficients

    

}


///Operation (-1)^k for positive integers
///If the number is odd it returns -1 and
///if the number is even it returns 1
pub fn alt_sign(number:u64)->i64{
    

    return ((number as i64 & 1) ^ 1)+(-1*(number as i64 & 1));

}
///A functions that returns a value of a given polynomial at an input x, 
/// when its coefficients are given
pub fn poly_evaluate(coefficients:&Vec<f64>,x:f64)->f64{

    let mut result = 0.0;
    let mut x_power = 1.0;

    for &coeff in coefficients {

        result += coeff * x_power;
        x_power *= x;
    }

    return result
}
