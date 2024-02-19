use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use wide::{f32x4, u32x4, CmpNe};

use crate::{actor::Actor, CAMERA, GRAPHICS_DB};

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

    // TODO: Make this take a triangle buffer
    pub fn render_actor(&mut self, actor: &Actor) {
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

            let a_screen = clip_to_screen_space(&a_clip, self.screen_width, self.screen_height);
            let b_screen = clip_to_screen_space(&b_clip, self.screen_width, self.screen_height);
            let c_screen = clip_to_screen_space(&c_clip, self.screen_width, self.screen_height);

            // Rasterize the triangle
            self.rasterize_triangle(a_screen, b_screen, c_screen);
        }
    }
}

pub struct ZBuffer {
    pub z_buffer: Box<[f32]>,
}

impl ZBuffer {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            z_buffer: (0..screen_height * screen_width)
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
    // TODO: Double check this logic
    pub fn test_and_set(&mut self, pixel_indices: u32x4, depths: f32x4, mask: f32x4) -> i32 {
        let pixel_indices = pixel_indices.min(u32x4::splat(self.z_buffer.len() as u32 - 1));
        let pixel_indices = pixel_indices.as_array_ref();
        let current_depths = f32x4::new([
            self.z_buffer[pixel_indices[0] as usize],
            self.z_buffer[pixel_indices[1] as usize],
            self.z_buffer[pixel_indices[2] as usize],
            self.z_buffer[pixel_indices[3] as usize],
        ]);

        // The incoming data
        let invalid_depths = f32x4::splat(f32::NEG_INFINITY) & !mask;
        let valid_depths = depths & mask;
        let to_test = valid_depths | invalid_depths;

        // See if any results are > the current depths
        let result = to_test.max(current_depths);
        let changed = result.cmp_ne(current_depths);

        // We have to update depths
        if changed.any() {
            self.z_buffer[pixel_indices[0] as usize] = changed.as_array_ref()[0];
            self.z_buffer[pixel_indices[1] as usize] = changed.as_array_ref()[1];
            self.z_buffer[pixel_indices[2] as usize] = changed.as_array_ref()[2];
            self.z_buffer[pixel_indices[3] as usize] = changed.as_array_ref()[3];
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

fn clip_to_screen_space(
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
