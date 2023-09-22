use core::fmt;

static SUCCESS_MAX_ITER: &str = "Warning! Maximum number of iterations reached.\n";
static SUCCESS_CONVERGENCE: &str = "Achieved convergence with the specified tolerance.\n";

pub struct AlgoMetrics {
    pub msg: String,
    pub funcalls: u32,
    pub iter: usize,
    pub est_x: f64,
}

impl fmt::Display for AlgoMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}funcalls={}, iter={}, est_x={}",
            self.msg, self.funcalls, self.iter, self.est_x
        )
    }
}

pub enum RootFindingError {
    SignAgreementError(AlgoMetrics),
    NonConvergenceError(AlgoMetrics),
    ZeroDerivativeError(AlgoMetrics),
    IdenticalInitialGuessesError(AlgoMetrics),
}

impl fmt::Display for RootFindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RootFindingError::SignAgreementError(_algo_metrics) => {
                write!(f, "The signs of the initial numbers are the same.")
            }
            RootFindingError::NonConvergenceError(algo_metrics) => {
                write!(f, "The algorithm failed to converge.\n{}", algo_metrics)
            }
            RootFindingError::ZeroDerivativeError(algo_metrics) => {
                write!(
                    f,
                    "Derivative became zero during computation.\n{}",
                    algo_metrics
                )
            }
            RootFindingError::IdenticalInitialGuessesError(_algo_metrics) => {
                write!(f, "Initial guesses x0 and x1 cannot be identical.")
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

    pub fn run(self) -> Result<AlgoMetrics, RootFindingError> {
        let mut algo_metrics = AlgoMetrics {
            est_x: 0.0,
            msg: String::from(""),
            funcalls: 0,
            iter: 0,
        };

        let mut a = self.a;
        let mut b = self.b;
        let mut m;

        let f_a = (self.f)(a);
        algo_metrics.funcalls += 1;
        let f_b = (self.f)(b);
        algo_metrics.funcalls += 1;
        let mut f_m;

        if f_a == 0.0 {
            algo_metrics.est_x = a;
            algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
            return Ok(algo_metrics);
        }

        if f_b == 0.0 {
            algo_metrics.est_x = b;
            algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
            return Ok(algo_metrics);
        }

        if f_a.signum() == f_b.signum() {
            return Err(RootFindingError::SignAgreementError(algo_metrics));
        }

        for i in 0..self.iter {
            m = a + (b - a) * 0.5;
            f_m = (self.f)(m);
            algo_metrics.funcalls += 1;
            if f_m.signum() == f_a.signum() {
                a = m;
            } else {
                b = m;
            }

            if self.convergence_achieved(&a, &b, &m) {
                algo_metrics.iter = i;
                algo_metrics.est_x = m;
                algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
                return Ok(algo_metrics);
            }
        }
        algo_metrics.iter = self.iter;
        Err(RootFindingError::NonConvergenceError(algo_metrics))
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

    pub fn run(self) -> Result<AlgoMetrics, RootFindingError> {
        let mut algo_metrics = AlgoMetrics {
            est_x: 0.0,
            msg: String::from(""),
            funcalls: 0,
            iter: 0,
        };
        let mut x = self.x0;

        match &self.fp {
            // If f prime is given, proceed with the Newton-Raphson Method
            Some(f_prime) => {
                let mut x_n;
                let mut f_x = (self.f)(x);
                algo_metrics.funcalls += 1;
                let mut f_prime_x = f_prime(x);
                algo_metrics.funcalls += 1;
                let mut newton_step;

                for i in 0..self.iter {
                    // If root has been found, terminate
                    if f_x == 0.0 {
                        algo_metrics.iter = i;
                        algo_metrics.est_x = x;
                        algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
                        return Ok(algo_metrics);
                    }

                    if f_prime_x == 0.0 {
                        algo_metrics.iter = i;
                        algo_metrics.est_x = x;
                        return Err(RootFindingError::ZeroDerivativeError(algo_metrics));
                    }

                    newton_step = f_x / f_prime_x;

                    match &self.fdp {
                        // If f double prime is given, use Halley's Method
                        Some(f_double_prime) => {
                            let f_d_prime_x = f_double_prime(x);
                            algo_metrics.funcalls += 1;
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
                        algo_metrics.est_x = x;
                        algo_metrics.iter = i;
                        algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
                        return Ok(algo_metrics);
                    }

                    // Update variables
                    x = x_n;
                    f_x = (self.f)(x);
                    algo_metrics.funcalls += 1;
                    f_prime_x = f_prime(x);
                    algo_metrics.funcalls += 1;
                }
                algo_metrics.est_x = x;
                algo_metrics.iter = self.iter;
                algo_metrics.msg.push_str(SUCCESS_MAX_ITER);
                Ok(algo_metrics)
            }
            // In case f prime is not given, proceed with Secant Method
            None => {
                let mut p0 = self.x0;
                let mut p1;

                match self.x1 {
                    Some(x1) => {
                        if x1 == self.x0 {
                            return Err(RootFindingError::IdenticalInitialGuessesError(
                                algo_metrics,
                            ));
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
                algo_metrics.funcalls += 1;
                let mut f_p1 = (self.f)(p1);
                algo_metrics.funcalls += 1;
                if f_p1.abs() < f_p0.abs() {
                    std::mem::swap(&mut p0, &mut p1);
                    std::mem::swap(&mut f_p0, &mut f_p1);
                }
                let mut p = p0;
                for i in 0..self.iter {
                    // If function values are not the same, we have not converged yet
                    if f_p0 != f_p1 {
                        if f_p1.abs() > f_p0.abs() {
                            p = (-f_p0 / f_p1 * p1 + p0) / (1.0 - f_p0 / f_p1);
                        } else {
                            p = (-f_p1 / f_p0 * p0 + p1) / (1.0 - f_p1 / f_p0);
                        }
                    } else {
                        // If function values are the same, Secant cannot continue because denominator is zero
                        algo_metrics.msg.push_str("Cannot apply secant step because denominator became zero during computation.");
                        return Err(RootFindingError::NonConvergenceError(algo_metrics));
                    }
                    // Check for convergence
                    if self.convergence_achieved(&p, &p1) {
                        algo_metrics.iter = i;
                        algo_metrics.est_x = p;
                        algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
                        return Ok(algo_metrics);
                    }
                    p0 = p1;
                    f_p0 = f_p1;
                    p1 = p;
                    f_p1 = (self.f)(p1);
                    algo_metrics.funcalls += 1;
                }

                algo_metrics.est_x = p;
                algo_metrics.msg.push_str(SUCCESS_MAX_ITER);
                algo_metrics.iter = self.iter;
                Ok(algo_metrics)
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
