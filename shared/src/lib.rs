use glam::Vec3;
use rkyv::{Archive, Deserialize, Serialize};
use types::Color;

pub mod mesh;
pub mod shapes;
pub mod skeleton;
pub mod skin;
pub mod texture;
pub mod types;
pub mod vertex_parameters;

pub const SKELETON_MAX_BONES: usize = 64;
pub const SKELETON_MAX_CHILDREN: usize = 8;
pub const SKIN_MAX_BONE_INFLUENCES: usize = 4;
pub const VERTEX_MAX_PARAMETERS: usize = 16;

#[derive(Clone, Copy, Serialize, Deserialize, Archive)]
pub struct TriangleIndices(pub u16, pub u16, pub u16);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct IndexList(pub Box<[TriangleIndices]>);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct VertexList(pub Box<[Vec3]>);
