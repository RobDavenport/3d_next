use std::f32::consts::FRAC_PI_2;

use glam::{Mat4, Vec3};

use crate::{
    actor::Actor,
    generated::meshes,
    shaders::{BaseVertexShader, TexturedLit},
};

use super::Scene;

pub struct HelmetModelSimpleScene {
    helmet: Actor<5>,
}

impl HelmetModelSimpleScene {
    pub fn new() -> Self {
        Self {
            helmet: Actor {
                mesh: meshes::DAMAGEDHELMET.as_mesh(),
                transform: Mat4::from_scale(Vec3::splat(4.0)) * Mat4::from_rotation_x(FRAC_PI_2),
                delta: 0.0,
            },
        }
    }
}

impl Scene for HelmetModelSimpleScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.helmet.transform;
        gpu.uniforms.diffuse = meshes::DAMAGEDHELMET_0_TEX.as_texture();
        gpu.uniforms.normal = meshes::DAMAGEDHELMET_4_TEX.as_texture();
        gpu.render_actor(&self.helmet, BaseVertexShader, TexturedLit);
    }

    fn update(&mut self) {
        // do nothing
    }
}
