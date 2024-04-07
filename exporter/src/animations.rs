use bytemuck::cast_slice;
use gltf::animation::{Interpolation, Property};
use shared::animation::{
    Animation, AnimationChannel, AnimationChannelType, AnimationInterprolationType,
};

use self::skeleton::SkeletonMetaData;
use crate::*;

pub struct AnimationOutputVec {
    pub name: String,
    channels: Vec<AnimationChannel>,
    length: f32,
}

impl AnimationOutputVec {
    fn to_output(&self, config: &AssetList) -> String {
        let filename = format!("{}_{ANIMATION_EXTENSION}", self.name);

        let out = Animation {
            length: self.length,
            channels: self.channels.clone().into_boxed_slice(),
        };
        let archive = rkyv::to_bytes::<_, 256>(&out).unwrap();
        write_file(config, &filename, &archive);

        let name = filename.to_uppercase();
        format!(
            "pub const {name}: &AnimationBytes = &AnimationBytes(include_bytes!(\"{filename}\"));\n"
        )
    }
}

pub fn generate_animation(
    config: &AssetList,
    animation: &gltf::Animation,
    blob: &[u8],
    metadata: &SkeletonMetaData,
    filename: &str,
) -> String {
    let name = animation.name().unwrap_or("Unnamed");
    let name = format!("{filename}_{name}");
    println!("Animation found: {name}.");

    // Bone Index -> List of Transforms
    let mut animation_channels = Vec::new();

    let index_to_bone = &metadata.node_to_index;

    for channel in animation.channels() {
        let target = channel.target();
        let target_index = *index_to_bone.get(&target.node().index()).unwrap();
        let sampler = channel.sampler();

        // Get Input Keyframes
        let input_accessor = sampler.input();
        let input_view = input_accessor.view().unwrap();
        let start = input_view.offset() + input_accessor.offset();
        let end = start + input_accessor.count() * input_accessor.size();
        let input = &blob[start..end];
        let input: &[f32] = cast_slice(input);

        if let Some(stride) = input_view.stride() {
            panic!("ANIMATION INPUT HAS STRIDE: {stride}");
        };

        let keyframes = input.to_vec();

        // Get outputs
        let output_accessor = sampler.output();
        let output_view = output_accessor.view().unwrap();
        let start = output_view.offset() + output_accessor.offset();
        let end = start + output_accessor.count() * output_accessor.size();
        let output = &blob[start..end];
        if sampler.output().sparse().is_some() {
            panic!("Sparse animation not handled")
        };

        if let Some(stride) = output_view.stride() {
            panic!("ANIMATION OUTPUT HAS STRIDE: {stride}");
        };

        let property = target.property();

        let output: &[f32] = cast_slice(output);
        let values = output.to_vec();

        let interpolate = match sampler.interpolation() {
            Interpolation::Linear => AnimationInterprolationType::Linear,
            Interpolation::Step => AnimationInterprolationType::Step,
            Interpolation::CubicSpline => panic!("Cubic Spline animation is not supported"),
        };

        let channel_type = match property {
            Property::Translation => AnimationChannelType::Translation,
            Property::Rotation => AnimationChannelType::Rotation,
            Property::Scale => AnimationChannelType::Scale,
            Property::MorphTargetWeights => panic!("Morph target weights not implemented"),
        };

        let out = AnimationChannel {
            channel_type,
            interpolation_type: interpolate,
            target_bone: target_index,
            timestamps: keyframes,
            values,
        };

        animation_channels.push(out);
    }

    let mut length: f32 = 0.0;

    for channel in animation_channels.iter() {
        let max = channel.timestamps.last().unwrap();
        length = length.max(*max)
    }

    AnimationOutputVec {
        name: name.to_owned(),
        channels: animation_channels,
        length,
    }
    .to_output(config)
}
