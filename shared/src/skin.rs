use rkyv::{Archive, Deserialize, Serialize};

// A skin is a collection of bone indices and weights
#[derive(Archive, Serialize, Deserialize)]
pub struct Skin<const MAX_INFLUENCES: usize>(pub Box<[SkinEntry<MAX_INFLUENCES>]>);

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct SkinEntry<const MAX_INFLUENCES: usize> {
    pub bones_indices: [u8; MAX_INFLUENCES],
    pub weights: [f32; MAX_INFLUENCES],
}

pub struct SkinBytes<const MAX_INFLUENCES: usize>(pub &'static [u8]);

impl<const MAX_INFLUENCES: usize> SkinBytes<MAX_INFLUENCES> {
    pub fn as_skin(&self) -> &ArchivedSkin<MAX_INFLUENCES> {
        unsafe { rkyv::archived_root::<Skin<MAX_INFLUENCES>>(self.0) }
    }
}
