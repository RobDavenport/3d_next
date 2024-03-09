mod binner;
mod clipping;
mod frame_buffer;
mod gpu;
mod rasterizer;
mod render_tile;
mod tile_manager;
mod z_buffer;

pub use frame_buffer::FrameBuffer;
pub use gpu::Gpu;
use shared::{texture::ArchivedTexture, vertex_parameters::VertexParameters, TriangleIndices};
pub use z_buffer::ZBuffer;

use glam::{Mat4, Vec3, Vec4};

use crate::generated::{meshes, textures};

#[derive(Clone)]
pub(crate) struct Triangle<const P: usize> {
    positions: [Vec4; 3],
    parameters: [VertexParameters<P>; 3],
}

pub struct Uniforms {
    // Pixel Shader
    pub light_position: Vec3,
    pub light_intensity: f32,
    pub ambient_light: f32,
    pub diffuse: &'static ArchivedTexture,
    pub normal: &'static ArchivedTexture,
    pub emissive: &'static ArchivedTexture,
    pub occlusion: &'static ArchivedTexture,

    // Vertex Shader
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            light_position: Vec3::default(),
            light_intensity: 1.25,
            ambient_light: 0.15,
            diffuse: textures::BRICKWALL_TEX.as_texture(),
            normal: textures::BRICKWALL_NORMAL_TEX.as_texture(),
            emissive: meshes::DAMAGEDHELMET_2_TEX.as_texture(),
            occlusion: meshes::DAMAGEDHELMET_3_TEX.as_texture(),

            model: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            projection: Mat4::IDENTITY,
        }
    }
}

// A mesh which is ready to be used
// #[derive(Clone, Copy)]
// pub struct StaticMesh<const P: usize> {
//     pub vertices: VertexList,
//     pub indices: IndexList,
//     pub parameters: VertexParametersList<P>,
// }
