mod gpu;
pub mod graphics_db;

mod clipping;
mod rasterizer;

use glam::Vec4;
pub use gpu::Gpu;
pub use graphics_db::*;

use crate::shaders::PixelShaderInput;

#[derive(Clone)]
struct Triangle<const P: usize> {
    positions: [Vec4; 3],
    parameters: [PixelShaderInput<P>; 3],
}
