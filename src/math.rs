use ultraviolet::{Mat4, Vec3};

pub trait Math {
    fn up_vector(&self) -> Vec3;
    fn right_vector(&self) -> Vec3;
    fn forward_vector(&self) -> Vec3;
}

// Convenience Functions
impl Math for Mat4 {
    // Returns the Right vector (+X) of this Matrix
    fn right_vector(&self) -> Vec3 {
        Vec3::new(self.cols[0].x, self.cols[1].x, self.cols[2].x)
    }

    // Returns the Up vector (+Y) of this Matrix
    fn up_vector(&self) -> Vec3 {
        Vec3::new(self.cols[0].y, self.cols[1].y, self.cols[2].y)
    }

    // Returns the Forward vector (+Z) of this Matrix
    fn forward_vector(&self) -> Vec3 {
        Vec3::new(self.cols[0].z, self.cols[1].z, self.cols[2].z)
    }
}
