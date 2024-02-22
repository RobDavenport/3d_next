use glam::Vec3;

use crate::types::Color;

use super::PixelShader;

#[derive(Default)]
pub struct ColorBlend;

impl PixelShader<3> for ColorBlend {
    fn run(&self, parameters: [f32; 3]) -> Color {
        Color::from(parameters)
    }
}

#[derive(Default)]
pub struct ColorBlendLit {
    pub light_position: Vec3,
    pub light_intensity: f32,
    pub ambient_light: f32,
}

impl PixelShader<9> for ColorBlendLit {
    fn run(&self, parameters: [f32; 9]) -> Color {
        // Shader Setup
        let [r, g, b, norm_x, norm_y, norm_z, pixel_x, pixel_y, pixel_z] = parameters;
        let pixel_position = Vec3::new(pixel_x, pixel_y, pixel_z);
        let normal = Vec3::new(norm_x, norm_y, norm_z);
        let object_color = Vec3::new(r, g, b);

        let pixel_to_light = (self.light_position - pixel_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * self.light_intensity, 0.0);

        Color::from(object_color * (light_factor + self.ambient_light))
    }
}
