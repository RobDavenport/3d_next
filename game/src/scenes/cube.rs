use glam::Mat4;

use super::Scene;
use crate::{
    actor::Actor,
    generated::{meshes, textures},
    graphics::Gpu,
    shaders::{BaseVertexShader, TexturedNormalMapLit},
};

pub struct CubeScene {
    pub cube: Actor<8>,
}

impl CubeScene {
    pub fn new() -> Self {
        Self {
            cube: Actor {
                mesh: meshes::CUBE.as_mesh(),
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
        gpu.uniforms.diffuse = textures::BRICKWALL_TEX.as_texture();
        gpu.render_mesh(self.cube.mesh, BaseVertexShader, TexturedNormalMapLit);
    }
}
