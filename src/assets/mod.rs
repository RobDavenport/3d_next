mod generated;
pub use generated::*;

use crate::types::Color;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: &'static [u8],
}

const STRIDE: usize = 3;

impl Texture {
    pub fn get_sample(&self, u: usize, v: usize) -> Color {
        let index = ((v * self.width) + u) * STRIDE;
        let slice = &self.data[index..index + STRIDE];
        Color::new(slice[0], slice[1], slice[2])
    }
}
