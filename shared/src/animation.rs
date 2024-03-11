use glam::Mat4;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize)]
pub struct Animation(pub Box<[AnimationChannel]>);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct AnimationChannel {
    pub channel_type: AnimationChannelType,
    pub interpolation_type: AnimationInterprolationType,
    pub target_bone: i8,
    pub timestamps: Vec<f32>,
    pub values: Vec<Mat4>,
}

#[derive(Debug, Clone, Copy, Archive, Serialize, Deserialize)]
pub enum AnimationChannelType {
    Translation,
    Rotation,
    Scale,
}

#[derive(Debug, Clone, Copy, Archive, Serialize, Deserialize)]
pub enum AnimationInterprolationType {
    Linear,
    Step,
    CubicSpline,
}

pub struct AnimationBytes(pub &'static [u8]);

impl AnimationBytes {
    pub fn as_anim(&self) -> &ArchivedAnimation {
        unsafe { rkyv::archived_root::<Animation>(self.0) }
    }
}
