use std::{array, cmp::Ordering};

use glam::{Mat4, Quat, Vec3, Vec4};
use shared::{
    animation::{
        ArchivedAnimation, ArchivedAnimationChannelType, ArchivedAnimationInterprolationType,
    },
    skeleton::{ArchivedBoneTrs, ArchivedSkeleton, BoneTrs},
    skin::ArchivedSkin,
};

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

        if self.time >= self.animation.length {
            self.time -= self.animation.length
        }

        // Set default values
        let mut new_pose: [ArchivedBoneTrs; BONE_COUNT] =
            array::from_fn(|i| self.skeleton.0[i].local_matrix.clone());

        //Find the animation channel, and combine the outputs
        self.animation.channels.iter().for_each(|channel| {
            let target_bone = channel.target_bone as usize;

            let current_keyframe = match channel.timestamps.binary_search_by(|timestamp| {
                timestamp.partial_cmp(&self.time).unwrap_or(Ordering::Less)
            }) {
                Ok(current_keyframe) => current_keyframe,
                Err(index) => {
                    if index == 0 {
                        index
                    } else {
                        index - 1
                    }
                }
            };

            // Accumulte the individual channel transforms
            let target = &mut new_pose[target_bone];

            match channel.interpolation_type {
                ArchivedAnimationInterprolationType::Step => match channel.channel_type {
                    ArchivedAnimationChannelType::Translation => {
                        let value = Vec3::from_slice(&channel.values[current_keyframe * 3..]);
                        target.translation = value;
                    }
                    ArchivedAnimationChannelType::Rotation => {
                        let value = Vec4::from_slice(&channel.values[current_keyframe * 4..]);
                        target.rotation = value;
                    }
                    ArchivedAnimationChannelType::Scale => {
                        let value = Vec3::from_slice(&channel.values[current_keyframe * 3..]);
                        target.scale = value;
                    }
                },
                ArchivedAnimationInterprolationType::Linear => {
                    let start_keyframe = current_keyframe;
                    let end_keyframe = if start_keyframe == channel.timestamps.len() - 1 {
                        0 // Loop back to the first keyframe if at the end
                    } else {
                        start_keyframe + 1
                    };
                    let t = (self.time - channel.timestamps[start_keyframe])
                        / (channel.timestamps[end_keyframe] - channel.timestamps[start_keyframe]);
                    match channel.channel_type {
                        ArchivedAnimationChannelType::Translation => {
                            let first = Vec3::from_slice(&channel.values[start_keyframe * 3..]);
                            let second = Vec3::from_slice(&channel.values[end_keyframe * 3..]);
                            let value = first.lerp(second, t);
                            target.translation = value;
                        }
                        ArchivedAnimationChannelType::Rotation => {
                            let first = Quat::from_slice(&channel.values[start_keyframe * 4..]);
                            let second = Quat::from_slice(&channel.values[end_keyframe * 4..]);
                            let value = first.slerp(second, t).into();
                            target.rotation = value
                        }
                        ArchivedAnimationChannelType::Scale => {
                            let first = Vec3::from_slice(&channel.values[start_keyframe * 3..]);
                            let second = Vec3::from_slice(&channel.values[end_keyframe * 3..]);
                            let value = first.lerp(second, t);
                            target.scale = value
                        }
                    }
                }
            };
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
