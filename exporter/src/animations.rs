use bytemuck::cast_slice;
use glam::{Quat, Vec4};
use gltf::animation::{Interpolation, Property};
use shared::animation::{
    Animation, AnimationChannel, AnimationChannelType, AnimationInterprolationType,
};

use self::skeleton::SkeletonMetaData;
use crate::*;

pub struct AnimationOutputVec {
    pub name: String,
    channels: Vec<AnimationChannel>,
}

impl AnimationOutputVec {
    fn to_output(&self) -> String {
        let filename = format!("{}_{ANIMATION_EXTENSION}", self.name);

        let out = Animation(self.channels.clone().into_boxed_slice());
        let archive = rkyv::to_bytes::<_, 256>(&out).unwrap();
        write_file(&filename, &archive);

        let name = filename.to_uppercase();
        format!(
            "pub static {name}: &AnimationBytes = &AnimationBytes(include_bytes!(\"{filename}\"));\n"
        )
    }
}

pub fn generate_animation(
    animation: &gltf::Animation,
    blob: &[u8],
    metadata: &SkeletonMetaData,
    filename: &str,
    root_transform: Mat4,
) -> String {
    let name = animation.name().unwrap_or("Unnamed");
    let name = format!("{filename}_{name}");
    println!("Animation found: {name}.");

    // Bone Index -> List of Transforms
    let mut animation_channels = Vec::new();

    let named_bones = &metadata.named_bones;

    for channel in animation.channels() {
        let mut modifiers = Vec::new();
        let target = channel.target();
        let target_index = *named_bones.get(target.node().name().unwrap()).unwrap();
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

        if let Some(stride) = output_view.stride() {
            panic!("ANIMATION OUTPUT HAS STRIDE: {stride}");
        };

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
                Property::Rotation => Mat4::from_quat(Quat::from_vec4(Vec4::from_slice(chunk).normalize())),
                Property::Scale => Mat4::from_scale(Vec3::from_slice(chunk)),
                Property::MorphTargetWeights => panic!("Morph target weights not implemented"),
            };

            modifiers.push(matrix)
        });

        let out = AnimationChannel {
            channel_type,
            interpolation_type: interpolate,
            target_bone: target_index,
            timestamps: keyframes,
            values: modifiers,
        };

        animation_channels.push(out);
    }

    AnimationOutputVec {
        name: name.to_owned(),
        channels: animation_channels,
    }
    .to_output()
}
