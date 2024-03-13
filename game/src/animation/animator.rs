use std::array;

use gamercade_rs::{api::text::console_log, prelude as gc};
use glam::{Mat4, Vec3, Vec4};
use shared::{animation::{ArchivedAnimation, ArchivedAnimationChannelType}, skeleton::{ArchivedBoneTrs, ArchivedSkeleton, BoneTrs}, skin::ArchivedSkin};

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

        // Set default values
        let mut new_pose: [ArchivedBoneTrs; BONE_COUNT] = array::from_fn(|i| {
            self.skeleton.0[i].local_matrix.clone()
        });

        //Find the animation channel, and combine the outputs
        self.animation.0.iter().for_each(|channel| {
            let mut current_keyframe = 0;
            let target_bone = channel.target_bone as usize;

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

            // Accumulte the individual channel transforms
            // TODO: Lerp this
            let target = &mut new_pose[target_bone];

            match channel.channel_type {
                ArchivedAnimationChannelType::Translation => {
                    target.translation = Vec3::from_slice(&channel.values[current_keyframe * 3..]);
                },
                ArchivedAnimationChannelType::Rotation => {
                    target.rotation = Vec4::from_slice(&channel.values[current_keyframe * 4..])
                },
                ArchivedAnimationChannelType::Scale => {
                    target.scale = Vec3::from_slice(&channel.values[current_keyframe * 3..])
                },
            }
        });

        // Combine the animations for parent/child relationship
        for i in 0..BONE_COUNT {
            let local_matrix = new_pose[i].as_matrix();
            if self.skeleton.0[i].parent_index.is_negative() {
                self.current_pose[i] = local_matrix;
            } else {
                let parent_matrix = self.current_pose[self.skeleton.0[i].parent_index as usize];
                self.current_pose[i] = parent_matrix * local_matrix;
            }
        }

        // Premultiply here to avoid doing it in the vertex shader
        for (mat, bone) in self.current_pose.iter_mut().zip(self.skeleton.0.iter()) {
            *mat = *mat * bone.inverse_bind_matrix
        }
    }
}
