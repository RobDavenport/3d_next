use glam::{Mat4, Vec3};

use super::Scene;
use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::{BaseVertexShader, TexturedNormalLit, VertexParameters},
    shapes::{self},
};

pub struct CubeScene {
    pub cube: Actor<11>,
}

impl CubeScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let mut vertices = Vec::new();
        let mut parameters = Vec::new();

        shapes::cube(1.0)
            .into_iter()
            .for_each(|(position, uv, normal, tangent)| {
                vertices.push(position);
                let t = Vec3::from(tangent);
                let n = Vec3::from(normal);
                let b = n.cross(t);

                parameters.push(VertexParameters([
                    uv[0], uv[1], t.x, t.y, t.z, b.x, b.y, b.z, n.x, n.y, n.z,
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

    fn draw(&self, gpu: &mut Gpu) {
        gpu.uniforms.model = self.cube.transform;
        gpu.render_actor(&self.cube, BaseVertexShader, TexturedNormalLit);
    }
}
