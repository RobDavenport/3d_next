use glam::{Mat4, Vec3};

use crate::{
    actor::Actor,
    animation::Animator,
    generated::meshes,
    shaders::{Animated, ColorBlend, TexturedLit},
};

use super::Scene;

pub struct RiggedFigureScene {
    mesh: Actor<3>,
    shader: Animated<19, 3>,
}

impl RiggedFigureScene {
    pub fn new() -> Self {
        let shader = Animated {
            animator: Animator::new(
                meshes::RIGGEDFIGURE_SKL.as_skeleton(),
                meshes::RIGGEDFIGURE_SKN.as_skin(),
                meshes::RIGGEDFIGURE_UNNAMED_ANM.as_anim(),
            ),
        };

        Self {
            mesh: Actor {
                mesh: meshes::RIGGEDFIGURE.as_mesh(),
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
            shader,
        }
    }
}

impl Scene for RiggedFigureScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.mesh.transform;

        gpu.render_mesh(&self.mesh.mesh, self.shader, ColorBlend);
        //gpu.render_animator(&self.shader.animator);
    }

    fn update(&mut self) {
        self.shader.animator.update_time(0.016);
    }
}
