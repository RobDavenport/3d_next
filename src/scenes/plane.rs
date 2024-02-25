use glam::Mat4;

use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::{BaseVertexShader, Textured, VertexParameters},
    shapes::{self, PLANE_UVS},
};

use super::Scene;

pub struct PlaneScene {
    plane: Actor<2>,
}

impl Scene for PlaneScene {
    fn update(&mut self) { // Do nothing
    }
    
    fn draw(&self, gpu: &mut Gpu) {
        gpu.uniforms.model = self.plane.transform;
        gpu.render_actor(&self.plane, BaseVertexShader, Textured);
    }
}

impl PlaneScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let mut vertices = Vec::new();
        let mut parameters = Vec::new();

        shapes::plane(1.0)
            .into_iter()
            .enumerate()
            .for_each(|(i, x)| {
                vertices.push(x);
                let uv = PLANE_UVS[i];
                parameters.push(VertexParameters([uv[0], uv[1]]));
            });

        let indices = IndexList(
            shapes::PLANE_INDICES
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
            plane: Actor {
                mesh_id: actor_id,
                transform: Mat4::IDENTITY,
                delta: 0.0,
            },
        }
    }
}
