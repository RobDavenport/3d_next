use crate::Gpu;

mod cubes;


mod cube_model;


mod plane;
pub use plane::PlaneScene;

mod triangle;
pub use triangle::TriangleScene;

mod cube;


mod duck_model;


mod fox_model;


mod helmet_model;


mod helmet_model_simple;


pub trait Scene {
    fn update(&mut self);
    fn draw(&self, gpu: &mut Gpu);
}
