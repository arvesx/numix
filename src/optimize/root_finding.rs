pub fn bisection() -> f64 {
    return 4.0;
}

pub struct Newton<F> {
    f: F,
    f_prime: Option<F>,
    f_double_prime: Option<F>,
    initial_guess: Vec<f64>,
    tolerance: f64,
    relative_tolerance: f64,
    iterations: usize,
}

impl<F> Newton<F>
where
    F: Fn(f64) -> f64,
{
    pub fn initialize(f: F, initial_guess: Vec<f64>) -> Self {
        Self {
            f,
            f_prime: None,
            f_double_prime: None,
            initial_guess,
            tolerance: 1e-6,
            iterations: 100,
            relative_tolerance: 0.0,
        }
    }

    pub fn f_prime(mut self, f_prime: F) -> Self {
        self.f_prime = Some(f_prime);
        self
    }

    pub fn f_double_prime(mut self, f_double_prime: F) -> Self {
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

    pub fn run(self) -> Vec<f64> {
        self.initial_guess
            .iter()
            .map(|&guess| {
                let mut x = guess;
                let mut iter = 0;

                // If f prime is given we proceed with the Newton-Raphson Method
                if self.f_prime.as_ref().is_some() {
                    let f_prime_unwrapped = self.f_prime.as_ref().unwrap();
                    for i in 1..self.iterations {
                        x = x - (self.f)(x) / f_prime_unwrapped(x);
                        iter += 1;
                    }
                } else {
                    x = 35.0;
                }
                x
            })
            .collect()
    }
}
