use rand::prelude::*;

pub fn random_double(rng: &mut ThreadRng) -> f64 {
  rng.gen()
}

pub fn random_double_in_range(min: f64, max: f64, rng: &mut ThreadRng) -> f64 {
  min + (max-min) * random_double(rng)
}