use glam::{Mat4, Vec3};

pub trait Math {
    fn up_vector(&self) -> Vec3;
    fn right_vector(&self) -> Vec3;
    fn forward_vector(&self) -> Vec3;
}

// Convenience Functions
impl Math for Mat4 {
    // Returns the Right vector (+X) of this Matrix
    fn right_vector(&self) -> Vec3 {
        Vec3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x)
    }

    // Returns the Up vector (+Y) of this Matrix
    fn up_vector(&self) -> Vec3 {
        Vec3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y)
    }

    // Returns the Forward vector (+Z) of this Matrix
    fn forward_vector(&self) -> Vec3 {
        Vec3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z)
    }
}
