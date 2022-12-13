use image::{ImageBuffer, RgbImage};

fn main() {
    let img: RgbImage = ImageBuffer::new(512, 512);
    println!("Created an image");
    let (width, height) = img.dimensions();
    println!("Image width and height are {width} {height}");

    let f_name = "/home/itsv.org.sv-services.at/31100428/work/rust/out/image.png";
    image::save_buffer(f_name, &img, width, height, image::ColorType::Rgb8).unwrap();
    println!("Wrote image to {f_name}");

}
