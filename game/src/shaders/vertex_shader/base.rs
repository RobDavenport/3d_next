use glam::{Mat3, Vec3, Vec4, Vec4Swizzles};
use shared::vertex_parameters::VertexParameters;

use crate::graphics::Uniforms;

use super::{transform_point_to_clip_space, VertexShader, VertexShaderOutput};

#[derive(Default)]
pub struct BaseVertexShader;

/// Used for Color Blending (RGB)
impl VertexShader<3, 3> for BaseVertexShader {
    fn run(
        &self,
        _vertex_index: usize,
        uniforms: &Uniforms,
        position: Vec3,
        input: [f32; 3],
    ) -> VertexShaderOutput<3> {
        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let position = transform_point_to_clip_space(&position, &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters(input),
        }
    }
}

// Use for Textured
impl VertexShader<2, 2> for BaseVertexShader {
    fn run(
        &self,
        _vertex_index: usize,
        uniforms: &Uniforms,
        position: Vec3,
        input: [f32; 2],
    ) -> VertexShaderOutput<2> {
        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let position = transform_point_to_clip_space(&position, &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters(input),
        }
    }
}

// Used for Color Blend & Lit via Vertex Normals
impl VertexShader<6, 9> for BaseVertexShader {
    fn run(
        &self,
        _vertex_index: usize,
        uniforms: &Uniforms,
        position: Vec3,
        input: [f32; 6],
    ) -> VertexShaderOutput<9> {
        let [r, g, b, norm_x, norm_y, norm_z] = input;
        let frag_position = (uniforms.model * position.extend(1.0)).xyz();

        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let position = transform_point_to_clip_space(&position, &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters([
                r,
                g,
                b,
                norm_x,
                norm_y,
                norm_z,
                frag_position.x,
                frag_position.y,
                frag_position.z,
            ]),
        }
    }
}

// Used for Textured & Lit via Vertex Normals
impl VertexShader<5, 8> for BaseVertexShader {
    fn run(
        &self,
        _vertex_index: usize,
        uniforms: &Uniforms,
        position: Vec3,
        input: [f32; 5],
    ) -> VertexShaderOutput<8> {
        let [u, v, norm_x, norm_y, norm_z] = input;
        let frag_position = (uniforms.model * position.extend(1.0)).xyz();

        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let normal = (uniforms.model * Vec3::new(norm_x, norm_y, norm_z).extend(0.0)).normalize();
        let position = transform_point_to_clip_space(&position, &mvp);

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

// Used for Textured & Lit via Normal Map
impl VertexShader<8, 8> for BaseVertexShader {
    fn run(
        &self,
        _vertex_index: usize,
        uniforms: &Uniforms,
        position: Vec3,
        input: [f32; 8],
    ) -> VertexShaderOutput<8> {
        let [u, v, nx, ny, nz, tx, ty, tz] = input;
        let frag_position = (uniforms.model * position.extend(1.0)).xyz();

        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let position = transform_point_to_clip_space(&position, &mvp);

        let t = (uniforms.model * Vec4::new(tx, ty, tz, 0.0)).truncate();
        let n = (uniforms.model * Vec4::new(nx, ny, nz, 0.0)).truncate();
        let b = n.cross(t);
        let tbn = Mat3::from_cols(t, b, n).transpose();

        let tan_light = tbn * uniforms.light_position;
        let tan_pos = tbn * frag_position;

        VertexShaderOutput {
            position,
            parameters: VertexParameters([
                u,
                v,
                tan_light.x,
                tan_light.y,
                tan_light.z,
                tan_pos.x,
                tan_pos.y,
                tan_pos.z,
            ]),
        }
    }
}
