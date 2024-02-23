mod color_blend;
pub use color_blend::*;

mod textured;
pub use textured::*;

use crate::types::Color;

pub trait PixelShader<const PSIN: usize> {
    fn run(&self, parameters: [f32; PSIN]) -> Color;
}
