use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
  // for coordinate
  pub fn x(self) -> f64 {
    self.0
  }
  pub fn y(self) -> f64 {
    self.1
  }
  pub fn z(self) -> f64 {
    self.2
  }

  // for color
  pub fn r(self) -> f64 {
    self.0
  }
  pub fn g(self) -> f64 {
    self.1
  }
  pub fn b(self) -> f64 {
    self.2
  }

  pub fn len(self) -> f64 {
    (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
  }

  pub fn squared_len(self) -> f64 {
    self.0 * self.0 + self.1 * self.1 + self.2 * self.2
  }

  pub fn unit(self) -> Vec3 {
    // should this be panic?
    if self.0 == 0.0 && self.1 == 0.0 && self.2 == 0.0 {
      Vec3(0.0, 0.0, 0.0)
    } else {
      let k = 1.0 / self.len();
      Vec3(self.0 * k, self.1 * k, self.2 * k)
    }
  }

  pub fn dot(self, v: Vec3) -> f64 {
    self.0 * v.0 + self.1 * v.1 + self.2 * v.2
  }

  pub fn cross(self, v: Vec3) -> Vec3 {
    let x = self.1 * v.2 - self.2 * v.1;
    let y = self.2 * v.0 - self.0 * v.2;
    let z = self.0 * v.1 - self.1 * v.0;
    Vec3(x, y, z)
  }
}

impl ops::Add for Vec3 {
  type Output = Vec3;

  fn add(self, _rhs: Self) -> Self::Output {
    let x = self.0 + _rhs.0;
    let y = self.1 + _rhs.1;
    let z = self.2 + _rhs.2;
    Vec3(x, y, z)
  }
}

impl ops::Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, _rhs: Self) -> Self::Output {
    let x = self.0 - _rhs.0;
    let y = self.1 - _rhs.1;
    let z = self.2 - _rhs.2;
    Vec3(x, y, z)
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, _rhs: Self) -> Self::Output {
    let x = self.0 * _rhs.0;
    let y = self.1 * _rhs.1;
    let z = self.2 * _rhs.2;
    Vec3(x, y, z)
  }
}

impl ops::Div for Vec3 {
  type Output = Vec3;

  fn div(self, _rhs: Self) -> Self::Output {
    let x = self.0 / _rhs.0;
    let y = self.1 / _rhs.1;
    let z = self.2 / _rhs.2;
    Vec3(x, y, z)
  }
}

impl ops::Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, _rhs: f64) -> Self::Output {
    Vec3(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs)
  }
}

impl ops::Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, _rhs: Self::Output) -> Self::Output {
    Vec3(self * _rhs.0, self * _rhs.1, self * _rhs.2)
  }
}

impl ops::Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, _rhs: f64) -> Self::Output {
    Vec3(self.0 / _rhs, self.1 / _rhs, self.2 / _rhs)
  }
}

#[cfg(test)]
mod test {
  use crate::vec3::Vec3;

  const A: Vec3 = Vec3(0.1, 0.2, 0.3);
  const B: Vec3 = Vec3(0.0, 0.4, 0.3);
  const C: Vec3 = Vec3(0.0, 0.1, 0.2);
  const P: f64 = 44.5;
  const Q: f64 = 0.5;

  fn is_approx_zero(q: f64) -> bool {
    q.abs() < std::f64::EPSILON
  }

  fn is_approx_zero_vec(v: Vec3) -> bool {
    is_approx_zero(v.x()) && is_approx_zero(v.y()) && is_approx_zero(v.z())
  }

  #[test]
  fn arithmetic_add_works() {
    let actual = A + B;
    let expected = Vec3(0.1, 0.6, 0.6);
    assert!(is_approx_zero_vec(actual - expected));
  }

  #[test]
  fn arithmetic_mul_works() {
    let actual = B * A;
    let expected = Vec3(0.0, 0.08, 0.09);
    assert!(is_approx_zero_vec(actual - expected));
  }

  #[test]
  fn arithmetic_sub_works() {
    let actual = B / A;
    let expected = Vec3(0.0, 2.0, 1.0);
    assert!(is_approx_zero_vec(actual - expected));
  }

  #[test]
  fn broadcast_mul_prefix_works() {
    let actual = P * C;
    let expected = Vec3(0.0, 4.45, 8.9);
    assert!(is_approx_zero_vec(actual - expected));
  }

  #[test]
  fn broadcast_mul_suffix_works() {
    let actual = C * P;
    let expected = Vec3(0.0, 4.45, 8.9);
    assert!(is_approx_zero_vec(actual - expected));
  }

  #[test]
  fn broadcast_div_suffix_works() {
    let actual = C / Q;
    let expected = Vec3(0.0, 0.2, 0.4);
    assert!(is_approx_zero_vec(actual - expected));
  }

  #[test]
  fn unitvector_nonzero_len_1() {
    let actual = A.unit().len();
    let expected = 1.0;
    assert!(is_approx_zero(actual - expected));
  }

  #[test]
  fn unitvector_withzero_len_1() {
    let actual = B.unit().len();
    let expected = 1.0;
    assert!(is_approx_zero(actual - expected));
  }

  #[test]
  fn unitvector_zerovect_zerovect() {
    let z = Vec3(0.0, 0.0, 0.0);
    let actual = z.unit();
    assert!(is_approx_zero_vec(actual));
  }
}
