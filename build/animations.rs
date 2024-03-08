use glam::{Quat, Vec4};
use gltf::{
    animation::{Interpolation, Property},
    Animation,
};

use crate::*;

pub struct AnimationOutput {
    pub name: String,
    channels: Vec<AnimationChannel>,
}

enum AnimationChannelType {
    Translation,
    Rotation,
    Scale,
}

enum AnimationInterprolationType {
    Linear,
    Step,
    CubicSpline,
}

pub struct AnimationChannel {
    channel_type: AnimationChannelType,
    interpolation_type: AnimationInterprolationType,
    target_bone: u32,
    keyframes: usize,
    timestamps: Vec<f32>,
    values: Vec<Mat4>,
}

pub fn generate_animation(animation: &Animation, blob: &[u8]) {
    let name = animation.name().unwrap_or("Unnamed");
    println!("Animation found: {name}.");

    // Bone Index -> List of Transforms
    let mut animation_channels = Vec::new();

    for channel in animation.channels() {
        let mut modifiers = Vec::new();
        let target = channel.target();
        let target_index = target.node().index();
        let sampler = channel.sampler();

        // Get Input Keyframes
        let input_accessor = sampler.input();
        let input_view = input_accessor.view().unwrap();
        let start = input_view.offset() + input_accessor.offset();
        let end = start + input_accessor.count() * input_accessor.size();
        let input = &blob[start..end];
        let input: &[f32] = cast_slice(input);

        let keyframes = input.to_vec();
        let keyframe_count = keyframes.len();

        // Get outputs
        let output_accessor = sampler.output();
        let output_view = output_accessor.view().unwrap();
        let start = output_view.offset() + output_accessor.offset();
        let end = start + output_accessor.count() * output_accessor.size();
        let output = &blob[start..end];

        //let data_type = output_accessor.data_type();
        let dimensions = output_accessor.dimensions();
        let property = target.property();

        let output: &[f32] = cast_slice(output);
        let output = output.chunks_exact(dimensions.multiplicity());

        let interpolate = match sampler.interpolation() {
            Interpolation::Linear => AnimationInterprolationType::Linear,
            Interpolation::Step => AnimationInterprolationType::Step,
            Interpolation::CubicSpline => AnimationInterprolationType::CubicSpline,
        };

        let channel_type = match property {
            Property::Translation => AnimationChannelType::Translation,
            Property::Rotation => AnimationChannelType::Rotation,
            Property::Scale => AnimationChannelType::Scale,
            Property::MorphTargetWeights => panic!("Morph target weights not implemented"),
        };

        output.into_iter().for_each(|chunk| {
            let matrix = match property {
                Property::Translation => Mat4::from_translation(Vec3::from_slice(chunk)),
                Property::Rotation => Mat4::from_quat(Quat::from_vec4(Vec4::from_slice(chunk))),
                Property::Scale => Mat4::from_scale(Vec3::from_slice(chunk)),
                Property::MorphTargetWeights => panic!("Morph target weights not implemented"),
            };

            modifiers.push(matrix)
        });

        let out = AnimationChannel {
            channel_type,
            interpolation_type: interpolate,
            target_bone: target_index as u32,
            keyframes: keyframe_count,
            timestamps: keyframes,
            values: modifiers,
        };

        animation_channels.push(out);
    }

    println!("channel_count: {}", animation.channels().count());
}
