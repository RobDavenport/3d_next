use glam::Vec3;

use crate::{graphics::Uniforms, types::Color};

use super::PixelShader;

#[derive(Clone, Copy)]
pub struct ColorBlend;

impl PixelShader<3> for ColorBlend {
    fn run(_: &Uniforms, parameters: [f32; 3]) -> Color {
        Color::from(parameters)
    }
}

#[derive(Clone, Copy)]
pub struct ColorBlendLit;

impl PixelShader<9> for ColorBlendLit {
    fn run(uniforms: &Uniforms, parameters: [f32; 9]) -> Color {
        // Shader Setup
        let [r, g, b, norm_x, norm_y, norm_z, pixel_x, pixel_y, pixel_z] = parameters;
        let pixel_position = Vec3::new(pixel_x, pixel_y, pixel_z);
        let normal = Vec3::new(norm_x, norm_y, norm_z);
        let object_color = Vec3::new(r, g, b);

        let pixel_to_light = (uniforms.light_position - pixel_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * uniforms.light_intensity, 0.0);

        Color::from(object_color * (light_factor + uniforms.ambient_light))
    }
}
