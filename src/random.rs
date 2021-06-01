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

    pub fn shuffle<T>(&mut self, v: &mut Vec<T>) {
        let mut i = 0;
        let attempts = v.len() * 2;
        for _ in 0..attempts {
            let random_idx = self.next() as usize % v.len();
            if i != random_idx {
                v.swap(i, random_idx);
            }
            i = (i + 1) % v.len();
        }
    }
}
