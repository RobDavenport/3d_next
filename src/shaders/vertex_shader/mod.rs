use glam::{Mat4, Vec3, Vec4};

mod base;
pub use base::*;

use super::VertexParameters;

pub struct VertexShaderOutput<const OUT: usize> {
    pub position: Vec4,
    pub parameters: VertexParameters<OUT>,
}

pub trait VertexShader<const IN: usize, const OUT: usize> {
    fn run(&self, position: Vec3, input: [f32; IN]) -> VertexShaderOutput<OUT>;
}

fn transform_point_to_clip_space(position: &Vec3, mvp: &Mat4) -> Vec4 {
    // Convert vertex position to homogeneous coordinates (4D)
    let mut position_homogeneous = position.extend(1.0);

    // Apply projection transformation
    position_homogeneous = *mvp * position_homogeneous;

    // Return the transformed vertex in clip space
    position_homogeneous
}
