use gamercade_rs::api::graphics_parameters::GraphicsParameters;
use glam::{Mat3, Mat4, Vec3, Vec4, Vec4Swizzles};
use wide::{f32x4, CmpNe};

use crate::{
    actor::Actor,
    shaders::{PixelShader, VertexShader},
};

use super::{
    clipping::{clip_triangle, ClipResult},
    rasterizer::X_STEP_SIZE,
    Triangle, Uniforms,
};

pub struct Gpu {
    pub(super) screen_width: usize,
    pub(super) screen_height: usize,
    pub(super) z_buffer: ZBuffer,
    pub frame_buffer: Box<[GraphicsParameters]>,
    pub uniforms: Uniforms,
}

use crate::assets::textures;

impl Gpu {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            screen_height,
            screen_width,
            z_buffer: ZBuffer::new(screen_width, screen_height),
            frame_buffer: (0..(screen_height * screen_width) + X_STEP_SIZE)
                .map(|_| Default::default())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            uniforms: Uniforms {
                light_position: Vec3::default(),
                light_intensity: 1.25,
                ambient_light: 0.15,
                diffuse: textures::BRICKWALL_T,
                normal: textures::BRICKWALL_NORMAL_T,
                model: Mat4::IDENTITY,
                view: Mat4::IDENTITY,
                projection: Mat4::IDENTITY,
            },
        }
    }

    pub fn clear_z_buffer(&mut self) {
        self.z_buffer.clear_z_buffer();
    }

    pub fn clear_frame_buffer(&mut self) {
        self.frame_buffer
            .iter_mut()
            .for_each(|x| *x = GraphicsParameters::new())
    }

    // Adds the triangles fr
    pub fn render_actor<VS, const VSIN: usize, PS, const PSIN: usize>(
        &mut self,
        actor: &Actor<VSIN>,
        _vs: VS,
        ps: PS,
    ) where
        VS: VertexShader<VSIN, PSIN>,
        PS: PixelShader<PSIN>,
    {
        let vertex_list = actor.mesh.vertices.0;
        let indices = actor.mesh.indices.0;

        // Iterate each triangle of the mesh
        for triangle_indices in indices.iter() {
            let a = vertex_list[triangle_indices.0 as usize];
            let b = vertex_list[triangle_indices.1 as usize];
            let c = vertex_list[triangle_indices.2 as usize];
            let params = actor.mesh.parameters;

            // Run Vertex shader on every vertexs
            // This should output them into clip space
            let a_clip = VS::run(&self.uniforms, a, params.0[triangle_indices.0 as usize].0);
            let b_clip = VS::run(&self.uniforms, b, params.0[triangle_indices.1 as usize].0);
            let c_clip = VS::run(&self.uniforms, c, params.0[triangle_indices.2 as usize].0);

            // Culling Stage
            if is_backfacing(a_clip.position, b_clip.position, c_clip.position) {
                continue; // Skip this triangle if it's a backface
            }

            let triangle = Triangle {
                positions: [a_clip.position, b_clip.position, c_clip.position],
                parameters: [a_clip.parameters, b_clip.parameters, c_clip.parameters],
            };

            // Clip triangles, and whatever remains, rasterize them
            let clip_result = clip_triangle(triangle);
            match clip_result {
                ClipResult::Culled => continue,
                ClipResult::One(triangle) => {
                    let triangle = self.tri_clip_to_screen_space(triangle);
                    self.rasterize_triangle(triangle, ps);
                }
                ClipResult::Two((first, second)) => {
                    let first = self.tri_clip_to_screen_space(first);
                    let second = self.tri_clip_to_screen_space(second);
                    self.rasterize_triangle(first, ps);
                    self.rasterize_triangle(second, ps);
                }
            }
        }
    }

    // Converts a triangle from clip space into screen space
    fn tri_clip_to_screen_space<const P: usize>(
        &self,
        mut clip_space_triangle: Triangle<P>,
    ) -> Triangle<P> {
        let clip_to_screen = |clip_space_vertex: Vec4| {
            // Move to cartesian coordinates
            let clip_space_vertex = clip_space_vertex / clip_space_vertex.w;

            // Convert NDC coordinates to screen space
            let screen_x = (clip_space_vertex.x + 1.0) * (self.screen_width as f32 / 2.0);
            let screen_y = (1.0 - clip_space_vertex.y) * (self.screen_height as f32 / 2.0);

            Vec4::new(screen_x, screen_y, clip_space_vertex.z, clip_space_vertex.w)
        };

        clip_space_triangle.positions[0] = clip_to_screen(clip_space_triangle.positions[0]);
        clip_space_triangle.positions[1] = clip_to_screen(clip_space_triangle.positions[1]);
        clip_space_triangle.positions[2] = clip_to_screen(clip_space_triangle.positions[2]);

        clip_space_triangle
    }
}

pub struct ZBuffer {
    pub z_buffer: Box<[f32]>,
}

impl ZBuffer {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            z_buffer: (0..(screen_height * screen_width) + X_STEP_SIZE)
                .map(|_| f32::INFINITY)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    // Clears the Z buffer by setting all values to f32::INFINITY
    fn clear_z_buffer(&mut self) {
        self.z_buffer.iter_mut().for_each(|d| *d = f32::INFINITY);
    }

    // Returns a u32x4 mask if the value was closer the target value
    // and therefore should be drawn. Also updates the buffer with the new value
    pub fn test_and_set(&mut self, pixel_index: usize, depths: f32x4, mask: f32x4) -> i32 {
        let current_depths = f32x4::new([
            self.z_buffer[pixel_index],
            self.z_buffer[pixel_index + 1],
            self.z_buffer[pixel_index + 2],
            self.z_buffer[pixel_index + 3],
        ]);

        //Take the max values between depths and current depths
        let merged_max = depths.min(current_depths);

        // If it's on the triangle, take the max value from the previous stetp
        // If its not on the triangle, take the previous value
        let new_depths = mask.blend(merged_max, current_depths);

        // Check if we got any new values
        let changed = new_depths.cmp_ne(current_depths);

        // If we did, we need to update the buffer and return the output
        if changed.any() {
            let data = self.z_buffer[pixel_index..pixel_index].as_mut_ptr();
            unsafe { (data as *mut [f32; 4]).write(new_depths.into()) }
            changed.move_mask()
        } else {
            0
        }
    }
}

fn is_backfacing(a: Vec4, b: Vec4, c: Vec4) -> bool {
    Mat3::from_cols(a.xyw(), b.xyw(), c.xyw()).determinant() < 0.0
}
