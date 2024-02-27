use glam::Mat4;

use super::Scene;
use crate::{
    actor::Actor,
    graphics::{Gpu, GraphicsDb, IndexList, Mesh, ParameterData, VertexList},
    shaders::{BaseVertexShader, TexturedNormalLit},
    shapes::{self},
};

pub struct CubeScene {
    pub cube: Actor<8>,
}

impl CubeScene {
    pub fn new(graphics_db: &mut GraphicsDb) -> Self {
        let actor_id = graphics_db.push_mesh(Mesh {
            vertices: VertexList(shapes::CUBE),
            indices: IndexList(shapes::CUBE_INDICES),
            parameters: ParameterData(shapes::CUBE_PARAMETERS),
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
        gpu.uniforms.diffuse = crate::assets::textures::BRICKWALL;
        gpu.render_actor(&self.cube, BaseVertexShader, TexturedNormalLit);
    }
}
