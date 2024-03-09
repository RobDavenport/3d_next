use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use rkyv::{Archive, Deserialize, Serialize};
use types::Color;

pub mod mesh;
pub mod shapes;
pub mod texture;
pub mod types;
pub mod vertex_parameters;

#[derive(Clone, Copy, Pod, Zeroable, Serialize, Deserialize, Archive)]
#[repr(C)]
pub struct TriangleIndices(pub u16, pub u16, pub u16);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct IndexList(pub Vec<TriangleIndices>);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct VertexList(pub Vec<Vec3>);
