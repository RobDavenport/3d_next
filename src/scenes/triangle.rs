use glam::Mat4;

use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::{BaseVertexShader, Textured},
    shapes::{self},
};

use super::Scene;

pub struct TriangleScene {
    triangle: Actor<2>,
}

impl Scene for TriangleScene {
    fn update(&mut self) {
        // Do nothing
    }

    fn draw(&self, gpu: &mut Gpu) {
        gpu.uniforms.model = self.triangle.transform;
        gpu.uniforms.diffuse = crate::assets::textures::GAMERCADE;
        gpu.render_actor(&self.triangle, BaseVertexShader, Textured);
    }
}

impl TriangleScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let actor_id = graphics_db.push_mesh(Mesh {
            vertices: VertexList(shapes::PLANE),
            indices: IndexList(shapes::TRI_INDICES),
            parameters: ParameterData(shapes::PLANE_UVS),
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
