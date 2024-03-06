use bytemuck::{Pod, Zeroable};
use glam::Mat4;

// A skin is a collection of bone indices and weights
pub struct Skin<const B: usize>(pub &'static [SkinEntry<B>]);

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
pub struct SkinEntry<const B: usize> {
    pub bones_indices: [u32; B],
    pub weights: [f32; B],
}

// A Skeleton is a collection of bones
pub struct Skeleton<const C: usize>(pub &'static [Bone<C>]);

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
pub struct Bone<const C: usize> {
    pub children: [u32; C],
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}
