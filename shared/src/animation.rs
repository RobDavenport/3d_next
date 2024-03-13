use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize)]
pub struct Animation {
    pub length: f32,
    pub channels: Box<[AnimationChannel]>,
}

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct AnimationChannel {
    pub channel_type: AnimationChannelType,
    pub interpolation_type: AnimationInterprolationType,
    pub target_bone: i8,
    pub timestamps: Vec<f32>,
    pub values: Vec<f32>,
}

#[derive(Debug, Clone, Copy, Archive, Serialize, Deserialize)]
pub enum AnimationChannelType {
    Translation,
    Rotation,
    Scale,
}

#[derive(Debug, Clone, Copy, Archive, Serialize, Deserialize)]
pub enum AnimationInterprolationType {
    Step,
    Linear,
}

pub struct AnimationBytes(pub &'static [u8]);

impl AnimationBytes {
    pub fn as_anim(&self) -> &ArchivedAnimation {
        unsafe { rkyv::archived_root::<Animation>(self.0) }
    }
}
