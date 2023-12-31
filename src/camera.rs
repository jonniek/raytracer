use crate::vec3::{Point3,Vec3};
use crate::ray::Ray;
use std::f64::consts::PI;

fn degrees_to_radians(deg: f64) -> f64 {
  deg * PI / 180.0
}

pub struct Camera {
  pub aspect_ratio: f64,
  pub viewport_height: f64,
  pub viewport_width: f64,
  pub focal_length: f64,
  
  origin: Point3,
  horizontal: Vec3,
  vertical: Vec3,
  lower_left_corner: Point3,
  lens_radius: f64,
  u: Vec3,
}

impl Camera {
  pub fn new(
    aspect_ratio: f64,
    vfov: f64,
    aperture: f64,
    focus_distance: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
  ) -> Camera {
    let theta = degrees_to_radians(vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let w = (lookfrom - lookat).unit_vector();
    let u = vup.cross(w).unit_vector();
    let v = w.cross(u);

    let origin = lookfrom;
    let horizontal = u * viewport_width * focus_distance;
    let vertical = v * viewport_height * focus_distance;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;
    let lens_radius = aperture / 2.0;

    Camera {
      aspect_ratio,
      viewport_height,
      viewport_width,
      focal_length,
      origin,
      horizontal,
      vertical,
      lower_left_corner,
      lens_radius,
      u,
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let rd = Vec3::random_in_unit_disk() * self.lens_radius;
    let offset = self.u * rd.x * rd.y;

    Ray {
      origin: self.origin + offset,
      direction:
        self.lower_left_corner +
        self.horizontal * s +
        self.vertical * t - self.origin - offset
    }
  }
}