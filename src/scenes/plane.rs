use glam::Mat4;

use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::{BaseVertexShader, Textured},
    shapes,
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
        gpu.uniforms.diffuse = crate::assets::textures::GAMERCADE;
        gpu.render_actor(&self.plane, BaseVertexShader, Textured);
    }
}

impl PlaneScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let actor_id = graphics_db.push_mesh(Mesh {
            vertices: VertexList(shapes::PLANE),
            indices: IndexList(shapes::PLANE_INDICES),
            parameters: ParameterData(shapes::PLANE_UVS),
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
