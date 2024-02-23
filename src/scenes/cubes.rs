use std::f32::consts::FRAC_2_PI;

use glam::{Mat4, Vec3};

use super::Scene;
use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::VertexParameters,
    shapes::{self, cube_normals, CUBE_UVS},
};

pub struct CubesScene {
    pub cubes: Vec<Actor<5>>,
}

impl CubesScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let mut vertices = Vec::new();
        let mut parameters = Vec::new();
        let normals = cube_normals();

        shapes::cube(1.0)
            .into_iter()
            .enumerate()
            .for_each(|(i, x)| {
                vertices.push(x);
                let normal = normals[i];
                let uv = CUBE_UVS[i];
                parameters.push(VertexParameters([
                    uv[0], uv[1], normal[0], normal[1], normal[2],
                ]));
            });

        let indices = IndexList(
            shapes::CUBE_INDICES
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        );
        let actor_id = graphics_db.push_mesh(Mesh {
            vertices: VertexList(vertices.into_boxed_slice()),
            indices,
            parameters: ParameterData(parameters.into_boxed_slice()),
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

    fn draw(&self, gpu: &mut Gpu, graphics_db: &mut GraphicsDb) {
        self.cubes.iter().for_each(|cube| {
            graphics_db.base_vertex_shader.model = cube.transform;
            gpu.render_actor(
                cube,
                &graphics_db.base_vertex_shader,
                &graphics_db.textured_lit,
            );
        })
    }
}
