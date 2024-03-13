use glam::Mat4;

use crate::{
    actor::Actor,
    generated::{meshes, textures},
    graphics::Gpu,
    shaders::{BaseVertexShader, Textured},
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
        gpu.uniforms.diffuse = textures::GAMERCADE_TEX.as_texture();
        gpu.render_mesh(self.plane.mesh, BaseVertexShader, Textured);
    }
}

impl PlaneScene {
    pub fn new() -> Self {
        Self {
            plane: Actor {
                mesh: meshes::PLANE.as_mesh(),
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}
