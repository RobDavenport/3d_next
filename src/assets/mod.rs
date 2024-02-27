mod generated;
pub use generated::*;

use crate::{
    graphics::{IndexList, Mesh, VertexList, VertexParametersList},
    types::Color,
};

use bytemuck::cast_slice;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: &'static [u8],
}

impl Texture {
    const STRIDE: usize = 3;

    pub fn get_sample(&self, u: f32, v: f32) -> Color {
        let u = (u * (self.width - 1) as f32).clamp(0.0, (self.width - 1) as f32) as usize;
        let v = (v * (self.height - 1) as f32).clamp(0.0, (self.height - 1) as f32) as usize;

        let index = ((v * self.width) + u) * Self::STRIDE;
        let slice = &self.data[index..index + Self::STRIDE];
        Color::new(slice[0], slice[1], slice[2])
    }
}

pub struct StaticMesh<const P: usize> {
    pub vertices: &'static [u8],
    pub indices: &'static [u8],
    pub parameters: &'static [u8],
}

impl<const P: usize> StaticMesh<P> {
    pub fn as_mesh(&self) -> Mesh<P> {
        Mesh {
            vertices: VertexList(cast_slice(self.vertices)),
            indices: IndexList(cast_slice(self.indices)),
            parameters: VertexParametersList(cast_slice(self.parameters)),
        }
    }
}
