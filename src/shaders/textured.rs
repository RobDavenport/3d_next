use crate::{image, types::Color};

use super::PixelShader;

pub struct Textured {
    width: usize,
    height: usize,
}

impl Default for Textured {
    fn default() -> Self {
        Self {
            width: image::IMAGE_WIDTH,
            height: image::IMAGE_HEIGHT,
        }
    }
}

impl Textured {
    fn sample_2d(&self, u: f32, v: f32) -> Color {
        let u = (u * (self.width - 1) as f32) as usize;
        let v = (v * (self.height - 1) as f32) as usize;

        let u = u.clamp(0, self.width - 1);
        let v = v.clamp(0, self.height - 1);

        image::get_image()[(v * self.width) + u]
    }
}

impl PixelShader<2> for Textured {
    fn run(&self, parameters: [f32; 2]) -> Color {
        self.sample_2d(parameters[0], parameters[1])
    }
}
