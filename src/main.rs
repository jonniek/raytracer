mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod random;
mod material;

use std::io::{self, Write};
use vec3::{Color,Point3,Vec3};
use ray::Ray;
use std::f64::INFINITY;
use hittable::{Hittable,HitList};
use sphere::Sphere;
use camera::Camera;
use random::random_double;
use rayon::prelude::*;
use material::{Scatterable,Material,Lambertian,Metal,Glass};

fn clamp(value: f64, min_value: f64, max_value: f64) -> f64 {
  if value < min_value {
    return min_value;
  }
  if value > max_value {
    return max_value;
  }
  value
}

fn ray_color(ray: &Ray, hit_list: &HitList, depth: isize) -> Color {
  if depth <= 0 {
    return Color::from(0.0, 0.0, 0.0);
  }

  match hit_list.hit(ray, 0.001, INFINITY) {
    Some(hit) => {

      match hit.material.scatter(ray, &hit) {
        Some((scattered_ray_option, albedo)) => {
          match scattered_ray_option {
            Some(scattered_ray) => {
              return albedo * ray_color(&scattered_ray, hit_list, depth - 1)
            },
            None => return albedo
          }
        },
        None => (),
      }
      return Color::from(0.0, 0.0, 0.0);
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


fn render(
  camera: &Camera,
  world: &HitList,
  image_width: usize,
  image_height: usize,
  samples: usize,
  max_depth: isize,
) -> Vec<Color> {

  let colors: Vec<Color> = (0..image_height * image_width)
    .into_par_iter()
    .map(|index| {
      let x = index % image_width;
      let y = index / image_width;

      let mut pixel_color = Color::from(0.0, 0.0, 0.0);
      for _ in 0..samples {
        let u = (x as f64 + random_double()) / (image_width-1) as f64;
        let v = (y as f64 + random_double()) / (image_height-1) as f64;
        let ray = camera.get_ray(u, v);
        pixel_color = pixel_color + ray_color(&ray, &world, max_depth);
      }

      pixel_color
    })
    .rev()
    .collect();

  colors
}

fn main() -> io::Result<()> {
  // world objects

  let mut world: HitList = HitList {
    objects: vec!(
      Sphere {
        radius: 1.0,
        center: Point3::from(-4.0, 1.0, 0.0),
        material: Material::Lambertian(Lambertian{ albedo: Color::from(0.4, 0.2, 0.1) }),
      },
      Sphere {
        radius: 1.0,
        center: Point3::from(0.0, 1.0, 0.0),
        material: Material::Glass(Glass{ refraction_index: 1.5 }),
      },
      Sphere {
        radius: 1.0,
        center: Point3::from(4.0, 1.0, 0.0),
        material: Material::Metal(Metal{ albedo: Color::from(0.7, 0.6, 0.5), fuzz: 0.0 }),
      },
      Sphere {
        radius: 100.0,
        center: Point3::from(0.0,-100.0,-1.0),
        material: Material::Lambertian(Lambertian{ albedo: Color::from(0.5, 0.5, 0.5) }),
      }
    )
  };

  for a in -11..11 {
    for b in -11..11 {
      let center = Point3::from(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());
      let material = match random_double() {
        n if n > 0.6 => Material::Lambertian(
          Lambertian{ albedo: Color::from(random_double(), random_double(), random_double()) }
        ),
        n if n > 0.2 => Material::Metal(
          Metal{ albedo: Color::from(random_double(), random_double(), random_double()), fuzz: random_double() }
        ),
        _ => Material::Glass(Glass{ refraction_index: 1.0 + random_double() }),
      };

        let sphere = Sphere {
          radius: random_double() * 0.4,
          center,
          material,
        };
        world.objects.push(sphere)
    }
  }


  // image params
  let lookfrom = Point3::from(13.0, 2.0, 3.0);
  let lookat = Point3::from(0.0,0.0,0.0);
  let vup = Vec3::from(0.0,1.0,0.0);
  let aspect_ratio: f64 = 3.0 / 2.0;
  let vertical_field_of_view = 20.0;
  let aperture = 0.1;
  let focus_distance = 10.0;
  let width: usize = 1200;
  let height: usize = (width as f64 / aspect_ratio) as usize;
  let samples_per_pixel = 500;
  let max_depth = 50;

  // Camera
  let camera = Camera::new(
    aspect_ratio,
    vertical_field_of_view,
    aperture,
    focus_distance,
    lookfrom,
    lookat,
    vup,
  );

  // render
  let stdout = io::stdout();
  {
    let mut handle = stdout.lock();
    let headers = format!("P3\n{} {}\n255\n", width, height);
    handle.write(headers.as_bytes())?;
  }
  let colors = render(&camera, &world, width, height, samples_per_pixel, max_depth);
  for color in colors {
    write_color(&stdout, color, samples_per_pixel)?;
  }

  Ok(())
}
