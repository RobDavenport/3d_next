use glam::Mat4;

use crate::{
    actor::Actor,
    animation::Animator,
    generated::meshes,
    shaders::{Animated, TexturedLit},
};

use super::Scene;

pub struct BlockbenchScene {
    mesh: Actor<5>,
    shader: Animated<2, 4>,
}

impl BlockbenchScene {
    pub fn new() -> Self {
        let shader = Animated {
            animator: Animator::new(
                meshes::BLOCKBENCH_SKL.as_skeleton(),
                meshes::BLOCKBENCH_SKN.as_skin(),
                meshes::BLOCKBENCH_ANIMATION_ANM.as_anim(),
            ),
        };

        Self {
            mesh: Actor {
                mesh: meshes::BLOCKBENCH.as_mesh(),
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
            shader,
        }
    }
}

impl Scene for BlockbenchScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.mesh.transform;
        gpu.uniforms.diffuse = meshes::BLOCKBENCH_0_TEX.as_texture();
        gpu.render_mesh(self.mesh.mesh, self.shader, TexturedLit);
    }

    fn update(&mut self) {
        self.shader.animator.update_time(0.008);
    }
}
