use crate::Gpu;

mod cubes;
pub use cubes::CubesScene;

mod cube_model;
pub use cube_model::CubeModelScene;

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
