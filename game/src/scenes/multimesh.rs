use glam::Mat4;

use crate::{
    actor::Actor,
    animation::Animator,
    generated::{meshes, textures},
    shaders::{Animated, TexturedLit},
};

use super::Scene;

pub struct MultimeshScene {
    mesh: Actor<5>,
    shader: Animated<2, 4>,
}

impl MultimeshScene {
    pub fn new() -> Self {
        let shader = Animated {
            animator: Animator::new(
                meshes::MULTIMESH_SKL.as_skeleton(),
                meshes::MULTIMESH_SKN.as_skin(),
                meshes::MULTIMESH_ARMATUREACTION_ANM.as_anim(),
            ),
        };

        Self {
            mesh: Actor {
                mesh: meshes::MULTIMESH.as_mesh(),
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
            shader,
        }
    }
}

impl Scene for MultimeshScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.mesh.transform;
        gpu.uniforms.diffuse = textures::ENDESGA32_TEX.as_texture();
        gpu.render_mesh(self.mesh.mesh, self.shader, TexturedLit);
    }

    fn update(&mut self) {
        self.shader.animator.update_time(0.008);
    }
}
