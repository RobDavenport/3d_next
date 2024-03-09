use glam::Mat4;

use crate::{
    actor::Actor,
    generated::{meshes, textures},
    graphics::Gpu,
    shaders::{BaseVertexShader, Textured},
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
        gpu.uniforms.diffuse = textures::GAMERCADE_TEX.as_texture();
        gpu.render_actor(&self.triangle, BaseVertexShader, Textured);
    }
}

impl TriangleScene {
    pub fn new() -> Self {
        Self {
            triangle: Actor {
                mesh: meshes::TRIANGLE.as_mesh(),
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}
