use std::array;

use glam::{Mat4, Vec4};
use shared::{skeleton::ArchivedSkeleton, skin::ArchivedSkin};

#[derive(Clone, Copy)]
pub struct Animator<const BONE_COUNT: usize, const MAX_CHILDREN: usize>
{
    pub skeleton: &'static ArchivedSkeleton<BONE_COUNT, MAX_CHILDREN>,
    pub skin: &'static ArchivedSkin,
    pub time: f32,
    pub current_pose: [Mat4; BONE_COUNT],
}

impl<const BONE_COUNT: usize, const MAX_CHILDREN: usize>
    Animator<BONE_COUNT, MAX_CHILDREN>
{
    pub fn new(skeleton: &'static ArchivedSkeleton<BONE_COUNT, MAX_CHILDREN>, skin: &'static ArchivedSkin) -> Self {
        let mut out  =Self {
            skeleton,
            skin,
            time: 0.0,
            current_pose: array::from_fn(|_| Mat4::IDENTITY)
        };

        out.current_pose = out.calculate_animation_pose(&out.current_pose);
        out
    }

    // pub fn skin_vertices(&self, vertices: &mut [Vec4], in_pose: &[Mat4; BONE_COUNT]) {
    //     let pose = self.calculate_animation_pose(in_pose);

    //     for (vertex, skin) in vertices.iter_mut().zip(self.skin.0.iter()) {
    //         for (index, weight) in skin.bones_indices.into_iter().zip(skin.weights.into_iter()) {
    //             let matrix = pose[*index as usize];
    //             *vertex = matrix.mul_scalar(weight) * *vertex;
    //         }
    //     }
    // }

    fn calculate_animation_pose(&self, in_pose: &[Mat4; BONE_COUNT]) -> [Mat4; BONE_COUNT] {
        let mut out = array::from_fn(|index| self.skeleton.0[index].local_matrix * in_pose[index]);

        for index in 0..BONE_COUNT {
            let children = self.skeleton.0[index].children;
            let parent = self.skeleton.0[index].local_matrix;

            for child_index in children.into_iter() {
                // Root node can't be a child, so it's an unused index
                if child_index == 0 {
                    continue;
                }

                out[child_index as usize] *= parent;
            }
        }

        for (out_mat, bone) in out.iter_mut().zip(self.skeleton.0.iter()) {
            *out_mat *= bone.inverse_bind_matrix
        }

        out
    }
}
