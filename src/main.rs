mod pixel_type;

use crate::pixel_type::*;

fn main() {
    let pixel = Rgb8::new(120, 200, 0);
    println!("{:?}", pixel);
}
