use glam::{Mat4, Vec3};

use crate::{
    actor::Actor,
    assets::meshes,
    graphics::Skin,
    shaders::{BaseVertexShader, TexturedLit},
};

use super::Scene;

pub struct FoxModelScene {
    fox: Actor<5>,
    skin: Skin<24>,
}

impl FoxModelScene {
    pub fn new() -> Self {
        Self {
            fox: Actor {
                mesh: meshes::FOX.as_mesh(),
                transform: Mat4::from_scale(Vec3::splat(0.015)),
                delta: 0.0,
            },
            skin: meshes::FOX_S.as_skin(),
        }
    }
}

impl Scene for FoxModelScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.fox.transform;
        gpu.uniforms.diffuse = crate::assets::meshes::FOX_0_T;
        gpu.render_actor(&self.fox, BaseVertexShader, TexturedLit);
    }

    fn update(&mut self) {
        // do nothing
    }
}
