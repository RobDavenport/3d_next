use std::{
    array,
    ops::{Add, Mul, MulAssign, Sub},
};

mod color_blend;
pub use color_blend::*;

mod textured;
pub use textured::*;

use crate::types::Color;

#[derive(Clone, Copy)]
pub struct VertexParameters<const P: usize>(pub [f32; P]);

impl<const P: usize> VertexParameters<P> {
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }
}

impl<const P: usize> Add<Self> for VertexParameters<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<const P: usize> Sub<Self> for VertexParameters<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<const P: usize> Mul<f32> for VertexParameters<P> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(array::from_fn(|i| self.0[i] * rhs))
    }
}

impl<const P: usize> MulAssign<f32> for VertexParameters<P> {
    fn mul_assign(&mut self, rhs: f32) {
        self.0.iter_mut().for_each(|v| *v *= rhs)
    }
}

pub trait PixelShader<const PSIN: usize> {
    fn run(&self, parameters: [f32; PSIN]) -> Color;
}
