use glam::Mat4;

use crate::{
    actor::Actor,
    animation::Animator,
    generated::{meshes, textures},
    shaders::{Animated, TexturedLit},
};

use super::Scene;

pub struct MechScene {
    mesh: Actor<5>,
    shader: Animated<19, 4>,
}

impl MechScene {
    pub fn new() -> Self {
        let shader = Animated {
            animator: Animator::new(
                meshes::MECH_SKL.as_skeleton(),
                meshes::MECH_SKN.as_skin(),
                meshes::MECH_IDLE_ANM.as_anim(),
            ),
        };

        Self {
            mesh: Actor {
                mesh: meshes::MECH.as_mesh(),
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
            shader,
        }
    }
}

impl Scene for MechScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.mesh.transform;
        gpu.uniforms.diffuse = textures::ENDESGA32_TEX.as_texture();
        gpu.render_mesh(self.mesh.mesh, self.shader, TexturedLit);
    }

    fn update(&mut self) {
        self.shader.animator.update_time(0.016);
    }
}
