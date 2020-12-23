extern crate image;
use image::ImageBuffer;
use raytracing::ray::Ray;
use raytracing::vec3::{dot, Vec3};

fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
  let oc = ray.origin() - center;
  let a = dot(ray.direction(), ray.direction());
  let b = 2.0 * dot(oc, ray.direction());
  let c = dot(oc, oc) - radius * radius;
  let discriminant = b * b - 4.0 * a * c;

  if discriminant < 0.0 {
    -1.0
  } else {
    (-b - discriminant.sqrt()) / (2.0 * a)
  }
}

fn color(r: Ray) -> Vec3 {
  let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
  if t > 0.0 {
    let n = (r.point_at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
    return 0.5 * (n + Vec3::new(1.0, 1.0, 1.0));
  }

  let unit_direction = r.direction().unit();
  let t = 0.5 * (unit_direction.y() + 1.0);

  // linear blend (or linear interpolation / lerp) from white to blue
  (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
  let nx = 200;
  let ny = 100;
  let mut imgbuf: ImageBuffer<image::Rgb<u8>, _> = ImageBuffer::new(nx, ny);

  let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);

  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let u = x as f64 / nx as f64;
    let v = (ny - 1 - y) as f64 / ny as f64;
    let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
    let col = color(r);
    let (r, g, b) = (255.99 * col).map_to(&|p: f64| p.trunc() as u8);

    *pixel = image::Rgb([r, g, b]);
  }

  imgbuf.save("output.png").unwrap();
}
