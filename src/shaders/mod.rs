mod pixel_shader;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
pub use pixel_shader::*;

pub struct VertexShaderOutput<const OUT: usize> {
    pub position: Vec4,
    pub parameters: VertexParameters<OUT>,
}

pub trait VertexShader<const IN: usize, const OUT: usize> {
    fn run(&self, position: Vec3, input: [f32; IN]) -> VertexShaderOutput<OUT>;
}

#[derive(Default)]
pub struct BaseVertexShader {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

impl VertexShader<3, 3> for BaseVertexShader {
    fn run(&self, position: Vec3, input: [f32; 3]) -> VertexShaderOutput<3> {
        let mvp = self.projection * (self.view * self.model);
        let position = transform_point_to_clip_space(&position, &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters(input),
        }
    }
}

impl VertexShader<2, 2> for BaseVertexShader {
    fn run(&self, position: Vec3, input: [f32; 2]) -> VertexShaderOutput<2> {
        let mvp = self.projection * (self.view * self.model);
        let position = transform_point_to_clip_space(&position, &mvp);

        VertexShaderOutput {
            position,
            parameters: VertexParameters(input),
        }
    }
}

impl VertexShader<6, 9> for BaseVertexShader {
    fn run(&self, position: Vec3, input: [f32; 6]) -> VertexShaderOutput<9> {
        let [r, g, b, norm_x, norm_y, norm_z] = input;
        let frag_position = (self.model * position.extend(1.0)).xyz();

        let mvp = self.projection * (self.view * self.model);
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

impl VertexShader<5, 8> for BaseVertexShader {
    fn run(&self, position: Vec3, input: [f32; 5]) -> VertexShaderOutput<8> {
        let [u, v, norm_x, norm_y, norm_z] = input;
        let frag_position = (self.model * position.extend(1.0)).xyz();

        let mvp = self.projection * (self.view * self.model);
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

fn transform_point_to_clip_space(position: &Vec3, mvp: &Mat4) -> Vec4 {
    // Convert vertex position to homogeneous coordinates (4D)
    let mut position_homogeneous = position.extend(1.0);

    // Apply projection transformation
    position_homogeneous = *mvp * position_homogeneous;

    // Return the transformed vertex in clip space
    position_homogeneous
}
