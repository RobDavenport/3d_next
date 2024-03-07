use std::array;

use glam::{Mat4, Vec4};

use super::{Skeleton, Skin};

pub struct Animator<const BONE_COUNT: usize, const MAX_CHILDREN: usize, const MAX_INFLUENCES: usize>
{
    pub skeleton: Skeleton<BONE_COUNT, MAX_CHILDREN>,
    pub skin: Skin<MAX_INFLUENCES>,
}

impl<const BONE_COUNT: usize, const MAX_CHILDREN: usize, const MAX_INFLUENCES: usize>
    Animator<BONE_COUNT, MAX_CHILDREN, MAX_INFLUENCES>
{
    pub fn skin_vertices(&self, vertices: &mut [Vec4], in_pose: &[Mat4; BONE_COUNT]) {
        let pose = self.skeleton.calculate_animation_pose(in_pose);

        for (vertex, skin) in vertices.iter_mut().zip(self.skin.0.iter()) {
            for (index, weight) in skin.bones_indices.into_iter().zip(skin.weights.into_iter()) {
                let matrix = pose[index as usize];
                *vertex = matrix.mul_scalar(weight) * *vertex;
            }
        }
    }
}

impl<const BONE_COUNT: usize, const MAX_CHILDREN: usize> Skeleton<BONE_COUNT, MAX_CHILDREN> {
    fn calculate_animation_pose(&self, in_pose: &[Mat4; BONE_COUNT]) -> [Mat4; BONE_COUNT] {
        let mut out = array::from_fn(|index| self.matrices[index].local_matrix * in_pose[index]);

        for index in 0..BONE_COUNT {
            let children = self.children[index];
            let parent = self.matrices[index].local_matrix;

            for child_index in children.0.into_iter() {
                if child_index == 0 {
                    continue;
                }

                out[child_index as usize] *= parent;
            }
        }

        for index in 0..BONE_COUNT {
            out[index] *= self.matrices[index].inverse_bind_matrix
        }

        out
    }
}
