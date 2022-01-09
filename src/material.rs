
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Color,Vec3};

#[derive(Debug,Clone,Copy)]
pub struct Material {}

impl Material {
  pub fn scatter(
    ray_in: &Ray,
    hit_record: &HitRecord,
    attenuation: &Color,
    outward_normal: &Vec3,
  ) -> bool {
    false
  }
}