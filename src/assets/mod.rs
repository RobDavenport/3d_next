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
    pub fn get_sample(&self, u: f32, v: f32) -> Color {
        let u = (u * (self.width - 1) as f32).clamp(0.0, (self.width - 1) as f32) as usize;
        let v = (v * (self.height - 1) as f32).clamp(0.0, (self.height - 1) as f32) as usize;

        let index = ((v * self.width) + u) * STRIDE;
        let slice = &self.data[index..index + STRIDE];
        Color::new(slice[0], slice[1], slice[2])
    }
}
