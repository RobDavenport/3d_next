use glam::{Mat4, Vec3};

use crate::{
    actor::Actor,
    graphics::GraphicsDb,
    shaders::{BaseVertexShader, TexturedLit},
};

use super::Scene;

pub struct DuckModelScene {
    duck: Actor<5>,
}

impl DuckModelScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let actor_id = graphics_db.push_mesh(crate::assets::meshes::DUCK());

        Self {
            duck: Actor {
                mesh_id: actor_id,
                transform: Mat4::from_scale(Vec3::splat(0.03)),
                delta: 0.0,
            },
        }
    }
}

impl Scene for DuckModelScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.duck.transform;
        gpu.uniforms.diffuse = crate::assets::meshes::DUCK_0;
        gpu.render_actor(&self.duck, BaseVertexShader, TexturedLit);
    }

    fn update(&mut self) {
        // do nothing
    }
}
