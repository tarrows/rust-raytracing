use crate::vec3::Vec3;

// TODO?: remove clone and copy?
#[derive(Debug, Clone, Copy)]
pub struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Self {
    Ray { origin, direction }
  }

  pub fn point_at(self, t: f64) -> Vec3 {
    self.origin + t * self.direction
  }

  pub fn origin(self) -> Vec3 {
    self.origin
  }

  pub fn direction(self) -> Vec3 {
    self.direction
  }
}
