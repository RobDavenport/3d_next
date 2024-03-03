use glam::Mat4;

use crate::{
    actor::Actor,
    graphics::{Gpu, IndexList, Mesh, VertexList, VertexParametersList},
    shaders::{BaseVertexShader, Textured},
    shapes::{self},
};

use super::Scene;

pub struct TriangleScene {
    triangle: Actor<2>,
}

impl Scene for TriangleScene {
    fn update(&mut self) {
        // Do nothing
    }

    fn draw(&self, gpu: &mut Gpu) {
        gpu.uniforms.model = self.triangle.transform;
        gpu.uniforms.diffuse = crate::assets::textures::GAMERCADE_T;
        gpu.render_actor(&self.triangle, BaseVertexShader, Textured);
    }
}

impl TriangleScene {
    pub fn new() -> Self {
        Self {
            triangle: Actor {
                mesh: Mesh {
                    vertices: VertexList(shapes::TRIANGLE),
                    indices: IndexList(shapes::TRI_INDICES),
                    parameters: VertexParametersList(shapes::PLANE_UVS),
                },
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}
