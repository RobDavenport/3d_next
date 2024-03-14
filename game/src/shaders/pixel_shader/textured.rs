use glam::Vec3A;

use crate::graphics::Uniforms;
use shared::types::Color;

use super::PixelShader;

#[derive(Clone, Copy)]
pub struct Textured;
impl PixelShader<2> for Textured {
    fn run(uniforms: &Uniforms, parameters: [f32; 2]) -> Color {
        let [u, v] = parameters;
        uniforms.diffuse.sample_color(u, v)
    }
}

#[derive(Clone, Copy)]
pub struct TexturedLit;

impl PixelShader<8> for TexturedLit {
    fn run(uniforms: &Uniforms, parameters: [f32; 8]) -> Color {
        // Shader Setup
        let [u, v, norm_x, norm_y, norm_z, pixel_x, pixel_y, pixel_z] = parameters;
        let pixel_position = Vec3A::new(pixel_x, pixel_y, pixel_z);
        let normal = Vec3A::new(norm_x, norm_y, norm_z);
        let object_color = uniforms.diffuse.sample_vec(u, v);

        let pixel_to_light = (uniforms.light_position - pixel_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * uniforms.light_intensity, 0.0);

        Color::from(object_color * (light_factor + uniforms.ambient_light))
    }
}

#[derive(Clone, Copy)]
pub struct TexturedNormalMapLit;

impl PixelShader<8> for TexturedNormalMapLit {
    fn run(uniforms: &Uniforms, parameters: [f32; 8]) -> Color {
        // Shader Setup
        let [u, v, tan_light_x, tan_light_y, tan_light_z, tan_pixel_x, tan_pixel_y, tan_pixel_z] =
            parameters;
        let index = uniforms.diffuse.get_index(u, v);
        let object_color = uniforms.diffuse.index_veca(index);
        let tan_light = Vec3A::new(tan_light_x, tan_light_y, tan_light_z);
        let tan_position = Vec3A::new(tan_pixel_x, tan_pixel_y, tan_pixel_z);

        // Normal is in (0 -> 1) Ranges
        // So we need to put it in (-1 -> 1) Range
        let normal = uniforms.normal.index_veca(index);
        let normal = ((normal * 2.0) - Vec3A::ONE).normalize();

        let pixel_to_light = (tan_light - tan_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * uniforms.light_intensity, 0.0);

        Color::from(object_color * (light_factor + uniforms.ambient_light))
    }
}
