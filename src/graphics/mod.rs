mod clipping;
mod frame_buffer;
mod gpu;
mod rasterizer;
mod render_tile;
mod tile_manager;
mod z_buffer;

pub use frame_buffer::FrameBuffer;
pub use gpu::Gpu;
pub use z_buffer::ZBuffer;

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};

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

#[derive(Clone, Copy)]
pub struct VertexParametersList<const P: usize>(pub &'static [VertexParameters<P>]);

// A mesh which is ready to be used
#[derive(Clone, Copy)]
pub struct Mesh<const P: usize> {
    pub vertices: VertexList,
    pub indices: IndexList,
    pub parameters: VertexParametersList<P>,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TriangleIndices(pub u32, pub u32, pub u32);

#[derive(Clone, Copy)]
pub struct IndexList(pub &'static [TriangleIndices]);

#[derive(Clone, Copy)]
pub struct VertexList(pub &'static [Vec3]);
