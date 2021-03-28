use nalgebra::DMatrix;
use pixel_test::pixel_types::Rgb8;
use pixel_test::image::img2matrix;

fn main() {
    let _p = Rgb8::new(129, 30, 40);

    let img = image::open("img.png").expect("Image not found");
    let img = img.to_rgb8();

    let (weigth, heigh) = img.dimensions();
    let a = img2matrix(img);

    println!("{:?}", a)

}
