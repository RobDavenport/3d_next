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

mod duck_model;
pub use duck_model::DuckModelScene;

mod fox_model;
pub use fox_model::FoxModelScene;

pub trait Scene {
    fn update(&mut self);
    fn draw(&self, gpu: &mut Gpu);
}
