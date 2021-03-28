use num_traits::sign::Unsigned;

pub mod rgb;

pub use rgb::*;

pub trait Pixel<U: Unsigned + Copy>: Copy {}
