use glam::Vec3;

use crate::assets::textures;
use crate::{assets::Texture, types::Color};

use super::PixelShader;
pub struct Textured {
    pub texture: &'static Texture,
}

impl Default for Textured {
    fn default() -> Self {
        Self {
            texture: crate::assets::textures::GAMERCADE,
        }
    }
}

impl Textured {
    fn sample_2d(&self, u: f32, v: f32) -> Color {
        let u = (u * (self.texture.width - 1) as f32) as usize;
        let v = (v * (self.texture.height - 1) as f32) as usize;

        let u = u.clamp(0, self.texture.width - 1);
        let v = v.clamp(0, self.texture.height - 1);

        self.texture.get_sample(u, v)
    }
}

impl PixelShader<2> for Textured {
    fn run(&self, parameters: [f32; 2]) -> Color {
        self.sample_2d(parameters[0], parameters[1])
    }
}

#[derive(Default)]
pub struct TexturedLit {
    pub light_position: Vec3,
    pub light_intensity: f32,
    pub ambient_light: f32,
    pub textured: Textured,
}

impl PixelShader<8> for TexturedLit {
    fn run(&self, parameters: [f32; 8]) -> Color {
        // Shader Setup
        let [u, v, norm_x, norm_y, norm_z, pixel_x, pixel_y, pixel_z] = parameters;
        let pixel_position = Vec3::new(pixel_x, pixel_y, pixel_z);
        let normal = Vec3::new(norm_x, norm_y, norm_z);
        let object_color = self.textured.sample_2d(u, v).to_vec3();

        let pixel_to_light = (self.light_position - pixel_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * self.light_intensity, 0.0);

        Color::from(object_color * (light_factor + self.ambient_light))
    }
}

pub struct TexturedNormalLit {
    pub light_position: Vec3,
    pub light_intensity: f32,
    pub ambient_light: f32,
    pub diffuse: Textured,
    pub normal: Textured,
}

impl Default for TexturedNormalLit {
    fn default() -> Self {
        Self {
            diffuse: Textured {
                texture: textures::BRICKWALL,
            },
            normal: Textured {
                texture: textures::BRICKWALL_NORMAL,
            },
            light_position: Default::default(),
            light_intensity: Default::default(),
            ambient_light: Default::default(),
        }
    }
}

impl PixelShader<5> for TexturedNormalLit {
    fn run(&self, parameters: [f32; 5]) -> Color {
        // Shader Setup
        let [u, v, pixel_x, pixel_y, pixel_z] = parameters;
        let pixel_position = Vec3::new(pixel_x, pixel_y, pixel_z);
        let object_color = self.diffuse.sample_2d(u, v).to_vec3();

        // Normal is in (0 -> 1) Ranges
        // So we need to put it in (-1 -> 1) Range
        let normal = self.normal.sample_2d(u, v).to_vec3();
        let normal = ((normal * 2.0) - Vec3::ONE).normalize();

        let pixel_to_light = (self.light_position - pixel_position).normalize();
        let light_factor = f32::max(pixel_to_light.dot(normal) * self.light_intensity, 0.0);

        Color::from(object_color * (light_factor + self.ambient_light))
    }
}
