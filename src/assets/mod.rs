mod generated;
pub use generated::*;

use glam::{UVec3, Vec3};

use crate::{
    animation::{Skeleton, Skin},
    graphics::{IndexList, StaticMesh, VertexList, VertexParametersList},
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

    pub fn get_index(&self, u: f32, v: f32) -> usize {
        let u = (u.abs().fract() * self.width as f32) as usize;
        let v = (v.abs().fract() * self.height as f32) as usize;

        ((v * self.width) + u) * Self::STRIDE
    }

    pub fn index_vec(&self, index: usize) -> Vec3 {
        let rgb = &self.data[index..index + Self::STRIDE];
        UVec3::new(rgb[0] as u32, rgb[1] as u32, rgb[2] as u32).as_vec3() / u8::MAX as f32
    }

    pub fn index_color(&self, index: usize) -> Color {
        let slice = &self.data[index..index + Self::STRIDE];
        Color::new(slice[0], slice[1], slice[2])
    }

    /// Simpler convenience functions, prefer get_index variants for performance
    pub fn sample_vec(&self, u: f32, v: f32) -> Vec3 {
        let index = self.get_index(u, v);
        self.index_vec(index)
    }

    /// Simpler convenience functions, prefer get_index variants for performance
    pub fn sample_color(&self, u: f32, v: f32) -> Color {
        let index = self.get_index(u, v);
        self.index_color(index)
    }
}

pub struct StaticMeshData<const P: usize> {
    pub vertices: &'static [u8],
    pub indices: &'static [u8],
    pub parameters: &'static [u8],
}

impl<const P: usize> StaticMeshData<P> {
    pub fn as_mesh(&self) -> StaticMesh<P> {
        let vertices = VertexList(cast_slice(self.vertices));
        let indices = IndexList(cast_slice(self.indices));
        let parameters = VertexParametersList(cast_slice(self.parameters));

        StaticMesh {
            vertices,
            indices,
            parameters,
        }
    }
}

pub struct SkinData<const B: usize> {
    skin_entries: &'static [u8],
}

impl<const B: usize> SkinData<B> {
    pub fn as_skin(&self) -> Skin<B> {
        Skin(cast_slice(self.skin_entries))
    }
}

pub struct SkeletonData<const C: usize> {
    bones: &'static [u8],
}

impl<const C: usize> SkeletonData<C> {
    pub fn as_skeleton(&self) -> Skeleton<C> {
        Skeleton(cast_slice(self.bones))
    }
}
