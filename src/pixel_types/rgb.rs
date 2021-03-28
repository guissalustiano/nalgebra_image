use num_traits::sign::Unsigned;
use crate::pixel_types::Pixel;

/// Representation of an RGB pixel (Red, Green, Blue)
#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub struct Rgb<U: Unsigned + Copy> {
    pub r: U,
    pub g: U,
    pub b: U,
}

pub type Rgb8 = Rgb<u8>;

impl<U: Unsigned + Copy> Rgb<U> {
    pub fn new(red: U, green: U, blue: U) -> Rgb<U> {
        Rgb {
            r: red,
            g: green,
            b: blue,
        }
    }

    pub fn from_array(s: &[U]) -> Rgb<U> {
        Rgb::new(s[0], s[1], s[2])
    }
}

impl<U: Unsigned + Copy> Pixel<U> for Rgb<U> {}
