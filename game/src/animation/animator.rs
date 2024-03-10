use std::array;

use glam::Mat4;
use shared::{animation::ArchivedAnimation, skeleton::ArchivedSkeleton, skin::ArchivedSkin};

#[derive(Clone, Copy)]
pub struct Animator<const BONE_COUNT: usize, const MAX_CHILDREN: usize, const MAX_INFLUENCES: usize>
{
    pub skeleton: &'static ArchivedSkeleton<BONE_COUNT, MAX_CHILDREN>,
    pub skin: &'static ArchivedSkin<MAX_INFLUENCES>,
    pub time: f32,
    pub current_pose: [Mat4; BONE_COUNT],
    pub animation: &'static ArchivedAnimation,
}

impl<const BONE_COUNT: usize, const MAX_CHILDREN: usize, const MAX_INFLUENCES: usize>
    Animator<BONE_COUNT, MAX_CHILDREN, MAX_INFLUENCES>
{
    pub fn new(
        skeleton: &'static ArchivedSkeleton<BONE_COUNT, MAX_CHILDREN>,
        skin: &'static ArchivedSkin<MAX_INFLUENCES>,
        animation: &'static ArchivedAnimation,
    ) -> Self {
        let mut out = Self {
            skeleton,
            skin,
            time: 0.0,
            current_pose: array::from_fn(|_| Mat4::ZERO),
            animation,
        };

        out.current_pose = out.calculate_animation_pose(&out.current_pose);

        out
    }

    pub fn update_time(&mut self, delta: f32) {
        self.time += delta;
    }

    fn calculate_animation_pose(&self, in_pose: &[Mat4; BONE_COUNT]) -> [Mat4; BONE_COUNT] {
        let mut out = [Mat4::IDENTITY; BONE_COUNT];

        for (index, bone) in self.skeleton.0.iter().enumerate() {
            let mut bone_matrix = bone.local_matrix;

            // Apply the animation transformation
            bone_matrix *= in_pose[index];

            // Apply transformations for children bones
            for &child_index in bone.children.iter() {
                if child_index == 0 {
                    continue; // Skip root node
                }
                bone_matrix *= out[child_index as usize];
            }

            // Apply the inverse bind matrix
            bone_matrix *= bone.inverse_bind_matrix;

            // Store the final bone transformation
            out[index] = bone_matrix;
        }

        out
    }
}
