use pixel_test::pixel_types::Rgb8;
use nalgebra::Vector2;

fn main() {
    let p = Rgb8::new(129, 30, 40);
    let v = Vector2::new(p, p);
    println!("{:?}", v);
}
