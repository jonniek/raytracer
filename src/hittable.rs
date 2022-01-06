use crate::vec3::{Point3,Vec3};
use crate::ray::Ray;
use crate::sphere::Sphere;

#[derive(Debug,Clone,Copy)]
pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


pub struct HitList {
  pub objects: Vec<Sphere>
}

impl Hittable for HitList {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest_hit: Option<HitRecord> = None;

    for hittable in &self.objects {
      match (closest_hit, hittable.hit(ray, t_min, t_max)) {
        (None, Some(hit)) => closest_hit = Some(hit),
        (Some(record), Some(hit)) => {
          if hit.t < record.t {
            closest_hit = Some(hit);
          }
        }
        (_, None) => (),
      }
    }

    closest_hit
  }
}
