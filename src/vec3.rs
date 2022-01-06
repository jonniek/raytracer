use std::ops::{Sub,Add,Mul,Div};
use std::io::{self, Write};
use crate::random::{random_double,random_double_in_range};

#[derive(Debug,PartialEq,Default,Clone,Copy)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}



pub type Color = Vec3;
pub type Point3 = Vec3;

impl Sub for Vec3 {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Add for Vec3 {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Add<f64> for Vec3 {
  type Output = Self;
  fn add(self, amount: f64) -> Self::Output {
    Self {
      x: self.x + amount,
      y: self.y + amount,
      z: self.z + amount,
    }
  }
}

impl Mul for Vec3 {
  type Output = Self;
  fn mul(self, other: Self) -> Self::Output {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;
  fn mul(self, multiplier: f64) -> Self::Output {
    Self {
      x: self.x * multiplier,
      y: self.y * multiplier,
      z: self.z * multiplier,
    }
  }
}

impl Div for Vec3 {
  type Output = Self;
  fn div(self, other: Self) -> Self::Output {
    Self {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z,
    }
  }
}

impl Div<f64> for Vec3 {
  type Output = Self;
  fn div(self, multiplier: f64) -> Self::Output {
    Self {
      x: self.x / multiplier,
      y: self.y / multiplier,
      z: self.z / multiplier,
    }
  }
}

impl Vec3 {
  pub fn random() -> Vec3 {
    Vec3::from(random_double(), random_double(), random_double())
  }

  pub fn random_range(min: f64, max: f64) -> Vec3 {
    Vec3::from(
      random_double_in_range(min, max),
      random_double_in_range(min, max),
      random_double_in_range(min, max),
    )
  }

  pub fn random_in_unit_sphere() -> Vec3 {
    loop {
      let p = Vec3::random_range(-1.0, 1.0);
      if p.len_squared() < 1.0 {
        return p;
      }
    }
  }

  pub fn random_unit_vector() -> Vec3 {
    Vec3::random_in_unit_sphere().unit_vector()
  }

  pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = Vec3::random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
      return in_unit_sphere;
    }

    in_unit_sphere * -1.0
  }

  pub fn from(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn len_squared(self) -> f64 {
    self.x*self.x + self.y*self.y + self.z*self.z
  }

  pub fn len(self) -> f64 {
    self.len_squared().sqrt()
  }

  pub fn dot(self, other: &Vec3) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(self, other: Vec3) -> Vec3 {
    Vec3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }

  pub fn unit_vector(self) -> Vec3 {
    let l = self.len();
    self / l
  }

  pub fn out(self, stdout: &io::Stdout) -> io::Result<()> {
    let mut handle = stdout.lock();
    handle.write_all(format!("{} {} {}", self.x, self.y, self.z).as_bytes())?;

    Ok(())
  }
}


#[test]
fn test_default() {
  assert_eq!(Vec3::default(), Vec3 { x: 0.0, y: 0.0, z: 0.0 })
}

#[test]
fn test_len_squared() {
  let v =  Vec3 { x: 2.0, y: 2.0, z: 1.0 };
  assert_eq!(v.len(), 3.0);
}

#[test]
fn test_len() {
  let v = Vec3 { x: 3.0, y: 3.0, z: 3.0 };
  assert_eq!(v.len_squared(), 22.0);
}

#[test]
fn test_sub() {
  assert_eq!(
    Vec3 { x: 3.0, y: 3.0, z: 1.2 } - Vec3 { x: 2.0, y: 3.0, z: 2.0 },
    Vec3 { x: 1.0, y: 0.0, z: -0.8 }
  );
}

#[test]
fn test_add() {
  assert_eq!(
    Vec3 { x: 3.0, y: 3.2, z: -1.2 } + Vec3 { x: 2.0, y: 3.0, z: 2.0 },
    Vec3 { x: 5.0, y: 6.2, z: 0.8 }
  );
}

#[test]
fn test_mul() {
  assert_eq!(
    Vec3 { x: 1.5, y: 3.1, z: -1.2 } * Vec3 { x: 2.0, y: 1.0, z: 2.0 },
    Vec3 { x: 3.0, y: 3.1, z: -2.4 }
  );
  assert_eq!(
    Vec3 { x: 1.5, y: 3.1, z: -1.2 } * 2.0,
    Vec3 { x: 3.0, y: 6.2, z: -2.4 }
  );
}

#[test]
fn test_div() {
  assert_eq!(
    Vec3 { x: 1.0, y: 3.1, z: -1.2 } / Vec3 { x: 2.0, y: 0.1, z: 2.0 },
    Vec3 { x: 0.5, y: 31.0, z: -0.6 }
  );
  assert_eq!(
    Vec3 { x: 1.0, y: 3.1, z: -1.2 } / 2.0,
    Vec3 { x: 0.5, y: 1.55, z: -0.6 }
  );
}