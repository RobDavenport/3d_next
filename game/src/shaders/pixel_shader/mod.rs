mod color_blend;
pub use color_blend::*;

mod textured;
use shared::types::Color;
pub use textured::*;

use crate::graphics::Uniforms;

pub trait PixelShader<const PSIN: usize>: Copy {
    fn run(uniforms: &Uniforms, parameters: [f32; PSIN]) -> Color;
}
