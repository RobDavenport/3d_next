use glam::Mat4;

use crate::{
    actor::Actor,
    graphics::GraphicsDb,
    shaders::{BaseVertexShader, ColorBlendLit},
};

use super::Scene;

pub struct CubeModelScene {
    cube: Actor<6>,
}

impl CubeModelScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let actor_id = graphics_db.push_mesh(crate::assets::meshes::BOXVERTEXCOLORS);

        Self {
            cube: Actor {
                mesh_id: actor_id,
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}

impl Scene for CubeModelScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.cube.transform;
        gpu.render_actor(&self.cube, BaseVertexShader, ColorBlendLit);
    }

    fn update(&mut self) {
        // do nothing
    }
}
