mod color_blend;
pub use color_blend::*;

mod textured;
pub use textured::*;

mod helmet;
pub use helmet::*;

use crate::{graphics::Uniforms, types::Color};

pub trait PixelShader<const PSIN: usize>: Copy {
    fn run(uniforms: &Uniforms, parameters: [f32; PSIN]) -> Color;
}
