use glam::{Mat4, Quat, Vec3, Vec4};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct Bone {
    pub parent_index: i8,
    pub local_matrix: BoneTrs,
    pub inverse_bind_matrix: Mat4,
}

#[derive(Clone, Copy, Archive, Serialize, Deserialize)]
#[archive_attr(derive(Clone))]
pub struct BoneTrs {
    pub translation: Vec3,
    pub rotation: Vec4,
    pub scale: Vec3,
}

impl ArchivedBoneTrs {
    pub fn as_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, Quat::from_vec4(self.rotation), self.translation)
    }
}

#[derive(Archive, Serialize, Deserialize)]
pub struct Skeleton<const BONE_COUNT: usize>(pub [Bone; BONE_COUNT]);

pub struct SkeletonBytes<const BONE_COUNT: usize>(pub &'static [u8]);

impl<const BONE_COUNT: usize> SkeletonBytes<BONE_COUNT> {
    pub fn as_skeleton(&self) -> &ArchivedSkeleton<BONE_COUNT> {
        unsafe { rkyv::archived_root::<Skeleton<BONE_COUNT>>(self.0) }
    }
}
