use crate::Color;
use glam::{UVec3, Vec3A};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Archive)]
pub struct Texture {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct TextureBytes(pub &'static [u8]);

impl TextureBytes {
    pub fn as_texture(&self) -> &ArchivedTexture {
        unsafe { rkyv::archived_root::<Texture>(self.0) }
    }
}

impl ArchivedTexture {
    const STRIDE: usize = 3;

    pub fn get_index(&self, u: f32, v: f32) -> usize {
        let u = (u.abs().fract() * self.width as f32) as usize;
        let v = (v.abs().fract() * self.height as f32) as usize;

        ((v * self.width as usize) + u) * Self::STRIDE
    }

    pub fn index_veca(&self, index: usize) -> Vec3A {
        let rgb = &self.data[index..index + Self::STRIDE];
        UVec3::new(rgb[0] as u32, rgb[1] as u32, rgb[2] as u32).as_vec3a() / u8::MAX as f32
    }

    pub fn index_color(&self, index: usize) -> Color {
        let slice = &self.data[index..index + Self::STRIDE];
        Color::new(slice[0], slice[1], slice[2])
    }

    /// Simpler convenience functions, prefer get_index variants for performance
    pub fn sample_vec(&self, u: f32, v: f32) -> Vec3A {
        let index = self.get_index(u, v);
        self.index_veca(index)
    }

    /// Simpler convenience functions, prefer get_index variants for performance
    pub fn sample_color(&self, u: f32, v: f32) -> Color {
        let index = self.get_index(u, v);
        self.index_color(index)
    }
}
