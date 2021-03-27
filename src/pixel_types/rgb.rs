use num_traits::sign::Unsigned;

/// Representation of an RGB pixel (Red, Green, Blue)
#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub struct Rgb<U: Unsigned> {
    pub r: U,
    pub g: U,
    pub b: U,
}

pub type Rgb8 = Rgb<u8>;

impl<U: Unsigned> Rgb<U> {
    pub fn new(red: U, green: U, blue: U) -> Rgb<U> {
        Rgb {
            r: red,
            g: green,
            b: blue,
        }
    }
}
