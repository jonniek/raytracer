use crate::ray::Ray;
use crate::vec3::{Point3};
use crate::hittable::{Hittable,HitRecord};

#[derive(Default,Debug,Clone,Copy)]
pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin - self.center;
    let a = r.direction.len_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.len_squared() - self.radius*self.radius;

    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
      return None;
    }
    let sqrtd = discriminant.sqrt();

    // Find the nearest root that lies in the acceptable range.
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
        root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
          return None
        }
    }

    let t = root;
    let p = r.at(t);
    let outward_normal = (p - self.center) / self.radius;
    let front_face = r.direction.dot(outward_normal) < 0.0;

    Some(HitRecord {
      t,
      p,
      front_face,
      normal: match front_face {
        true => outward_normal,
        false => outward_normal * -1.0,
      },
    })
  }
}


#[test]
fn test_sphere_hit() {
  let result = Sphere::default().hit(&Ray::default(), 0.0, 2.0);
  assert_eq!(result.is_none(), true);
}