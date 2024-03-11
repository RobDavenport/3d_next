use std::array;

use gamercade_rs::api::text::console_log;
use glam::Mat4;
use shared::{animation::ArchivedAnimation, skeleton::ArchivedSkeleton, skin::ArchivedSkin};

#[derive(Clone, Copy)]
pub struct Animator<const BONE_COUNT: usize, const MAX_INFLUENCES: usize> {
    pub skeleton: &'static ArchivedSkeleton<BONE_COUNT>,
    pub skin: &'static ArchivedSkin<MAX_INFLUENCES>,
    pub time: f32,
    pub current_pose: [Mat4; BONE_COUNT],
    pub animation: &'static ArchivedAnimation,
}

impl<const BONE_COUNT: usize, const MAX_INFLUENCES: usize> Animator<BONE_COUNT, MAX_INFLUENCES> {
    pub fn new(
        skeleton: &'static ArchivedSkeleton<BONE_COUNT>,
        skin: &'static ArchivedSkin<MAX_INFLUENCES>,
        animation: &'static ArchivedAnimation,
    ) -> Self {
        let current_pose = array::from_fn(|_| Mat4::IDENTITY);

        Self {
            skeleton,
            skin,
            time: 0.0,
            current_pose,
            animation,
        }
    }

    pub fn update_time(&mut self, delta: f32) {
        self.time += delta;

        let mut new_pose = [Mat4::IDENTITY; BONE_COUNT];

        self.animation.0.iter().for_each(|channel| {
            let mut current_keyframe = 0;

            // TODO: binary search
            for timestamp in channel.timestamps.iter() {
                current_keyframe += 1;
                if self.time < *timestamp {
                    break;
                }
            }

            if current_keyframe >= channel.timestamps.len() {
                self.time = 0.0;
                current_keyframe = channel.timestamps.len() - 1;
            }

            new_pose[channel.target_bone as usize] *= channel.values[current_keyframe]
        });

        self.current_pose = self.calculate_animation_pose(&new_pose);
    }

    // fn calculate_animation_pose(&self, skeleton_pose: &[Mat4; BONE_COUNT]) -> [Mat4; BONE_COUNT] {
    //     let mut model_transforms = [Mat4::IDENTITY; BONE_COUNT];

    //     // Calculate model transforms for each bone
    //     for (index, bone) in self.skeleton.0.iter().enumerate() {
    //         let local_transform = skeleton_pose[index];
    //         let parent_index = bone.parent_index;

    //         if parent_index.is_positive() {
    //             let transformed_local = local_transform * bone.inverse_bind_matrix;
    //             model_transforms[index] =
    //                 model_transforms[parent_index as usize] * transformed_local;
    //         } else {
    //             model_transforms[index] = local_transform;
    //         }
    //     }

    //     model_transforms
    // }

    fn calculate_animation_pose(&self, skeleton_pose: &[Mat4; BONE_COUNT]) -> [Mat4; BONE_COUNT] {
        let mut model_transforms = [Mat4::IDENTITY; BONE_COUNT];

        // Calculate model transforms for each bone
        for (index, bone) in self.skeleton.0.iter().enumerate() {
            let local_transform = skeleton_pose[index];
            let parent_index = bone.parent_index;

            // Handle root or unparented nodes
            if parent_index.is_negative() {
                model_transforms[index] = local_transform;
            } else {
                model_transforms[index] = model_transforms[parent_index as usize] * local_transform;
            }
        }

        array::from_fn(|i| model_transforms[i] * self.skeleton.0[i].inverse_bind_matrix)
    }
}
