use crate::vec3::{Point3,Vec3};
use crate::ray::Ray;

pub struct Camera {
  pub aspect_ratio: f64,
  pub viewport_height: f64,
  pub viewport_width: f64,
  pub focal_length: f64,

  origin: Point3,
  horizontal: Vec3,
  vertical: Vec3,
  lower_left_corner: Point3,
}

impl Camera {
  pub fn new(aspect_ratio: f64) -> Camera {
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);

    Camera {
      aspect_ratio,
      viewport_height,
      viewport_width,
      focal_length: 1.0,

      origin,
      horizontal: Vec3::from(viewport_width, 0.0, 0.0),
      vertical: Vec3::from(0.0, viewport_height, 0.0),
      lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length),
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    Ray {
      origin: self.origin,
      direction: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
    }
  }
}