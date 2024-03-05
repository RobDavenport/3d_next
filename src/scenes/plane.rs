use glam::Mat4;

use crate::{
    actor::Actor,
    graphics::{Gpu, IndexList, StaticMesh, VertexList, VertexParametersList},
    shaders::{BaseVertexShader, Textured},
    shapes,
};

use super::Scene;

pub struct PlaneScene {
    plane: Actor<2>,
}

impl Scene for PlaneScene {
    fn update(&mut self) { // Do nothing
    }

    fn draw(&self, gpu: &mut Gpu) {
        gpu.uniforms.model = self.plane.transform;
        gpu.uniforms.diffuse = crate::assets::textures::GAMERCADE_T;
        gpu.render_actor(&self.plane, BaseVertexShader, Textured);
    }
}

impl PlaneScene {
    pub fn new() -> Self {
        Self {
            plane: Actor {
                mesh: StaticMesh {
                    vertices: VertexList(shapes::PLANE),
                    indices: IndexList(shapes::PLANE_INDICES),
                    parameters: VertexParametersList(shapes::PLANE_UVS),
                },
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}
