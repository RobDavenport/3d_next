use bytemuck::{Pod, Zeroable};
use glam::Mat4;

// A skin is a collection of bone indices and weights
pub struct Skin<const B: usize>(pub &'static [SkinEntry<B>]);

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
pub struct SkinEntry<const MAX_BONES: usize> {
    pub bones_indices: [u32; MAX_BONES],
    pub weights: [f32; MAX_BONES],
}

// A Skeleton is a collection of bones
pub struct Skeleton<const BONE_COUNT: usize, const MAX_CHILDREN: usize> {
    pub matrices: &'static [BoneMatrices],
    pub children: &'static [BoneChildren<MAX_CHILDREN>],
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct BoneChildren<const MAX_CHILDREN: usize>(pub [u32; 4]);

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BoneMatrices {
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}
