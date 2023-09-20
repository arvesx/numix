pub struct Bisection {
    f: fn(f64) -> f64,
    a: f64,
    b: f64,
    tolerance: f64,
    relative_tolerance: f64,
    iterations: usize,
}

impl Bisection {
    pub fn initialize(f: fn(f64) -> f64, a: f64, b: f64) -> Self {
        Self {
            f,
            a,
            b,
            tolerance: 1e-8,
            iterations: 100,
            relative_tolerance: 0.0,
        }
    }
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }
    pub fn relative_tolerance(mut self, relative_tolerance: f64) -> Self {
        self.relative_tolerance = relative_tolerance;
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn run(self) -> f64 {
        let mut a = self.a;
        let mut b = self.b;
        let mut m;

        let f_a = (self.f)(a);
        let f_b = (self.f)(b);
        let mut f_m;

        if f_a == 0.0 {
            return a;
        }

        if f_b == 0.0 {
            return b;
        }

        if f_a.signum() == f_b.signum() {
            // will need to return error
            return 0.0;
        }

        for _i in 1..self.iterations {
            m = a + (b - a) * 0.5;
            f_m = (self.f)(m);
            if f_m.signum() == f_a.signum() {
                a = m;
            } else {
                b = m;
            }

            if self.convergence_achieved(&a, &b, &m) {
                return m;
            }
        }

        // will need to return error
        0.0
    }

    fn convergence_achieved(&self, a: &f64, b: &f64, m: &f64) -> bool {
        (a - b).abs() < self.tolerance + self.relative_tolerance * m
    }
}

pub struct Newton {
    f: fn(f64) -> f64,
    f_prime: Option<fn(f64) -> f64>,
    f_double_prime: Option<fn(f64) -> f64>,
    initial_guess: f64,
    tolerance: f64,
    relative_tolerance: f64,
    iterations: usize,
}

impl Newton {
    pub fn initialize(f: fn(f64) -> f64, initial_guess: f64) -> Self {
        Self {
            f,
            f_prime: None,
            f_double_prime: None,
            initial_guess,
            tolerance: 1e-8,
            iterations: 100,
            relative_tolerance: 0.0,
        }
    }

    pub fn f_prime(mut self, f_prime: fn(f64) -> f64) -> Self {
        self.f_prime = Some(f_prime);
        self
    }

    pub fn f_double_prime(mut self, f_double_prime: fn(f64) -> f64) -> Self {
        self.f_double_prime = Some(f_double_prime);
        self
    }

    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn relative_tolerance(mut self, relative_tolerance: f64) -> Self {
        self.relative_tolerance = relative_tolerance;
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn run(self) -> f64 {
        let mut x = self.initial_guess;

        match &self.f_prime {
            // If f prime is given, proceed with the Newton-Raphson Method
            Some(f_prime) => {
                let mut x_n;
                let mut f_x = (self.f)(x);
                let mut f_prime_x = f_prime(x);
                let mut newton_step;

                for _i in 1..self.iterations {
                    // If root has been found, terminate
                    if f_x == 0.0 {
                        return x;
                    }

                    if f_prime_x == 0.0 {
                        return x;
                    }

                    newton_step = f_x / f_prime_x;
                    x_n = x - newton_step;

                    // Check for convergence
                    if Self::convergence_achieved(&self, &x, &x_n) {
                        return x;
                    }

                    // Update variables
                    x = x_n;
                    f_x = (self.f)(x);
                    f_prime_x = f_prime(x);
                }
                return x;
            }
            // In case f prime is not given, proceed with Secant Method
            None => {}
        }

        x
    }

    fn convergence_achieved(&self, x: &f64, x_n: &f64) -> bool {
        // If |x - x_n| < tolerance, convergence achieved
        if (x - x_n).abs() < self.tolerance {
            return true;
        }

        // If |x - x_n| / |max(x, x_n, 1)| < relative_tolerance, convergence achieved
        if (x - x_n).abs() / x.abs().max(x_n.abs()).max(1.0) < self.relative_tolerance {
            return true;
        }

        false
    }
}
