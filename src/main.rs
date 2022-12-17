use image::{ImageBuffer, RgbImage};
use rand::Rng;
use std::cmp::min;
use rayon::prelude::*;

fn main() {
    let n = 100;
    let index_list: Vec<i16> = (0..n).into_iter().collect();
    index_list.par_iter().for_each(|i| create_and_write_image(i, &(n - 1)));
}

fn create_and_write_image(index: &i16, max_index: &i16) {
    println!("running {index} {max_index}");
    let _green = ((*index as f32 / *max_index as f32) * 255.0) as u8;
    println!("blue: {:4}", _green);
    let mut img: RgbImage = ImageBuffer::new(500, 500);
    let (width, height) = img.dimensions();
    let mut cnt = 0;
    let max = (width * height) as f32;
    let _diff = 1.0 / max;
    for p in img.pixels_mut() {
        let color = cnt as f32 * _diff;
        let noise = rand::thread_rng().gen_range(0..5) as u16;
        let blue = min(((color * 256.0) as u16) + noise, 255) as u8;
        let red = (noise + 100) as u8;
        *p = image::Rgb([red, _green, blue]);
        cnt = cnt + 1;
    }
    println!("created buffer {index}");
    let f_name = format!("/home/wwagner4/work/rust/out/image-{:04}.png", index);
    image::save_buffer(&f_name, &img, width, height, image::ColorType::Rgb8).unwrap();
    println!("wrote image to {f_name}");
}
