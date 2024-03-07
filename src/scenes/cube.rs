use glam::Mat4;

use super::Scene;
use crate::{
    actor::Actor,
    graphics::{Gpu, IndexList, StaticMesh, VertexList, VertexParametersList},
    shaders::{BaseVertexShader, TexturedNormalMapLit},
    shapes::{self},
};

pub struct CubeScene {
    pub cube: Actor<8>,
}

impl CubeScene {
    pub fn new() -> Self {
        Self {
            cube: Actor {
                mesh: StaticMesh {
                    vertices: VertexList(shapes::CUBE),
                    indices: IndexList(shapes::CUBE_INDICES),
                    parameters: VertexParametersList(shapes::CUBE_PARAMETERS),
                },
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}

impl Scene for CubeScene {
    fn update(&mut self) {}

    fn draw(&self, gpu: &mut Gpu) {
        gpu.uniforms.model = self.cube.transform;
        gpu.uniforms.diffuse = crate::assets::textures::BRICKWALL_TEX;
        gpu.render_actor(&self.cube, BaseVertexShader, TexturedNormalMapLit);
    }
}
