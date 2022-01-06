use crate::vec3::{Vec3,Point3};

#[derive(Debug,Copy,Clone,Default)]
pub struct Ray {
  pub origin: Point3,
  pub direction: Vec3,
}

impl Ray {
  pub fn at(self, t: f64) -> Vec3 {
    self.origin + self.direction * t
  }
}

#[test]
fn test_at() {

  let origin = Point3::default();
  let direction = Vec3 { x: 1.0, y: 0.0, z: 0.0 };

  assert_eq!(
    Ray { origin, direction }.at(3.5),
    Vec3 { x: 3.5, y: 0.0, z: 0.0 }
  );
}