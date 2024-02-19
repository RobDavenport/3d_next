mod gpu;
pub mod graphics_db;

mod rasterizer;

use glam::Vec4;
pub use gpu::Gpu;
pub use graphics_db::*;

struct Triangle<P> {
    positions: [Vec4; 3],
    parameters: [P; 3],
}
