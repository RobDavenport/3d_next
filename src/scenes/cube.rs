use glam::Mat4;

use super::Scene;
use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::VertexParameters,
    shapes::{self, CUBE_SIMPLE_UVS},
};

pub struct CubeScene {
    pub cube: Actor<2>,
}

impl CubeScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let mut vertices = Vec::new();
        let mut parameters = Vec::new();

        shapes::cube_simple(1.0)
            .into_iter()
            .enumerate()
            .for_each(|(i, x)| {
                vertices.push(x);
                let uv = CUBE_SIMPLE_UVS[i];
                parameters.push(VertexParameters([uv[0], uv[1]]));
            });

        let indices = IndexList(
            shapes::CUBE_SIMPLE_INDICES
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        );
        let actor_id = graphics_db.push_mesh(Mesh {
            vertices: VertexList(vertices.into_boxed_slice()),
            indices,
            parameters: ParameterData(parameters.into_boxed_slice()),
        });

        Self {
            cube: Actor {
                mesh_id: actor_id,
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}

impl Scene for CubeScene {
    fn update(&mut self) {}

    fn draw(&self, gpu: &mut Gpu, graphics_db: &mut GraphicsDb) {
        graphics_db.base_vertex_shader.model = self.cube.transform;
        gpu.render_actor(
            &self.cube,
            &graphics_db.base_vertex_shader,
            &graphics_db.textured_normal_lit,
        );
    }
}
