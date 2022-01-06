use rand::prelude::*;

pub fn random_double() -> f64 {
  rand::thread_rng().gen()
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
  min + (max-min) * random_double()
}