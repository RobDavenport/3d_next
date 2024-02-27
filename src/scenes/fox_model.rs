use glam::{Mat4, Vec3};

use crate::{
    actor::Actor,
    graphics::GraphicsDb,
    shaders::{BaseVertexShader, Textured},
};

use super::Scene;

pub struct FoxModelScene {
    fox: Actor<2>,
}

impl FoxModelScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let actor_id = graphics_db.push_mesh(crate::assets::meshes::FOX());

        Self {
            fox: Actor {
                mesh_id: actor_id,
                transform: Mat4::from_scale(Vec3::splat(0.015)),
                delta: 0.0,
            },
        }
    }
}

impl Scene for FoxModelScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.fox.transform;
        gpu.uniforms.diffuse = crate::assets::meshes::FOX_0;
        gpu.render_actor(&self.fox, BaseVertexShader, Textured);
    }

    fn update(&mut self) {
        // do nothing
    }
}
