use glam::Vec3;

use crate::graphics::Uniforms;
use crate::types::Color;

use super::PixelShader;

#[derive(Clone, Copy)]
pub struct Textured;
impl PixelShader<2> for Textured {
    fn run(uniforms: &Uniforms, parameters: [f32; 2]) -> Color {
        let [u, v] = parameters;
        uniforms.diffuse.get_sample(u, v)
    }
}

#[derive(Clone, Copy)]
pub struct TexturedLit;

impl PixelShader<8> for TexturedLit {
    fn run(uniforms: &Uniforms, parameters: [f32; 8]) -> Color {
        // Shader Setup
        let [u, v, norm_x, norm_y, norm_z, pixel_x, pixel_y, pixel_z] = parameters;
        let pixel_position = Vec3::new(pixel_x, pixel_y, pixel_z);
        let normal = Vec3::new(norm_x, norm_y, norm_z);
        let object_color = uniforms.diffuse.get_sample(u, v).to_vec3();

        let pixel_to_light = (uniforms.light_position - pixel_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * uniforms.light_intensity, 0.0);

        Color::from(object_color * (light_factor + uniforms.ambient_light))
    }
}

#[derive(Clone, Copy)]
pub struct TexturedNormalLit;

impl PixelShader<8> for TexturedNormalLit {
    fn run(uniforms: &Uniforms, parameters: [f32; 8]) -> Color {
        // Shader Setup
        let [u, v, tan_light_x, tan_light_y, tan_light_z, tan_pixel_x, tan_pixel_y, tan_pixel_z] =
            parameters;
        let object_color = uniforms.diffuse.get_sample(u, v).to_vec3();
        let tan_light = Vec3::new(tan_light_x, tan_light_y, tan_light_z);
        let tan_position = Vec3::new(tan_pixel_x, tan_pixel_y, tan_pixel_z);

        // Normal is in (0 -> 1) Ranges
        // So we need to put it in (-1 -> 1) Range
        let normal = uniforms.normal.get_sample(u, v).to_vec3();
        let normal = ((normal * 2.0) - Vec3::ONE).normalize();

        let pixel_to_light = (tan_light - tan_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * uniforms.light_intensity, 0.0);

        Color::from(object_color * (light_factor + uniforms.ambient_light))
    }
}
