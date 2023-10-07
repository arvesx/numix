pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod optimize {
    pub mod root_finding;
}

pub mod interpolate {
    pub mod polynomial;
}
pub mod integrate {
    pub mod integrator;
    pub mod quad;
}

pub mod arithmetic {
    pub mod binomial;
}

pub mod special {
    pub mod polynomials;
}

pub mod common {
    pub mod functions;
}
