// https://en.wikipedia.org/wiki/Linear_congruential_generator

// These parameters are from Numerical Recipes.
const MODULUS: u64 = 2u64.pow(32);
const MULTIPLIER: u64 = 1664525;
const INCREMENT: u64 = 1013904223;

pub struct Random {
    pub seed: u64,
}

impl Random {
    pub fn next(&mut self) -> u64 {
        self.seed = (MULTIPLIER * self.seed + INCREMENT) % MODULUS;
        self.seed
    }

    pub fn next_float(&mut self) -> f64 {
        self.next() as f64 / MODULUS as f64
    }
}
