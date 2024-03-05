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
pub use z_buffer::ZBuffer;

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};

use crate::{
    assets::{meshes, textures, Texture},
    shaders::VertexParameters,
};

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
    pub diffuse: &'static Texture,
    pub normal: &'static Texture,
    pub emissive: &'static Texture,
    pub occlusion: &'static Texture,

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
            diffuse: textures::BRICKWALL_T,
            normal: textures::BRICKWALL_NORMAL_T,
            emissive: meshes::DAMAGEDHELMET_2_T,
            occlusion: meshes::DAMAGEDHELMET_3_T,

            model: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            projection: Mat4::IDENTITY,
        }
    }
}

#[derive(Clone, Copy)]
pub struct VertexParametersList<const P: usize>(pub &'static [VertexParameters<P>]);

// A mesh which is ready to be used
#[derive(Clone, Copy)]
pub struct StaticMesh<const P: usize> {
    pub vertices: VertexList,
    pub indices: IndexList,
    pub parameters: VertexParametersList<P>,
}

#[derive(Clone, Copy)]
pub struct Skin<const B: usize> {
    pub bones: BonesList<B>,
    pub weights: WeightsList<B>,
}

pub struct Skeleton {
    pub inverse_bind_matrices: &'static [Mat4],
}

#[derive(Clone, Copy)]
pub struct BonesList<const B: usize>(pub &'static [[u32; B]]);
#[derive(Clone, Copy)]
pub struct WeightsList<const B: usize>(pub &'static [[f32; B]]);

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TriangleIndices(pub u32, pub u32, pub u32);

#[derive(Clone, Copy)]
pub struct IndexList(pub &'static [TriangleIndices]);

#[derive(Clone, Copy)]
pub struct VertexList(pub &'static [Vec3]);
