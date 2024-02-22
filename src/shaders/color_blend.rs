use crate::types::Color;

use super::PixelShader;

#[derive(Default)]
pub struct ColorBlend;

impl PixelShader<3> for ColorBlend {
    fn run(&self, parameters: [f32; 3]) -> Color {
        Color {
            r: (parameters[0].clamp(0.0, 1.0) * 255.0) as u8,
            g: (parameters[1].clamp(0.0, 1.0) * 255.0) as u8,
            b: (parameters[2].clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}
