use std::f32::consts::FRAC_PI_2;

use glam::{Mat4, Vec3};

use crate::{
    actor::Actor,
    animation::Animator,
    generated::meshes,
    shaders::{Animated, ColorBlend, TexturedLit},
};

use super::Scene;

pub struct RiggedSimpleScene {
    mesh: Actor<3>,
    shader: Animated<2, 4>,
}

impl RiggedSimpleScene {
    pub fn new() -> Self {
        let shader = Animated {
            animator: Animator::new(
                meshes::RIGGEDSIMPLE_SKL.as_skeleton(),
                meshes::RIGGEDSIMPLE_SKN.as_skin(),
                meshes::RIGGEDSIMPLE_UNNAMED_ANM.as_anim(),
            ),
        };

        Self {
            mesh: Actor {
                mesh: meshes::RIGGEDSIMPLE.as_mesh(),
                transform: Mat4::from_rotation_y(-FRAC_PI_2) * Mat4::from_rotation_x(-FRAC_PI_2),
                delta: 0.0,
            },
            shader,
        }
    }
}

impl Scene for RiggedSimpleScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.mesh.transform;

        gpu.render_mesh(&self.mesh.mesh, self.shader, ColorBlend);

        // gpu.render_animator(&self.shader.animator);
    }

    fn update(&mut self) {
        //self.shader.animator.update_time(0.008);
    }
}
