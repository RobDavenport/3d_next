use glam::Vec3A;
use rkyv::{Archive, Deserialize, Serialize};
use types::Color;

pub mod animation;
pub mod mesh;
pub mod shapes;
pub mod skeleton;
pub mod skin;
pub mod texture;
pub mod types;
pub mod vertex_parameters;

pub const SKELETON_MAX_BONES: usize = 256;
pub const SKIN_MAX_BONE_INFLUENCES: usize = 4;
pub const VERTEX_MAX_PARAMETERS: usize = 16;

#[derive(Clone, Copy, Serialize, Deserialize, Archive)]
pub struct TriangleIndices(pub u16, pub u16, pub u16);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct IndexList(pub Box<[TriangleIndices]>);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct VertexList(pub Box<[Vec3A]>);

pub mod bytes {
    pub use super::animation::AnimationBytes;
    pub use super::mesh::MeshBytes;
    pub use super::skeleton::SkeletonBytes;
    pub use super::skin::SkinBytes;
    pub use super::texture::TextureBytes;
}
