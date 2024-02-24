mod gpu;
pub mod graphics_db;

mod clipping;
mod rasterizer;

use glam::{Mat4, Vec3, Vec4};
pub use gpu::Gpu;
pub use graphics_db::*;

use crate::{assets::Texture, shaders::VertexParameters};

#[derive(Clone)]
struct Triangle<const P: usize> {
    positions: [Vec4; 3],
    parameters: [VertexParameters<P>; 3],
}

pub struct Uniforms {
    // Pixel Shader
    pub light_position: Vec3,
    pub light_intensity: f32,
    pub ambient_light: f32,
    pub diffuse: &'static Texture,
    pub normal: &'static Texture,

    // Vertex Shader
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}
