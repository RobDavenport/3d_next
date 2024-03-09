use glam::Mat4;

use crate::{
    actor::Actor,
    generated::meshes,
    shaders::{BaseVertexShader, ColorBlendLit},
};

use super::Scene;

pub struct CubeModelScene {
    cube: Actor<6>,
}

impl CubeModelScene {
    pub fn new() -> Self {
        Self {
            cube: Actor {
                mesh: meshes::BOXVERTEXCOLORS.as_mesh(),
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
