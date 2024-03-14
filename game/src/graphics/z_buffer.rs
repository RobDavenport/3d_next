use wide::{f32x4, CmpNe};

//use super::rasterizer::X_STEP_SIZE;

const RESET_DEPTH: f32 = f32::NEG_INFINITY;

pub struct ZBuffer {
    pub z_buffer: Box<[f32]>,
}

impl ZBuffer {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            z_buffer: (0..(screen_height * screen_width))
                .map(|_| RESET_DEPTH)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    // Clears the Z buffer by setting all values to reset value
    pub(crate) fn clear(&mut self) {
        self.z_buffer.iter_mut().for_each(|d| *d = RESET_DEPTH);
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
