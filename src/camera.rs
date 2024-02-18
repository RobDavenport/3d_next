use ultraviolet::{Mat4, Vec3};

use gamercade_rs::prelude as gc;

use crate::math::Math;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
    pub movement_speed: f32,
    pub projection: Mat4,
    pub view: Mat4,
}

impl Camera {
    // Position and aspect_ratio (width / height)
    pub fn new(position: Vec3, aspect_ratio: f32) -> Self {
        let yaw = 0.0f32;
        let pitch = 0.0f32;
        let sensitivity = 0.90;
        let movement_speed = 0.1;

        let hfov = 103f32.to_radians();
        let vfov = 2.0 * ((hfov / 2.0).tan() * aspect_ratio.recip()).atan();

        // Calculate new forward, right, and up vectors based on yaw and pitch
        let forward = Vec3::new(
            yaw.to_radians().sin() * pitch.to_radians().cos(),
            pitch.to_radians().sin(),
            yaw.to_radians().cos() * pitch.to_radians().cos(),
        )
        .normalized();

        Camera {
            position,
            yaw,
            pitch,
            sensitivity,
            movement_speed,
            projection: ultraviolet::projection::perspective_reversed_infinite_z_wgpu_dx_gl(
                vfov.to_degrees(),
                aspect_ratio,
                1.0,
            ),
            view: Mat4::look_at(position, position + forward, Vec3::unit_y()),
        }
    }

    pub fn update(&mut self) {
        // TODO: Mouse movement

        // Look Up/Down
        if Some(true) == gc::button_up_held(0) {
            self.pitch -= self.sensitivity;
        } else if Some(true) == gc::button_down_held(0) {
            self.pitch += self.sensitivity
        }
        self.pitch = self.pitch.max(-89.9).min(89.9); // Clamp pitch to prevent flipping

        // Look Right/Left
        if Some(true) == gc::button_right_held(0) {
            self.yaw -= self.sensitivity;
        } else if Some(true) == gc::button_left_held(0) {
            self.yaw += self.sensitivity
        }
        self.pitch %= 360.0;

        // Calculate new forward, right, and up vectors based on yaw and pitch
        let new_forward = Vec3::new(
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
        )
        .normalized();
        self.view = Mat4::look_at(self.position, self.position + new_forward, Vec3::unit_y());

        // Keyboard movement
        let mut velocity = Vec3::zero();
        if let Some(true) = gc::button_b_held(0) {
            velocity += self.view.forward_vector();
        } else if let Some(true) = gc::button_c_held(0) {
            velocity -= self.view.forward_vector();
        };

        // Strafe Right/Left
        if let Some(true) = gc::button_d_held(0) {
            velocity -= self.view.right_vector();
        } else if let Some(true) = gc::button_a_held(0) {
            velocity += self.view.right_vector();
        };

        self.position += velocity * self.movement_speed;
    }
}
