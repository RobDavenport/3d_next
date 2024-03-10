use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use shared::vertex_parameters::VertexParameters;

use crate::{animation::Animator, graphics::Uniforms};

use super::{transform_point_to_clip_space, VertexShader, VertexShaderOutput};

#[derive(Clone, Copy)]
pub struct Animated<const BONE_COUNT: usize, const MAX_INFLUENCES: usize>
{
    pub animator: Animator<BONE_COUNT, MAX_INFLUENCES>,
}

impl<const BONE_COUNT: usize, const MAX_INFLUENCES: usize>
    VertexShader<3, 3> for Animated<BONE_COUNT, MAX_INFLUENCES>
{
    fn run(
        &self,
        vertex_index: usize,
        uniforms: &Uniforms,
        position: Vec3,
        input: [f32; 3],
    ) -> VertexShaderOutput<3> {
        let [r, g, b] = input;

        let mut position = position.extend(1.0);
        let mut skeletal_mat = Mat4::IDENTITY;

        let skin = &self.animator.skin.0[vertex_index];
        skin.bones_indices.iter().zip(skin.weights.iter()).for_each(
            |(bone_index, bone_weights)| {
                let mat = self.animator.current_pose[*bone_index as usize] * *bone_weights;
                skeletal_mat += mat
            },
        );

        position = skeletal_mat * position;

        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let position = transform_point_to_clip_space(&position.xyz(), &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters([
                0.0 + r,
                0.25 + g,
                0.15 +  b,
            ]),
        }
    }
}

impl<const BONE_COUNT: usize, const MAX_INFLUENCES: usize>
    VertexShader<5, 8> for Animated<BONE_COUNT, MAX_INFLUENCES>
{
    fn run(
        &self,
        vertex_index: usize,
        uniforms: &Uniforms,
        position: Vec3,
        input: [f32; 5],
    ) -> VertexShaderOutput<8> {
        let [u, v, norm_x, norm_y, norm_z] = input;

        let mut normal = Vec4::new(norm_x, norm_y, norm_z, 0.0);
        let mut position = position.extend(1.0);
        let mut skeletal_mat = Mat4::IDENTITY;

        let skin = &self.animator.skin.0[vertex_index];
        skin.bones_indices.iter().zip(skin.weights.iter()).for_each(
            |(bone_index, bone_weights)| {
                let mat = self.animator.current_pose[*bone_index as usize] * *bone_weights;
                skeletal_mat += mat
            },
        );

        position = skeletal_mat * position;
        normal = skeletal_mat * normal;

        let frag_position = (uniforms.model * position).xyz();

        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let normal = (uniforms.model * normal).normalize();
        let position = transform_point_to_clip_space(&position.xyz(), &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters([
                u,
                v,
                normal.x,
                normal.y,
                normal.z,
                frag_position.x,
                frag_position.y,
                frag_position.z,
            ]),
        }
    }
}
