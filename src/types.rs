use gamercade_rs::api::graphics_parameters::GraphicsParameters;
use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        Self {
            r: (value[0].clamp(0.0, 1.0) * 255.0) as u8,
            g: (value[1].clamp(0.0, 1.0) * 255.0) as u8,
            b: (value[2].clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self::from([value.x, value.y, value.z])
    }
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    pub fn to_graphics_params(self) -> GraphicsParameters {
        let a_level = self.r / 8;
        let g_level = self.g / 8;
        let b_level = self.b / 16;

        let g_palette = g_level / 4;
        let g_color = (g_level % 4) * 16;

        let r_palette = a_level * 8;

        GraphicsParameters::default()
            .palette_index(r_palette + g_palette)
            .color_index(g_color + b_level)
    }
}
