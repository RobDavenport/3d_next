use glam::{Mat4, Vec3, Vec4};

mod base;
pub use base::*;
use shared::vertex_parameters::VertexParameters;

mod animated;
pub use animated::*;

use crate::graphics::Uniforms;

pub struct VertexShaderOutput<const OUT: usize> {
    pub position: Vec4,
    pub parameters: VertexParameters<OUT>,
}

pub trait VertexShader<const IN: usize, const OUT: usize> {
    fn run(
        &self,
        vertex_index: usize,
        uniforms: &Uniforms,
        position: Vec3,
        input: [f32; IN],
    ) -> VertexShaderOutput<OUT>;
}

fn transform_point_to_clip_space(position: &Vec4, mvp: &Mat4) -> Vec4 {
    // Convert vertex position to homogeneous coordinates (4D)
    let mut position_homogeneous = *position;

    // Apply projection transformation
    position_homogeneous = *mvp * position_homogeneous;

    // Return the transformed vertex in clip space
    position_homogeneous
}
