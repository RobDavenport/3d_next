use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use wide::{f32x4, CmpNe};

use crate::{
    actor::Actor,
    shaders::{PixelShader, PixelShaderInput},
    CAMERA, GRAPHICS_DB,
};

use super::{ParameterDataBuffer, ParameterDb, Triangle};

pub struct Gpu {
    pub(super) screen_width: usize,
    pub(super) screen_height: usize,
    pub(super) z_buffer: ZBuffer,
}

impl Gpu {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            screen_height,
            screen_width,
            z_buffer: ZBuffer::new(screen_width, screen_height),
        }
    }

    pub fn clear_z_buffer(&mut self) {
        self.z_buffer.clear_z_buffer();
    }

    // Adds the triangles fr
    pub fn render_actor<PS, PSIN>(&mut self, actor: &Actor<PSIN>)
    where
        PSIN: PixelShaderInput,
        PS: PixelShader<PSIN>,
        ParameterDb: ParameterDataBuffer<PSIN>,
    {
        let graphics_db = unsafe { GRAPHICS_DB.assume_init_ref() };
        let mesh = graphics_db.get(actor.mesh_id);
        let vertex_list = mesh.vertices;
        let indices = mesh.indices;

        // Pre-calculate the MVP
        let camera = unsafe { &CAMERA.assume_init_ref() };
        let projection_matrix = &camera.projection;
        let view_matrix = camera.view;
        let model_matrix = actor.transform;
        let mvp = *projection_matrix * (view_matrix * model_matrix);

        // Iterate each triangle of the mesh
        for triangle_indices in indices.iter() {
            let a = &vertex_list[triangle_indices.0];
            let b = &vertex_list[triangle_indices.1];
            let c = &vertex_list[triangle_indices.2];

            // Transform all vertices to clip space
            let a_clip = transform_to_clip_space(a, &mvp);
            let b_clip = transform_to_clip_space(b, &mvp);
            let c_clip = transform_to_clip_space(c, &mvp);

            if is_backfacing(a_clip, b_clip, c_clip) {
                continue; // Skip this triangle if it's a backface
            }

            // TODO: Triangle Geometry Clipping

            let a_screen =
                translate_clip_to_screen_space(&a_clip, self.screen_width, self.screen_height);
            let b_screen =
                translate_clip_to_screen_space(&b_clip, self.screen_width, self.screen_height);
            let c_screen =
                translate_clip_to_screen_space(&c_clip, self.screen_width, self.screen_height);

            let params = mesh.parameters;
            let triangle = Triangle {
                positions: [a_screen, b_screen, c_screen],
                parameters: [
                    params[triangle_indices.0],
                    params[triangle_indices.1],
                    params[triangle_indices.2],
                ],
            };

            // Rasterize the triangle
            self.rasterize_triangle::<PS, PSIN>(triangle);
        }
    }
}

pub struct ZBuffer {
    pub z_buffer: Box<[f32]>,
}

impl ZBuffer {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            // Add padding in case SIMD access extra values
            z_buffer: (0..(screen_height * screen_width) + super::rasterizer::X_STEP_SIZE)
                .map(|_| f32::NEG_INFINITY)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    // Clears the Z buffer by setting all values to f32::NEG_INFINITY
    fn clear_z_buffer(&mut self) {
        self.z_buffer
            .iter_mut()
            .for_each(|d| *d = f32::NEG_INFINITY);
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
        let merged_max = depths.max(current_depths);

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

fn transform_to_clip_space(vertex: &Vec3, mvp: &Mat4) -> Vec4 {
    // Convert vertex position to homogeneous coordinates (4D)
    let mut position_homogeneous = vertex.extend(1.0);

    // Apply projection transformation
    position_homogeneous = *mvp * position_homogeneous;

    // Homogenize the result
    position_homogeneous /= position_homogeneous.w;

    // Return the transformed vertex in clip space
    position_homogeneous
}

fn translate_clip_to_screen_space(
    clip_space_vertex: &Vec4,
    screen_width: usize,
    screen_height: usize,
) -> Vec4 {
    // to NDC
    let ndc_vertex = *clip_space_vertex / clip_space_vertex.w;

    // Convert NDC coordinates to screen space
    let screen_x = (ndc_vertex.x + 1.0) * (screen_width as f32 / 2.0);
    let screen_y = (1.0 - ndc_vertex.y) * (screen_height as f32 / 2.0);

    Vec4::new(screen_x, screen_y, ndc_vertex.z, ndc_vertex.w)
}

fn is_backfacing(a: Vec4, b: Vec4, c: Vec4) -> bool {
    let ab = b - a;
    let ac = c - a;
    let normal = ab.xyz().cross(ac.xyz()); // Calculate the normal of the triangle
    let view_dir = Vec3::new(0.0, 0.0, 1.0); // Assuming camera looks along the positive z-axis

    normal.dot(view_dir) > 0.0 // Check if the triangle is facing away from the camera, zero is perpendicular
}
