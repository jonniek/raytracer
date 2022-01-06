mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod random;

use std::io::{self, Write};
use vec3::{Vec3,Color,Point3};
use ray::Ray;
use std::f64::INFINITY;
use std::f64::consts::PI;
use hittable::{Hittable,HitList};
use sphere::Sphere;
use camera::Camera;
use random::random_double;
use rand::prelude::*;
use rayon::prelude::*;


fn degrees_to_radians(deg: f64) -> f64 {
  deg * PI / 180.0
}

fn clamp(value: f64, min_value: f64, max_value: f64) -> f64 {
  if value < min_value {
    return min_value;
  }
  if value > max_value {
    return max_value;
  }
  value
}

fn ray_color(ray: &Ray, hit_list: &HitList, depth: isize, rng: &mut ThreadRng) -> Color {
  if depth <= 0 {
    return Color::from(0.0, 0.0, 0.0);
  }

  match hit_list.hit(ray, 0.001, INFINITY) {
    Some(hit) => {
      let target = hit.normal + Vec3::random_in_hemisphere(&hit.normal, rng);
      let new_ray = Ray { origin: hit.p, direction: target - hit.p };
      ray_color(&new_ray, &hit_list, depth - 1, rng) * 0.5
    },
    None => {
      let unit_dir = ray.direction.unit_vector();
      let t = 0.5 * (unit_dir.y + 1.0);
      Color::from(1.0, 1.0, 1.0) * (1.0 - t)  + Color::from(0.5, 0.7, 1.0) * t
    }
  }
}

fn write_color(stdout: &std::io::Stdout, color: Color, samples_per_pixel: usize) -> io::Result<()> {
  let mut handle = stdout.lock();

  let scale = 1.0 / samples_per_pixel as f64;

  let r = (color.x * scale).sqrt();
  let g = (color.y * scale).sqrt();
  let b = (color.z * scale).sqrt();

  let ir: usize = (256.0 * clamp(r, 0.0, 0.999)).floor() as usize;
  let ig: usize = (256.0 * clamp(g, 0.0, 0.999)).floor() as usize;
  let ib: usize = (256.0 * clamp(b, 0.0, 0.999)).floor() as usize;

  handle.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;

  Ok(())
}

fn main() -> io::Result<()> {
  let mut rng = rand::thread_rng();

  // image
  let aspect_ratio: f64 = 16.0 / 9.0;
  let width: usize = 400;
  let height: usize = (width as f64 / aspect_ratio) as usize;
  let samples_per_pixel = 100;
  let max_depth = 50;

  // Camera
  let camera = Camera::new(aspect_ratio);

  // render
  let stdout = io::stdout();
  let stderr = io::stderr();
  let mut stderr_handle = stderr.lock();

  {
    let mut handle = stdout.lock();
    let headers = format!("P3\n{} {}\n255\n", width, height);
    handle.write(headers.as_bytes())?;
  }

  let hit_list = HitList {
    objects: vec!(
      Sphere {
        radius: 0.5,
        center: Point3::from(0.0, 0.0, -1.0)
      },
      Sphere {
        radius: 100.0,
        center: Point3::from(0.0,-100.5,-1.0)
      }
    )
  };


  for j in (0..height).rev() {

    stderr_handle.write(format!("Scanlines remaining: {}\n", j).as_bytes())?;
    stderr_handle.flush()?;

    for i in 0..width {

      let mut pixel_color = Color::from(0.0, 0.0, 0.0);

      for _ in 0..samples_per_pixel {
        let u = (i as f64 + random_double(&mut rng)) / (width-1) as f64;
        let v = (j as f64 + random_double(&mut rng)) / (height-1) as f64;
        let ray = camera.get_ray(u, v);
        pixel_color = pixel_color + ray_color(&ray, &hit_list, max_depth, &mut rng);
      }

      write_color(&stdout, pixel_color, samples_per_pixel)?;
    }
  }

  Ok(())
}
