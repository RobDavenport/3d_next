use glam::{Mat4, Quat, Vec3};
use shared::{mesh::ArchivedMesh, texture::ArchivedTexture};

use crate::{
    actor::Actor,
    animation::Animator,
    generated::meshes,
    shaders::{Animated, BaseVertexShader, TexturedLit},
};

use super::Scene;

pub struct VsScene {
    background: &'static ArchivedMesh<5>,
    bg_texture: &'static ArchivedTexture,

    mesh_1: Actor<5>,
    shader_1: Animated<33, 4>,

    mesh_2: Actor<5>,
    shader_2: Animated<33, 4>,
}

impl VsScene {
    pub fn new() -> Self {
        let background = meshes::VSBACKGROUND.as_mesh();
        let bg_texture = meshes::VSBACKGROUND_0_TEX.as_texture();

        let shader_1 = Animated {
            animator: Animator::new(
                meshes::CHARTEST_SKL.as_skeleton(),
                meshes::CHARTEST_SKN.as_skin(),
                meshes::CHARTEST_IDLE_ANM.as_anim(),
            ),
        };

        let mesh_1 = Actor {
            mesh: meshes::CHARTEST.as_mesh(),
            transform: Mat4::from_translation(Vec3::new(-4.0, -1.0, 0.0))
                * Mat4::from_scale(Vec3::splat(2.0)),
            delta: 0.0,
        };

        let shader_2 = Animated {
            animator: Animator::new(
                meshes::CHARTEST_SKL.as_skeleton(),
                meshes::CHARTEST_SKN.as_skin(),
                meshes::CHARTEST_IDLE_ANM.as_anim(),
            ),
        };

        let mesh_2 = Actor {
            mesh: meshes::CHARTEST.as_mesh(),
            transform: Mat4::from_scale_rotation_translation(
                Vec3::new(-2.0, 2.0, 2.0),
                Quat::IDENTITY,
                Vec3::new(4.0, -1.0, 0.0),
            ),
            delta: 0.0,
        };

        Self {
            mesh_1,
            shader_1,
            mesh_2,
            shader_2,
            bg_texture,
            background,
        }
    }
}

impl Scene for VsScene {
    fn draw(&self, gpu: &mut crate::graphics::Gpu) {
        // Characters first
        gpu.uniforms.diffuse = crate::generated::textures::TESTCHARTEXTURE_TEX.as_texture();
        gpu.uniforms.model = self.mesh_1.transform;
        gpu.render_mesh(self.mesh_1.mesh, self.shader_1, TexturedLit);

        gpu.uniforms.model = self.mesh_2.transform;
        gpu.render_mesh(self.mesh_2.mesh, self.shader_2, TexturedLit);

        // Then Background
        gpu.uniforms.diffuse = self.bg_texture;
        gpu.uniforms.model = Mat4::from_translation(Vec3::new(0.0, -1.0, 0.0));
        gpu.render_mesh(self.background, BaseVertexShader, TexturedLit);
    }

    fn update(&mut self) {
        self.shader_1.animator.update_time(0.016);
        self.shader_2.animator.update_time(0.016);
    }
}
