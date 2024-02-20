use std::f32::consts::{FRAC_PI_2, TAU};

use glam::{Mat4, Vec3};

use gamercade_rs::prelude as gc;

use crate::math::Math;

const PITCH_CLAMP: f32 = FRAC_PI_2 * 0.99;

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
        let sensitivity = 0.09;
        let movement_speed = 0.1;

        let hfov = 103f32.to_radians();
        let vfov = 2.0 * ((hfov / 2.0).tan() * aspect_ratio.recip()).atan();

        Camera {
            position,
            yaw: 0.0,
            pitch: 0.0,
            sensitivity,
            movement_speed,
            projection: Mat4::perspective_infinite_reverse_rh(vfov, aspect_ratio, 1.0),
            view: Mat4::look_to_rh(position, Vec3::NEG_Z, Vec3::Y),
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
        self.pitch = self.pitch.max(-PITCH_CLAMP).min(PITCH_CLAMP); // Clamp pitch to prevent flipping

        // Look Right/Left
        if Some(true) == gc::button_right_held(0) {
            self.yaw -= self.sensitivity;
        } else if Some(true) == gc::button_left_held(0) {
            self.yaw += self.sensitivity
        }
        self.pitch %= TAU;

        // Calculate new forward, right, and up vectors based on yaw and pitch
        let new_forward = forward_from_yaw_pitch(self.yaw, self.pitch);
        self.view = Mat4::look_to_rh(self.position, new_forward, Vec3::Y);

        // Keyboard movement
        let mut velocity = Vec3::ZERO;
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

fn forward_from_yaw_pitch(yaw: f32, pitch: f32) -> Vec3 {
    // Calculate the components of the forward vector
    let x = yaw.sin() * pitch.cos();
    let y = pitch.sin();
    let z = yaw.cos() * pitch.cos();

    // Return the resulting forward vector
    Vec3::new(x, y, z)
}
