use crate::Gpu;

mod cubes;
pub use cubes::CubesScene;

mod plane;
pub use plane::PlaneScene;

mod triangle;
pub use triangle::TriangleScene;

mod cube;
pub use cube::CubeScene;

pub trait Scene {
    fn update(&mut self);
    fn draw(&self, gpu: &mut Gpu);
}
