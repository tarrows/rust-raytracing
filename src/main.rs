extern crate image;
use image::ImageBuffer;

fn main() {
  let nx = 200;
  let ny = 100;
  let mut imgbuf: ImageBuffer<image::Rgb<u8>, _> = ImageBuffer::new(nx, ny);

  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let r = (255.99 * (x as f32 / nx as f32)).trunc() as u8;
    let g = (255.99 * ((ny - 1 - y) as f32 / ny as f32)).trunc() as u8;
    let b = (255.99 * 0.2 as f32).trunc() as u8;
    *pixel = image::Rgb([r, g, b]);
  }

  imgbuf.save("output.png").unwrap();
}
