use glam::{Vec3, Vec4Swizzles};

use crate::{graphics::Uniforms, shaders::VertexParameters};

use super::{transform_point_to_clip_space, VertexShader, VertexShaderOutput};

#[derive(Default)]
pub struct BaseVertexShader;

/// Used for Color Blending (RGB)
impl VertexShader<3, 3> for BaseVertexShader {
    fn run(uniforms: &Uniforms, position: Vec3, input: [f32; 3]) -> VertexShaderOutput<3> {
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
    fn run(uniforms: &Uniforms, position: Vec3, input: [f32; 2]) -> VertexShaderOutput<2> {
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
    fn run(uniforms: &Uniforms, position: Vec3, input: [f32; 6]) -> VertexShaderOutput<9> {
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
    fn run(uniforms: &Uniforms, position: Vec3, input: [f32; 5]) -> VertexShaderOutput<8> {
        let [u, v, norm_x, norm_y, norm_z] = input;
        let frag_position = (uniforms.model * position.extend(1.0)).xyz();

        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let position = transform_point_to_clip_space(&position, &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters([
                u,
                v,
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

// Used for Textured & Lit via Normal Map
impl VertexShader<2, 5> for BaseVertexShader {
    fn run(uniforms: &Uniforms, position: Vec3, input: [f32; 2]) -> VertexShaderOutput<5> {
        let [u, v] = input;
        let frag_position = (uniforms.model * position.extend(1.0)).xyz();

        let mvp = uniforms.projection * (uniforms.view * uniforms.model);
        let position = transform_point_to_clip_space(&position, &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters([u, v, frag_position.x, frag_position.y, frag_position.z]),
        }
    }
}
