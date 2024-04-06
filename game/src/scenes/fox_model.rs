use gamercade_rs::prelude as gc;
use glam::{Mat4, Vec3};
use shared::animation::ArchivedAnimation;

use crate::{
    actor::Actor,
    animation::Animator,
    generated::meshes,
    shaders::{Animated, TexturedLit},
};

use super::Scene;

pub struct FoxModelScene {
    fox: Actor<5>,
    shader: Animated<24, 4>,
    anim_index: usize,
    anims: [&'static ArchivedAnimation; 3],
}

impl FoxModelScene {
    pub fn new() -> Self {
        gc::console_log("Initialize FOX SCENE:");

        let anims = [
            meshes::FOX_SURVEY_ANM.as_anim(),
            meshes::FOX_WALK_ANM.as_anim(),
            meshes::FOX_RUN_ANM.as_anim(),
        ];

        let shader = Animated {
            animator: Animator::new(
                meshes::FOX_SKL.as_skeleton(),
                meshes::FOX_SKN.as_skin(),
                meshes::FOX_SURVEY_ANM.as_anim(),
            ),
        };

        Self {
            fox: Actor {
                mesh: meshes::FOX.as_mesh(),
                transform: Mat4::from_scale(Vec3::splat(0.03)),
                delta: 0.0,
            },
            shader,
            anim_index: 0,
            anims,
        }
    }
}

impl Scene for FoxModelScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.fox.transform;
        gpu.uniforms.diffuse = meshes::FOX_0_TEX.as_texture();

        gpu.render_mesh(self.fox.mesh, self.shader, TexturedLit);
    }

    fn update(&mut self) {
        self.shader.animator.update_time(0.016);

        if let Some(true) = gc::button_right_stick_pressed(0) {
            self.anim_index += 1;

            if self.anim_index == self.anims.len() {
                self.anim_index = 0;
            }

            self.shader.animator.animation = self.anims[self.anim_index];
        } else if let Some(true) = gc::button_left_stick_pressed(0) {
            self.anim_index -= 1;

            if self.anim_index == usize::MAX {
                self.anim_index = self.anims.len() - 1;
            }

            self.shader.animator.animation = self.anims[self.anim_index];
        }
    }
}
