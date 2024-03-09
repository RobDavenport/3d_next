use std::{
    array,
    ops::{Add, Mul, Sub},
};

mod pixel_shader;
mod vertex_shader;

pub use pixel_shader::*;
use shared::vertex_parameters::VertexParameters;
pub use vertex_shader::*;
use wide::f32x4;

#[derive(Clone)]
pub struct VertexParametersSimd<const P: usize>(pub [f32x4; P]);

impl<const P: usize> Mul<f32x4> for VertexParametersSimd<P> {
    type Output = Self;

    fn mul(self, rhs: f32x4) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] * rhs))
    }
}

impl<const P: usize> Add<Self> for VertexParametersSimd<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<const P: usize> Sub<Self> for VertexParametersSimd<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<const P: usize> VertexParametersSimd<P> {
    pub fn extract(&self, index: usize) -> [f32; P] {
        array::from_fn(|i| self.0[i].as_array_ref()[index])
    }
}

impl<const P: usize> VertexParametersSimd<P> {
    pub fn splat(values: &VertexParameters<P>) -> VertexParametersSimd<P> {
        VertexParametersSimd(array::from_fn(|i| f32x4::splat(values.0[i])))
    }
}
