use glam::Mat4;

use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::VertexParameters,
    shapes::{self, CUBE_SIMPLE_UVS},
};

use super::Scene;

pub struct TriangleScene {
    triangle: Actor<2>,
}

impl Scene for TriangleScene {
    fn update(&mut self) {
        // Do nothing
    }

    fn draw(&self, gpu: &mut Gpu, graphics_db: &mut GraphicsDb) {
        graphics_db.base_vertex_shader.model = self.triangle.transform;
        gpu.render_actor(
            &self.triangle,
            &graphics_db.base_vertex_shader,
            &graphics_db.textured,
        );
    }
}

impl TriangleScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let mut vertices = Vec::new();
        let mut parameters = Vec::new();

        shapes::plane(1.0)
            .into_iter()
            .enumerate()
            .for_each(|(i, x)| {
                vertices.push(x);
                let uv = CUBE_SIMPLE_UVS[i];
                parameters.push(VertexParameters([uv[0], uv[1]]));
            });

        let indices = IndexList(
            shapes::TRI_INDICES
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
            triangle: Actor {
                mesh_id: actor_id,
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}
