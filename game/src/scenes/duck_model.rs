use glam::{Mat4, Vec3};

use crate::{
    actor::Actor,
    generated::meshes,
    shaders::{BaseVertexShader, TexturedLit},
};

use super::Scene;

pub struct DuckModelScene {
    duck: Actor<5>,
}

impl DuckModelScene {
    pub fn new() -> Self {
        Self {
            duck: Actor {
                mesh: meshes::DUCK.as_mesh(),
                transform: Mat4::from_scale(Vec3::splat(0.01)),
                delta: 0.0,
            },
        }
    }
}

impl Scene for DuckModelScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.duck.transform;
        gpu.uniforms.diffuse = meshes::DUCK_0_TEX.as_texture();
        gpu.render_mesh(&self.duck.mesh, BaseVertexShader, TexturedLit);
    }

    fn update(&mut self) {
        // do nothing
    }
}
