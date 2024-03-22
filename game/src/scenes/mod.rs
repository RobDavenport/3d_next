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

mod helmet_model;
pub use helmet_model::HelmetModelScene;

mod helmet_model_simple;
pub use helmet_model_simple::HelmetModelSimpleScene;

mod rigged_figure;
pub use rigged_figure::RiggedFigureScene;

mod rigged_simple;
pub use rigged_simple::RiggedSimpleScene;

mod blockbench;
pub use blockbench::BlockbenchScene;

mod handrig;
pub use handrig::HandScene;

mod vs_scene;
pub use vs_scene::VsScene;

pub trait Scene {
    fn update(&mut self);
    fn draw(&self, gpu: &mut Gpu);
}
