use glam::{Mat4, Vec3A};

pub trait Math {
    fn up_vector(&self) -> Vec3A;
    fn right_vector(&self) -> Vec3A;
    fn forward_vector(&self) -> Vec3A;
}

// Convenience Functions
impl Math for Mat4 {
    // Returns the Right vector (+X) of this Matrix
    fn right_vector(&self) -> Vec3A {
        Vec3A::new(self.x_axis.x, self.y_axis.x, self.z_axis.x)
    }

    // Returns the Up vector (+Y) of this Matrix
    fn up_vector(&self) -> Vec3A {
        Vec3A::new(self.x_axis.y, self.y_axis.y, self.z_axis.y)
    }

    // Returns the Forward vector (+Z) of this Matrix
    fn forward_vector(&self) -> Vec3A {
        Vec3A::new(self.x_axis.z, self.y_axis.z, self.z_axis.z)
    }
}

// TODO: Consider using Fixed Point math
// #[derive(Clone, Copy)]
// pub struct FixedPoint<const FRACTIONAL_BITS: usize> {
//     value: i32,
// }

// impl<const FRACTIONAL_BITS: usize> From<f32> for FixedPoint<FRACTIONAL_BITS> {
//     fn from(value: f32) -> Self {
//         Self {
//             value: (value * (1 << FRACTIONAL_BITS) as f32) as i32,
//         }
//     }
// }

// impl<const FRACTIONAL_BITS: usize> From<FixedPoint<FRACTIONAL_BITS>> for f32 {
//     fn from(val: FixedPoint<FRACTIONAL_BITS>) -> Self {
//         val.value as f32 / (1 << FRACTIONAL_BITS) as f32
//     }
// }

// impl<const FRACTIONAL_BITS: usize> From<i32> for FixedPoint<FRACTIONAL_BITS> {
//     fn from(value: i32) -> Self {
//         Self {
//             value: (value * (1 << FRACTIONAL_BITS)),
//         }
//     }
// }

// impl<const FRACTIONAL_BITS: usize> From<FixedPoint<FRACTIONAL_BITS>> for i32 {
//     fn from(val: FixedPoint<FRACTIONAL_BITS>) -> Self {
//         val.value / (1 << FRACTIONAL_BITS)
//     }
// }

// impl<const FRACTIONAL_BITS: usize> FixedPoint<FRACTIONAL_BITS> {
//     // Addition
//     pub fn fixed_add(&self, rhs: Self) -> Self {
//         Self {
//             value: self.value + rhs.value,
//         }
//     }

//     // Subtraction
//     pub fn fixed_sub(&self, rhs: Self) -> Self {
//         Self {
//             value: self.value - rhs.value,
//         }
//     }

//     // Multiplication
//     pub fn fixed_mul(&self, rhs: Self) -> Self {
//         Self {
//             value: (self.value * rhs.value) >> FRACTIONAL_BITS,
//         }
//     }

//     // Division
//     pub fn fixed_div(&self, rhs: Self) -> Self {
//         Self {
//             value: (self.value << FRACTIONAL_BITS) / rhs.value,
//         }
//     }
// }
