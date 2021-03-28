use num_traits::sign::Unsigned;
use crate::pixel_types::{Rgb8, Pixel};
use image::RgbImage;

pub fn img2matrix(img: RgbImage) -> Pixel<U: Unsigned+Clone> {
    let ipixel = img.get_pixel(0,0);
    let pixel = Rgb8::new(0, 0, 0);
    println!("ipixel: {:?}", ipixel);
    println!("pixel: {:?}", pixel);
}
