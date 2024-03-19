use glam::Mat4;

use crate::{
    actor::Actor,
    animation::Animator,
    generated::meshes,
    shaders::{Animated, TexturedLit},
};

use super::Scene;

pub struct HandScene {
    mesh: Actor<5>,
    shader: Animated<10, 4>,
}

impl HandScene {
    pub fn new() -> Self {
        let shader = Animated {
            animator: Animator::new(
                meshes::HANDRIG_SKL.as_skeleton(),
                meshes::HANDRIG_SKN.as_skin(),
                meshes::HANDRIG_THUMBSUP_ANM.as_anim(),
            ),
        };

        Self {
            mesh: Actor {
                mesh: meshes::HANDRIG.as_mesh(),
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
            shader,
        }
    }
}

impl Scene for HandScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.mesh.transform;
        gpu.uniforms.diffuse = meshes::HANDRIG_0_TEX.as_texture();
        gpu.render_mesh(self.mesh.mesh, self.shader, TexturedLit);
    }

    fn update(&mut self) {
        self.shader.animator.update_time(0.008);
    }
}
