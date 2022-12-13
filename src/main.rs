use image::{ImageBuffer, RgbImage};

fn main() {
    let mut img: RgbImage = ImageBuffer::new(3000, 2000);
    let (width, height) = img.dimensions();
    let mut cnt = 0;
    let max = (width * height) as f32;
    let _diff = 1.0 / max;
    for p in img.pixels_mut() {
        let color = cnt as f32 * _diff;
        *p = image::Rgb([0, 100, (color * 256.0) as u8]);
        cnt = cnt + 1;
    }

    let f_name = "/home/itsv.org.sv-services.at/31100428/work/rust/out/image.png";
    image::save_buffer(f_name, &img, width, height, image::ColorType::Rgb8).unwrap();
    println!("Wrote image to {f_name}");

}
