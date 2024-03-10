use std::{array};

use gamercade_rs::api::text::console_log;
use glam::Mat4;
use shared::{animation::{ArchivedAnimation, ArchivedAnimationChannelType}, skeleton::{ArchivedBone, ArchivedSkeleton, Bone}, skin::ArchivedSkin};

#[derive(Clone, Copy)]
pub struct Animator<const BONE_COUNT: usize, const MAX_INFLUENCES: usize>
{
    pub skeleton: &'static ArchivedSkeleton<BONE_COUNT>,
    pub skin: &'static ArchivedSkin<MAX_INFLUENCES>,
    pub time: f32,
    pub current_pose: [Mat4; BONE_COUNT],
    pub animation: &'static ArchivedAnimation,
}

impl<const BONE_COUNT: usize, const MAX_INFLUENCES: usize>
    Animator<BONE_COUNT, MAX_INFLUENCES>
{
    pub fn new(
        skeleton: &'static ArchivedSkeleton<BONE_COUNT>,
        skin: &'static ArchivedSkin<MAX_INFLUENCES>,
        animation: &'static ArchivedAnimation,
    ) -> Self {
        let mut out = Self {
            skeleton,
            skin,
            time: 0.0,
            current_pose: array::from_fn(|_| Mat4::IDENTITY),
            animation,
        };

        out.current_pose = out.calculate_animation_pose(&out.current_pose);

        out
    }

    pub fn update_time(&mut self, delta: f32) {
        self.time += delta;

        let mut new_pose = [Mat4::IDENTITY; BONE_COUNT];

        self.animation.0.iter().for_each(|channel| {
            let mut current_keyframe = 0;

            for timestamp in channel.timestamps.iter() {
                current_keyframe += 1;
                if self.time < *timestamp {
                    break;
                }
            }

            if current_keyframe >= channel.timestamps.len() {
                self.time = 0.0;
                current_keyframe = 0;
            }

            new_pose[channel.target_bone as usize] *= channel.values[current_keyframe]
        });

        self.current_pose = self.calculate_animation_pose(&new_pose);
    }
    

    fn calculate_animation_pose(&self, in_pose: &[Mat4; BONE_COUNT]) -> [Mat4; BONE_COUNT] {
        let mut model_transform: [Mat4; BONE_COUNT] = [Mat4::IDENTITY; BONE_COUNT];
    
        // Calculate model transforms for each bone
        model_transform[0] = self.skeleton.0[0].local_matrix;

        for (index, bone) in self.skeleton.0.iter().enumerate().skip(1) {
            let local_transform = bone.local_matrix * in_pose[index];
            let parent_index = bone.parent_index as usize;
            model_transform[index] = model_transform[parent_index] * local_transform;
        }
    
        // Apply inverse bind matrices to model transforms
        let mut animation_pose: [Mat4; BONE_COUNT] = [Mat4::IDENTITY; BONE_COUNT];
        for index in 0..BONE_COUNT {
            // Multiply each model transform by its corresponding inverse bind matrix
            animation_pose[index] = model_transform[index] * self.skeleton.0[index].inverse_bind_matrix;
        }
    
        animation_pose
    }
}
