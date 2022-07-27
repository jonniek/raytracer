
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Color,Vec3};

pub trait Scatterable {
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)>;
}

#[derive(Debug,Clone)]
pub enum Material {
  Lambertian(Lambertian),
  Metal(Metal),
}

impl Default for Material {
  fn default() -> Self {
    Material::Lambertian(Lambertian{ albedo: Color::default() })
  }
}

impl Scatterable for Material {
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
    match self {
      Material::Lambertian(l) => l.scatter(ray, hit_record),
      Material::Metal(l) => l.scatter(ray, hit_record),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
  pub albedo: Color,
}

impl Scatterable for Lambertian {
  fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
    let mut scatter_direction = hit_record.normal + Vec3::random_in_unit_sphere();
    if scatter_direction.near_zero() {
      scatter_direction = hit_record.normal;
    }
    let target = hit_record.p + scatter_direction;
    let scattered = Ray { origin: hit_record.p, direction: target - hit_record.p };
    let attenuation = self.albedo;
    Some((Some(scattered), attenuation))
  }
}


#[derive(Debug, Clone, Copy)]
pub struct Metal {
  pub albedo: Color,
  pub fuzz: f64,
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  *v - *n * (2.0 * v.dot(n))
}

impl Scatterable for Metal {
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
    let reflected = reflect(&ray.direction, &hit_record.normal);
    let scattered = Ray {
      origin: hit_record.p,
      direction: reflected + Vec3::random_in_unit_sphere() * self.fuzz,
    };
    let attenuation = self.albedo;
    if scattered.direction.dot(&hit_record.normal) > 0.0 {
      Some((Some(scattered), attenuation))
    } else {
      None
    }
  }
}