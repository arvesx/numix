use core::fmt;

static MACH_EPS: f64 = f64::EPSILON;
static DEFAULT_RTOL: f64 = 4.0 * MACH_EPS;

static SUCCESS_CONVERGENCE: &str =
    "The algorithm achieved convergence with the specified tolerance.\n";
static MAX_ITER: &str = "Variable est_x is the last approximation made by the algorithm.\n";

pub struct AlgoMetrics {
    pub msg: String,
    pub func_evals: u32,
    pub iter: usize,
    pub est_x: f64,
}

impl fmt::Display for AlgoMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}func_evals={}, iter={}, est_x={}",
            self.msg, self.func_evals, self.iter, self.est_x
        )
    }
}

pub enum RootFindingError {
    SignAgreementError,
    NonConvergenceError(AlgoMetrics),
    ZeroDerivativeError(AlgoMetrics),
    IdenticalInitialGuessesError,
    UnacceptableToleranceError(AlgoMetrics),
    IterationLimitExceededError(AlgoMetrics),
}

impl fmt::Display for RootFindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RootFindingError::SignAgreementError => {
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
            RootFindingError::IdenticalInitialGuessesError => {
                write!(f, "Initial guesses x0 and x1 cannot be identical.")
            }
            RootFindingError::UnacceptableToleranceError(algo_metrics) => {
                write!(
                    f,
                    "Too small tolerance value was given.\n{}",
                    algo_metrics.msg
                )
            }
            RootFindingError::IterationLimitExceededError(algo_metrics) => {
                write!(f, "Maximum number of iterations reached.\n{}", algo_metrics)
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
            rtol: DEFAULT_RTOL,
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
            est_x: f64::NAN,
            msg: String::from(""),
            func_evals: 0,
            iter: 0,
        };

        if self.tol <= 0.0 {
            algo_metrics
                .msg
                .push_str("Value of tol is either negative or zero.");
            return Err(RootFindingError::UnacceptableToleranceError(algo_metrics));
        }

        if self.rtol < DEFAULT_RTOL {
            algo_metrics
                .msg
                .push_str("Value of rtol is either negative or extremely small.");
            return Err(RootFindingError::UnacceptableToleranceError(algo_metrics));
        }

        let mut a = self.a;
        let mut b = self.b;
        let mut m = a + (b - a) * 0.5;

        let f_a = (self.f)(a);
        algo_metrics.func_evals += 1;
        let f_b = (self.f)(b);
        algo_metrics.func_evals += 1;
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
            return Err(RootFindingError::SignAgreementError);
        }

        for i in 0..self.iter {
            m = a + (b - a) * 0.5;
            f_m = (self.f)(m);
            algo_metrics.func_evals += 1;
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
        algo_metrics.est_x = m;
        algo_metrics.msg.push_str(MAX_ITER);
        Err(RootFindingError::IterationLimitExceededError(algo_metrics))
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
            rtol: DEFAULT_RTOL,
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
            est_x: f64::NAN,
            msg: String::from(""),
            func_evals: 0,
            iter: 0,
        };

        if self.tol <= 0.0 {
            algo_metrics
                .msg
                .push_str("Value of tol is either negative or zero.");
            return Err(RootFindingError::UnacceptableToleranceError(algo_metrics));
        }

        if self.rtol < DEFAULT_RTOL {
            algo_metrics
                .msg
                .push_str("Value of rtol is either negative or extremely small.");
            return Err(RootFindingError::UnacceptableToleranceError(algo_metrics));
        }

        let mut x = self.x0;

        match &self.fp {
            // If f prime is given, proceed with the Newton-Raphson Method
            Some(f_prime) => {
                let mut x_n;
                let mut f_x = (self.f)(x);
                algo_metrics.func_evals += 1;
                let mut f_prime_x = f_prime(x);
                algo_metrics.func_evals += 1;
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
                            algo_metrics.func_evals += 1;
                            let adjustment = newton_step * f_d_prime_x / f_prime_x / 2.0;
                            if adjustment.abs() < 1.0 {
                                newton_step /= 1.0 - adjustment;
                            }
                        }
                        None => {}
                    }

                    x_n = x - newton_step;

                    // Check for convergence
                    if precision_equals(x, x_n, self.tol, self.rtol) {
                        algo_metrics.est_x = x;
                        algo_metrics.iter = i;
                        algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
                        return Ok(algo_metrics);
                    }

                    // Update variables
                    x = x_n;
                    f_x = (self.f)(x);
                    algo_metrics.func_evals += 1;
                    f_prime_x = f_prime(x);
                    algo_metrics.func_evals += 1;
                }
                algo_metrics.est_x = x;
                algo_metrics.iter = self.iter;
                algo_metrics.msg.push_str(MAX_ITER);
                Err(RootFindingError::IterationLimitExceededError(algo_metrics))
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
                algo_metrics.func_evals += 1;
                let mut f_p1 = (self.f)(p1);
                algo_metrics.func_evals += 1;
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
                    if precision_equals(p, p1, self.tol, self.rtol) {
                        algo_metrics.iter = i;
                        algo_metrics.est_x = p;
                        algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
                        return Ok(algo_metrics);
                    }
                    p0 = p1;
                    f_p0 = f_p1;
                    p1 = p;
                    f_p1 = (self.f)(p1);
                    algo_metrics.func_evals += 1;
                }

                algo_metrics.est_x = p;
                algo_metrics.msg.push_str(MAX_ITER);
                algo_metrics.iter = self.iter;
                Err(RootFindingError::IterationLimitExceededError(algo_metrics))
            }
        }
    }
}

pub fn precision_equals(x1: f64, x2: f64, tol: f64, rtol: f64) -> bool {
    (x1 - x2).abs() <= tol + rtol * x2.abs()
}

pub struct Ridders {
    f: fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    rtol: f64,
    iter: usize,
}

impl Ridders {
    pub fn initialize(f: fn(f64) -> f64, a: f64, b: f64) -> Self {
        Self {
            f,
            a,
            b,
            tol: 1e-8,
            iter: 100,
            rtol: DEFAULT_RTOL,
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
        // Initialize metrics for the algorithm
        let mut algo_metrics = AlgoMetrics {
            est_x: f64::NAN,
            msg: String::from(""),
            func_evals: 0,
            iter: 0,
        };

        // Check for acceptable tolerances
        if self.tol <= 0.0 {
            algo_metrics
                .msg
                .push_str("Value of tol is either negative or zero.");
            return Err(RootFindingError::UnacceptableToleranceError(algo_metrics));
        }

        if self.rtol < DEFAULT_RTOL {
            algo_metrics
                .msg
                .push_str("Value of rtol is either negative or extremely small.");
            return Err(RootFindingError::UnacceptableToleranceError(algo_metrics));
        }

        let mut a = self.a;
        let mut b = self.b;
        let mut m;
        let mut x_prev = f64::MAX; // To track previous x value

        let mut f_a = (self.f)(a);
        algo_metrics.func_evals += 1;
        let mut f_b = (self.f)(b);
        algo_metrics.func_evals += 1;
        let mut f_m;

        // Check if either boundary is a root
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

        // Ensure f(a) and f(b) have different signs
        if f_a.signum() == f_b.signum() {
            return Err(RootFindingError::SignAgreementError);
        }

        // Main iteration loop
        for i in 0..self.iter {
            m = 0.5 * (a + b); // Update midpoint
            f_m = (self.f)(m);
            algo_metrics.func_evals += 1;

            // Calculate 's' for Ridders' formula
            let s = f64::sqrt(f_m.powi(2) - f_a * f_b);
            if s == 0.0 {
                // Denominator became zero, non-convergence
                algo_metrics.msg.push_str("Cannot apply Ridders' step because denominator became zero during computation.");
                algo_metrics.iter = i;
                return Err(RootFindingError::NonConvergenceError(algo_metrics));
            }
            // Calculate dx and x using Ridders' formula
            let mut dx = (m - a) * f_m / s;
            if (f_a - f_b) < 0.0 {
                dx = -dx;
            }
            let x = m + dx;
            let f_x = (self.f)(x);
            algo_metrics.func_evals += 1;

            // Check for convergence
            if precision_equals(x, x_prev, self.tol, self.rtol) {
                algo_metrics.iter = i;
                algo_metrics.est_x = x;
                algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
                return Ok(algo_metrics);
            }
            x_prev = x;

            // Update a, b, f_a, f_b based on the new evaluations
            if f_m.signum() == f_x.signum() {
                if f_a.signum() == f_x.signum() {
                    a = x;
                    f_a = f_x;
                } else {
                    b = x;
                    f_b = f_x;
                }
            } else {
                a = m;
                b = x;
                f_a = f_m;
                f_b = f_x;
            }
        }

        // If reached here, max iterations hit without finding a root
        algo_metrics.iter = self.iter;
        algo_metrics.est_x = x_prev;
        algo_metrics.msg.push_str(MAX_ITER);
        Err(RootFindingError::IterationLimitExceededError(algo_metrics))
    }
}

pub struct Brent {
    f: fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    rtol: f64,
    iter: usize,
}

impl Brent {
    pub fn initialize(f: fn(f64) -> f64, a: f64, b: f64) -> Self {
        Self {
            f,
            a,
            b,
            tol: 1e-8,
            iter: 100,
            rtol: DEFAULT_RTOL,
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
            est_x: f64::NAN,
            msg: String::from(""),
            func_evals: 0,
            iter: 0,
        };

        if self.tol <= 0.0 {
            algo_metrics
                .msg
                .push_str("Value of tol is either negative or zero.");
            return Err(RootFindingError::UnacceptableToleranceError(algo_metrics));
        }

        if self.rtol < DEFAULT_RTOL {
            algo_metrics
                .msg
                .push_str("Value of rtol is either negative or extremely small.");
            return Err(RootFindingError::UnacceptableToleranceError(algo_metrics));
        }

        let mut a = self.a;
        let mut b = self.b;
        let mut f_a = (self.f)(a);
        algo_metrics.func_evals += 1;
        let mut f_b = (self.f)(b);
        algo_metrics.func_evals += 1;

        if precision_equals(f_a, 0.0, self.tol, self.rtol) {
            algo_metrics.est_x = a;
            algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
            return Ok(algo_metrics);
        }

        if precision_equals(f_b, 0.0, self.tol, self.rtol) {
            algo_metrics.est_x = b;
            algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
            return Ok(algo_metrics);
        }

        let mut last_bracket = a;
        let mut f_last_bracket = f_a;
        let mut last_interval_size = b - a;
        let mut prev_interval_size = last_interval_size;
        let mut effective_tol;
        let mut m;
        let mut s;
        let mut p;
        let mut q;
        let mut r;

        for i in 0..self.iter {
            // If the absolute value of f_last_bracket is less than the absolute value of f_b, swap a, b,
            // and last_bracket as well as their corresponding function values. This ensures that b is
            // always the best approximation
            if f_last_bracket.abs() < f_b.abs() {
                a = b;
                b = last_bracket;
                last_bracket = a;
                f_a = f_b;
                f_b = f_last_bracket;
                f_last_bracket = f_a
            }

            // Calculate effective tolerance and midpoint
            effective_tol = self.tol + 2.0 * self.rtol * b.abs();
            m = 0.5 * (last_bracket - b);

            // If the absolute value of the midpoint is less than or equal to the effective tolerance,
            // or if f_b is zero, then a root has been found. Return b.
            if m.abs() <= effective_tol || precision_equals(f_b, 0.0, self.tol, self.rtol) {
                algo_metrics.est_x = b;
                algo_metrics.iter = i;
                algo_metrics.msg.push_str(SUCCESS_CONVERGENCE);
                return Ok(algo_metrics);
            }

            if prev_interval_size.abs() < effective_tol || f_a.abs() < f_b.abs() {
                // In that case we use bisection
                last_interval_size = m;
                prev_interval_size = last_interval_size;
            } else {
                // Else, decide which interpolation method to use
                s = f_b / f_a;
                if a == last_bracket {
                    // Do linear interpolation
                    p = 2.0 * m * s;
                    q = 1.0 - s;
                } else {
                    // Do inverse quadratic interpolation
                    q = f_a / f_last_bracket;
                    r = f_b / f_last_bracket;
                    p = s * (2.0 * m * q * (q - r) - (b - a) * (r - 1.0));
                    q = (q - 1.0) * (r - 1.0) * (s - 1.0);
                }

                if p > 0.0 {
                    q = -q;
                } else {
                    p = -p;
                }

                s = prev_interval_size;
                prev_interval_size = last_interval_size;
                // We evaluate whether the interpolation is likely to be beneficial. If the calculated p is
                // too large compared to the midpoint and the effective tolerance, or if it's larger than half
                // of the previous interval size multiplied by q, we decide that interpolation isn't helping us much.
                if (p >= 1.5 * m * q - (effective_tol * q).abs()) || (p >= (0.5 * s * q).abs()) {
                    last_interval_size = m;
                    prev_interval_size = last_interval_size;
                } else {
                    last_interval_size = p / q;
                }
            }
            a = b;
            f_a = f_b;
            if last_interval_size.abs() > effective_tol {
                b += last_interval_size;
            } else if m > 0.0 {
                b += effective_tol;
            } else {
                b -= effective_tol;
            }

            f_b = (self.f)(b);
            algo_metrics.func_evals += 1;

            if (f_b > 0.0 && f_last_bracket > 0.0) || (f_b <= 0.0 && f_last_bracket <= 0.0) {
                last_bracket = a;
                f_last_bracket = f_a;
                last_interval_size = b - a;
                prev_interval_size = last_interval_size;
            }
        }

        Err(RootFindingError::IterationLimitExceededError(algo_metrics))
    }
}
