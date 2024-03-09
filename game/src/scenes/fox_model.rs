use gamercade_rs::api::text::console_log;
use glam::{Mat4, Vec3};

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
}

impl FoxModelScene {
    pub fn new() -> Self {
        let shader = Animated {
            animator: Animator::new(
            meshes::FOX_SKL.as_skeleton(),
            meshes::FOX_SKN.as_skin(),
        )};

        shader.animator.skeleton.0.iter().for_each(|bone| {
            console_log(&format!("children: {:?}", bone.children.to_vec()))
        });

        shader.animator.skin.0.iter().for_each(|entry| {
            console_log(&format!("indices: {:?}, weights: {:?}", entry.bones_indices.to_vec(), entry.weights.to_vec()))
        });

        Self {
            fox: Actor {
                mesh: meshes::FOX.as_mesh(),
                transform: Mat4::from_scale(Vec3::splat(0.015)),
                delta: 0.0,
            },
            shader,
        }
    }
}

impl Scene for FoxModelScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        gpu.uniforms.model = self.fox.transform;
        gpu.uniforms.diffuse = meshes::FOX_0_TEX.as_texture();

        gpu.render_mesh(&self.fox.mesh, self.shader, TexturedLit);
    }

    fn update(&mut self) {
        // do nothing
        self.shader.animator.time += 0.005;
    }
}
