use std::f32::consts::FRAC_PI_2;

use glam::{Mat4, Vec3};

use crate::{
    actor::Actor,
    assets::meshes,
    gc,
    shaders::{BaseVertexShader, TexturedLit},
};

use super::Scene;

pub struct HelmetModelScene {
    helmet: Actor<5>,
}

impl HelmetModelScene {
    pub fn new() -> Self {
        Self {
            helmet: Actor {
                mesh: meshes::DAMAGEDHELMET.as_mesh(),
                transform: Mat4::from_scale(Vec3::splat(3.0)) * Mat4::from_rotation_x(FRAC_PI_2),
                delta: 0.0,
            },
        }
    }
}

impl Scene for HelmetModelScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.helmet.transform;
        gpu.uniforms.diffuse = meshes::DAMAGEDHELMET_0_T;
        gpu.render_actor(&self.helmet, BaseVertexShader, TexturedLit);
    }

    fn update(&mut self) {
        // do nothing
    }
}
