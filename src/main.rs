extern crate image;
use image::ImageBuffer;
use raytracing::vec3::Vec3;

fn main() {
  let nx = 200;
  let ny = 100;
  let mut imgbuf: ImageBuffer<image::Rgb<u8>, _> = ImageBuffer::new(nx, ny);

  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let a = x as f64 / nx as f64;
    let b = (ny - 1 - y) as f64 / ny as f64;
    let col = Vec3::new(a, b, 0.2);
    let (r, g, b) = (255.99 * col).map_to(&|p: f64| p.trunc() as u8);

    *pixel = image::Rgb([r, g, b]);
  }

  imgbuf.save("output.png").unwrap();
}
