use core::fmt;

pub enum RootFindingError {
    SignAgreementError,
    NonConvergenceError,
    ZeroDerivativeError,
    IdenticalInitialGuessesError,
}

impl fmt::Display for RootFindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RootFindingError::SignAgreementError => {
                write!(f, "The signs of the initial numbers are the same.")
            }
            RootFindingError::NonConvergenceError => write!(f, "The algorithm failed to converge."),
            RootFindingError::ZeroDerivativeError => {
                write!(f, "Derivative became zero during computation.")
            }
            RootFindingError::IdenticalInitialGuessesError => {
                write!(f, "Initial guesses x0 and x1 cannot be identical!")
            }
        }
    }
}

pub struct Bisection {
    f: fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    rtol: f64,
    iter: usize,
}

impl Bisection {
    pub fn initialize(f: fn(f64) -> f64, a: f64, b: f64) -> Self {
        Self {
            f,
            a,
            b,
            tol: 1e-8,
            iter: 100,
            rtol: 0.0,
        }
    }
    pub fn tol(mut self, tol: f64) -> Self {
        self.tol = tol;
        self
    }
    pub fn rtol(mut self, rtol: f64) -> Self {
        self.rtol = rtol;
        self
    }

    pub fn iter(mut self, iter: usize) -> Self {
        self.iter = iter;
        self
    }

    pub fn run(self) -> Result<f64, RootFindingError> {
        let mut a = self.a;
        let mut b = self.b;
        let mut m;

        let f_a = (self.f)(a);
        let f_b = (self.f)(b);
        let mut f_m;

        if f_a == 0.0 {
            return Ok(a);
        }

        if f_b == 0.0 {
            return Ok(b);
        }

        if f_a.signum() == f_b.signum() {
            return Err(RootFindingError::SignAgreementError);
        }

        for _i in 0..self.iter {
            m = a + (b - a) * 0.5;
            f_m = (self.f)(m);
            if f_m.signum() == f_a.signum() {
                a = m;
            } else {
                b = m;
            }

            if self.convergence_achieved(&a, &b, &m) {
                return Ok(m);
            }
        }

        Err(RootFindingError::NonConvergenceError)
    }

    fn convergence_achieved(&self, a: &f64, b: &f64, m: &f64) -> bool {
        (a - b).abs() < self.tol + self.rtol * m
    }
}

pub struct Newton {
    f: fn(f64) -> f64,
    fp: Option<fn(f64) -> f64>,
    fdp: Option<fn(f64) -> f64>,
    x0: f64,
    x1: Option<f64>,
    tol: f64,
    rtol: f64,
    iter: usize,
}

impl Newton {
    pub fn initialize(f: fn(f64) -> f64, x0: f64) -> Self {
        Self {
            f,
            fp: None,
            fdp: None,
            x0,
            x1: None,
            tol: 1e-8,
            iter: 100,
            rtol: 0.0,
        }
    }

    pub fn x1(mut self, x1: f64) -> Self {
        self.x1 = Some(x1);
        self
    }

    pub fn fp(mut self, fp: fn(f64) -> f64) -> Self {
        self.fp = Some(fp);
        self
    }

    pub fn fdp(mut self, fdp: fn(f64) -> f64) -> Self {
        self.fdp = Some(fdp);
        self
    }

    pub fn tol(mut self, tol: f64) -> Self {
        self.tol = tol;
        self
    }

    pub fn rtol(mut self, rtol: f64) -> Self {
        self.rtol = rtol;
        self
    }

    pub fn iter(mut self, iter: usize) -> Self {
        self.iter = iter;
        self
    }

    pub fn run(self) -> Result<f64, RootFindingError> {
        let mut x = self.x0;

        match &self.fp {
            // If f prime is given, proceed with the Newton-Raphson Method
            Some(f_prime) => {
                let mut x_n;
                let mut f_x = (self.f)(x);
                let mut f_prime_x = f_prime(x);
                let mut newton_step;

                for _i in 0..self.iter {
                    // If root has been found, terminate
                    if f_x == 0.0 {
                        return Ok(x);
                    }

                    if f_prime_x == 0.0 {
                        return Err(RootFindingError::ZeroDerivativeError);
                    }

                    newton_step = f_x / f_prime_x;

                    match &self.fdp {
                        // If f double prime is given, use Halley's Method
                        Some(f_double_prime) => {
                            let f_d_prime_x = f_double_prime(x);
                            let adjustment = newton_step * f_d_prime_x / f_prime_x / 2.0;
                            if adjustment.abs() < 1.0 {
                                newton_step /= 1.0 - adjustment;
                            }
                        }
                        None => {}
                    }

                    x_n = x - newton_step;

                    // Check for convergence
                    if Self::convergence_achieved(&self, &x, &x_n) {
                        return Ok(x);
                    }

                    // Update variables
                    x = x_n;
                    f_x = (self.f)(x);
                    f_prime_x = f_prime(x);
                }
                Ok(x)
            }
            // In case f prime is not given, proceed with Secant Method
            None => {
                let mut p0 = self.x0;
                let mut p1;

                match self.x1 {
                    Some(x1) => {
                        if x1 == self.x0 {
                            return Err(RootFindingError::IdenticalInitialGuessesError);
                        }
                        p1 = x1;
                    }
                    None => {
                        let delta = 1e-4;
                        p1 = p0 * (1.0 + delta);
                        p1 += if p1 >= 0.0 { delta } else { -delta }
                    }
                }

                let mut f_p0 = (self.f)(p0);
                let mut f_p1 = (self.f)(p1);
                if f_p1.abs() < f_p0.abs() {
                    std::mem::swap(&mut p0, &mut p1);
                    std::mem::swap(&mut f_p0, &mut f_p1);
                }
                let mut p = p0;
                for _i in 0..self.iter {
                    // If function values are not the same, we have not converged yet
                    if f_p0 != f_p1 {
                        if f_p1.abs() > f_p0.abs() {
                            p = (-f_p0 / f_p1 * p1 + p0) / (1.0 - f_p0 / f_p1);
                        } else {
                            p = (-f_p1 / f_p0 * p0 + p1) / (1.0 - f_p1 / f_p0);
                        }
                    } else {
                        // If function values are the same, Secant cannot continue because denominator is zero
                        return Err(RootFindingError::NonConvergenceError);
                    }
                    // Check for convergence
                    if self.convergence_achieved(&p, &p1) {
                        return Ok(p);
                    }
                    p0 = p1;
                    f_p0 = f_p1;
                    p1 = p;
                    f_p1 = (self.f)(p1)
                }

                Ok(p)
            }
        }
    }

    fn convergence_achieved(&self, x: &f64, x_n: &f64) -> bool {
        // If |x - x_n| < tolerance, convergence achieved
        if (x - x_n).abs() < self.tol {
            return true;
        }

        // If |x - x_n| / |max(x, x_n, 1)| < relative_tolerance, convergence achieved
        if (x - x_n).abs() / x.abs().max(x_n.abs()).max(1.0) < self.rtol {
            return true;
        }

        false
    }
}
