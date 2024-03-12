use glam::Mat4;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct Bone {
    pub parent_index: i8,
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}


#[derive(Archive, Serialize, Deserialize)]
pub struct Skeleton<const BONE_COUNT: usize>(pub [Bone; BONE_COUNT]);

// impl<const BC: usize> ArchivedSkeleton<BC> {
//     pub fn convert_to_parent_space(&self, index: usize) -> Mat4 {
//         let bone = &self.0[index];
//         if bone.parent_index.is_negative() {
//             bone.inverse_bind_matrix.inverse()
//         } else {
//             let parent_index = bone.parent_index as usize;
//             self.0[parent_index].inverse_bind_matrix * bone.inverse_bind_matrix.inverse()
//         }
//     }
// }

pub struct SkeletonBytes<const BONE_COUNT: usize>(pub &'static [u8]);

impl<const BONE_COUNT: usize> SkeletonBytes<BONE_COUNT> {
    pub fn as_skeleton(&self) -> &ArchivedSkeleton<BONE_COUNT> {
        unsafe { rkyv::archived_root::<Skeleton<BONE_COUNT>>(self.0) }
    }
}
