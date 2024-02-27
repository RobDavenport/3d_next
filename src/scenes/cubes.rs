use std::f32::consts::FRAC_2_PI;

use glam::{Mat4, Vec3};

use super::Scene;
use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::{BaseVertexShader, TexturedNormalLit},
    shapes,
};

pub struct CubesScene {
    pub cubes: Vec<Actor<8>>,
}

impl CubesScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let actor_id = graphics_db.push_mesh(Mesh {
            vertices: VertexList(shapes::CUBE),
            indices: IndexList(shapes::CUBE_INDICES),
            parameters: ParameterData(shapes::CUBE_PARAMETERS),
        });

        let positions = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(2.5, -2.5, 0.0),
            Vec3::new(-2.5, 2.5, 0.0),
        ];

        let mut cubes = Vec::new();

        let mut delta = 0.0;
        positions.into_iter().for_each(|position| {
            cubes.push(Actor {
                mesh_id: actor_id,
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
            gpu.uniforms.diffuse = crate::assets::textures::BRICKWALL;
            gpu.uniforms.normal = crate::assets::textures::BRICKWALL_NORMAL;
            gpu.render_actor(cube, BaseVertexShader, TexturedNormalLit);
        })
    }
}
