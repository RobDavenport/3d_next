use glam::Mat4;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct Bone {
    pub parent_index: u8,
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}

#[derive(Archive, Serialize, Deserialize)]
pub struct Skeleton<const BONE_COUNT: usize>(
    pub [Bone; BONE_COUNT],
);

pub struct SkeletonBytes<const BONE_COUNT: usize>(pub &'static [u8]);

impl<const BONE_COUNT: usize> SkeletonBytes<BONE_COUNT> {
    pub fn as_skeleton(&self) -> &ArchivedSkeleton<BONE_COUNT> {
        unsafe { rkyv::archived_root::<Skeleton<BONE_COUNT>>(self.0) }
    }
}
