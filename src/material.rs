
use crate::random::random_double;
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
  Glass(Glass),
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
      Material::Glass(l) => l.scatter(ray, hit_record),
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


#[derive(Debug, Clone, Copy)]
pub struct Glass {
  pub refraction_index: f64,
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
  let cos_theta = match -uv.dot(n) {
    a if a < 1.0 => a,
    _ => 1.0,
  };
  let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
  let r_out_parallel = *n * ((1.0 - r_out_perp.len_squared()).abs().sqrt() * -1.0);
  return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
  let r01 = (1.0-ref_idx) / (1.0+ref_idx);
  let r0 = r01*r01;
  return r0 + (1.0-r0) * (1.0 - cosine).powf(5.0);
}

impl Scatterable for Glass {
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
    let attenuation = Color::from(1.0, 1.0, 1.0);
    let unit_direction = ray.direction.unit_vector();

    let refraction_ratio = match hit_record.front_face {
      true => 1.0 / self.refraction_index,
      false => self.refraction_index,
    };

    let cos_theta = match (unit_direction * -1.0).dot(&hit_record.normal) {
      a if a < 1.0 => a,
      _ => 1.0,
    };
    let sin_theta = (cos_theta * cos_theta - 1.0).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let schlick_approximation = reflectance(cos_theta, refraction_ratio) > random_double();

    let direction = match cannot_refract || schlick_approximation {
      true => reflect(&unit_direction, &hit_record.normal),
      false => refract(&unit_direction, &hit_record.normal, refraction_ratio),
    };

    let scattered = Ray {
      origin: hit_record.p,
      direction,
    };
    return Some((Some(scattered), attenuation));
  }
}