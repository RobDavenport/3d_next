use rkyv::{Archive, Deserialize, Serialize};

// A skin is a collection of bone indices and weights
#[derive(Archive, Deserialize)]
pub struct Skin<const MAX_BONES: usize>(pub Box<[SkinEntry<MAX_BONES>]>);

#[derive(Archive, Deserialize)]
pub struct SkinEntry<const MAX_BONES: usize> {
    pub bones_indices: [u32; MAX_BONES],
    pub weights: [f32; MAX_BONES],
}

#[derive(Archive, Serialize)]
pub struct SkinRaw(pub Box<[SkinEntryRaw]>);

#[derive(Clone, Archive, Serialize)]
pub struct SkinEntryRaw {
    pub bone_indices: Box<[u8]>,
    pub weights: Box<[f32]>,
}

pub struct SkinRawBytes<const MAX_BONES: usize>(pub &'static [u8]);

impl<const MAX_BONES: usize> SkinRawBytes<MAX_BONES> {
    pub fn as_skin(&self) -> &ArchivedSkin<MAX_BONES> {
        unsafe { rkyv::archived_root::<Skin<MAX_BONES>>(self.0) }
    }
}
