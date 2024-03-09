use std::f32::consts::FRAC_2_PI;

use glam::{Mat4, Vec3};

use super::Scene;
use crate::{
    actor::Actor,
    generated::{textures, meshes},
    graphics::Gpu,
    shaders::{BaseVertexShader, TexturedNormalMapLit},
};

pub struct CubesScene {
    pub cubes: Vec<Actor<8>>,
}

impl CubesScene {
    pub fn new() -> Self {
        let positions = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(2.5, -2.5, 0.0),
            Vec3::new(-2.5, 2.5, 0.0),
        ];

        let mesh = meshes::CUBE.as_mesh();
        let mut cubes = Vec::new();

        let mut delta = 0.0;
        positions.into_iter().for_each(|position| {
            cubes.push(Actor {
                mesh,
                transform: Mat4::from_translation(position),
                delta,
            });

            delta += FRAC_2_PI;
        });

        Self { cubes }
    }
}

impl Scene for CubesScene {
    fn update(&mut self) {
        self.cubes.iter_mut().for_each(|a| a.update());
    }

    fn draw(&self, gpu: &mut Gpu) {
        self.cubes.iter().for_each(|cube| {
            gpu.uniforms.model = cube.transform;
            gpu.uniforms.diffuse = textures::BRICKWALL_TEX.as_texture();
            gpu.uniforms.normal = textures::BRICKWALL_NORMAL_TEX.as_texture();
            gpu.render_actor(cube, BaseVertexShader, TexturedNormalMapLit);
        })
    }
}
