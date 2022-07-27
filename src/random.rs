use rand::prelude::*;

pub fn random_double() -> f64 {
  let mut rng = rand::thread_rng();

  rng.gen()
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
  let mut rng = rand::thread_rng();

  min + (max-min) * rng.gen::<f64>()
}