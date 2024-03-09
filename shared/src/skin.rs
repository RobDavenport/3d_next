use rkyv::{Archive, Deserialize, Serialize};

// A skin is a collection of bone indices and weights
#[derive(Archive, Serialize, Deserialize)]
pub struct Skin(pub Box<[SkinEntry]>);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct SkinEntry {
    pub bones_indices: Box<[u8]>,
    pub weights: Box<[f32]>,
}

pub struct SkinBytes(pub &'static [u8]);

impl SkinBytes {
    pub fn as_skin(&self) -> &ArchivedSkin {
        unsafe { rkyv::archived_root::<Skin>(self.0) }
    }
}