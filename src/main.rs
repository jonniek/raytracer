mod vec3;
mod ray;
mod hittable;
mod sphere;

use std::io::{self, Write};
use vec3::{Vec3,Color,Point3};
use ray::Ray;
use std::f64::INFINITY;
use std::f64::consts::PI;
use hittable::{Hittable,HitList};
use sphere::Sphere;


fn degrees_to_radians(deg: f64) -> f64 {
  deg * PI / 180.0
}

fn ray_color(ray: &Ray, hit_list: &HitList) -> Color {
  match hit_list.hit(ray, 0.0, INFINITY) {
    Some(hit) => {
      (hit.normal + 1.0) * 0.5
    },
    None => {
      let unit_dir = ray.direction.unit_vector();
      let t = 0.5 * (unit_dir.y + 1.0);
      Color::from(1.0, 1.0, 1.0) * (1.0 - t)  + Color::from(0.5, 0.7, 1.0) * t
    }
  }
}

fn write_color(color: Color, stdout: &std::io::Stdout) -> io::Result<()> {
  let mut handle = stdout.lock();

  let ir: usize = (255.999 * color.x).floor() as usize;
  let ig: usize = (255.999 * color.y).floor() as usize;
  let ib: usize = (255.999 * color.z).floor() as usize;

  handle.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;

  Ok(())
}

fn main() -> io::Result<()> {

  // image
  let aspect_ratio: f64 = 16.0 / 9.0;
  let width: usize = 400;
  let height: usize = (width as f64 / aspect_ratio) as usize;

  // Camera
  let viewport_height = 2.0;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length = 1.0;

  let origin = Point3::from(0.0, 0.0, 0.0);
  let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
  let vertical = Vec3::from(0.0, viewport_height, 0.0);
  let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

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
      let u = i as f64 / (width-1) as f64;
      let v = j as f64 / (height-1) as f64;
      let ray = Ray { origin, direction: lower_left_corner + horizontal * u + vertical * v - origin };
      write_color(ray_color(&ray, &hit_list), &stdout)?;
    }
  }

  Ok(())
}
