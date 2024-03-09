use glam::Mat4;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize)]
pub struct Bone<const MAX_CHILDREN: usize> {
    pub children: [u8; MAX_CHILDREN],
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}

#[derive(Archive, Deserialize)]
pub struct Skeleton<const BONE_COUNT: usize, const MAX_CHILDREN: usize>(
    pub Box<[Bone<MAX_CHILDREN>; BONE_COUNT]>,
);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct BoneRaw {
    pub children: Box<[u8]>,
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}

#[derive(Archive, Serialize)]
pub struct SkeletonRaw(pub Box<[BoneRaw]>);

pub struct SkeletonRawBytes<const BONE_COUNT: usize, const MAX_CHILDREN: usize>(pub &'static [u8]);

impl<const BONE_COUNT: usize, const MAX_CHILDREN: usize>
    SkeletonRawBytes<BONE_COUNT, MAX_CHILDREN>
{
    pub fn as_skeleton(&self) -> &ArchivedSkeleton<BONE_COUNT, MAX_CHILDREN> {
        unsafe { rkyv::archived_root::<Skeleton<BONE_COUNT, MAX_CHILDREN>>(self.0) }
    }
}
