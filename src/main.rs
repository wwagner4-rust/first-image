use image::{ImageBuffer, RgbImage};
use rand::Rng;
use std::cmp::min;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(1000, 800);
    let (width, height) = img.dimensions();
    let mut cnt = 0;
    let max = (width * height) as f32;
    let _diff = 1.0 / max;
    for p in img.pixels_mut() {
        let color = cnt as f32 * _diff;
        let noise = rand::thread_rng().gen_range(0..5) as u16;
        let blue = min(((color * 256.0) as u16) + noise, 255) as u8;
        let red = (noise + 100) as u8;
        *p = image::Rgb([red, 0, blue]);
        cnt = cnt + 1;
    }
    println!("Wrote buffer");
    let f_name = "/home/wwagner4/work/rust/out/image.png";
    image::save_buffer(f_name, &img, width, height, image::ColorType::Rgb8).unwrap();
    println!("Wrote image to {f_name}");
}
