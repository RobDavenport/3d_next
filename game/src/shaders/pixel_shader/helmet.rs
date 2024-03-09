use glam::Vec3;
use shared::types::Color;

use crate::graphics::Uniforms;

use super::PixelShader;

#[derive(Clone, Copy)]
pub struct HelmetShader;

impl PixelShader<8> for HelmetShader {
    fn run(uniforms: &Uniforms, parameters: [f32; 8]) -> Color {
        // Shader Setup
        let [u, v, norm_x, norm_y, norm_z, pixel_x, pixel_y, pixel_z] = parameters;
        let pixel_position = Vec3::new(pixel_x, pixel_y, pixel_z);
        let normal = Vec3::new(norm_x, norm_y, norm_z);
        let index = uniforms.diffuse.get_index(u, v);
        let object_color = uniforms.diffuse.index_vec(index);
        let emissive = uniforms.emissive.index_vec(index);
        let occlusion = uniforms.occlusion.index_vec(index);

        // Lighting Calculations
        // TODO: Add tangent & normal map to this shader

        let pixel_to_light = (uniforms.light_position - pixel_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * uniforms.light_intensity, 0.0);

        Color::from(object_color * (light_factor + (uniforms.ambient_light * occlusion)) + emissive)
    }
}
