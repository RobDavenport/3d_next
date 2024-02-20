use std::ops::{Add, Mul, MulAssign};

use glam::{Vec2, Vec3};

use crate::{image, types::Color};

pub trait PixelShaderInput:
    Copy + Add<Self, Output = Self> + Mul<f32, Output = Self> + MulAssign<f32>
{
}

impl<T> PixelShaderInput for T where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T> + MulAssign<f32>
{
}

pub trait PixelShader<PSIN: PixelShaderInput> {
    fn run(&self, parameters: PSIN) -> Color;
}

#[derive(Default)]
pub struct ColorBlend;

impl PixelShader<Vec3> for ColorBlend {
    fn run(&self, parameters: Vec3) -> Color {
        Color {
            r: (parameters.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (parameters.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (parameters.z.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

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

impl PixelShader<Vec2> for Textured {
    fn run(&self, parameters: Vec2) -> Color {
        self.sample_2d(parameters.x, parameters.y)
    }
}
