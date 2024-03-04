use glam::Vec2;

use crate::{shaders::PixelShader};

use super::{
    rasterizer::{EdgeStepperCombined, RenderTriangle},
    Gpu,
};

impl Gpu {
    pub(super) fn bin_triangle<const P: usize, PS>(&mut self, triangle: RenderTriangle<P>, ps: PS)
    where
        PS: PixelShader<P>,
    {
        let width = self.render_tiles.w();
        let height = self.render_tiles.h();
        let horizontal_count = self.render_tiles.tile_count_horizontal;
        let vertical_count = self.render_tiles.tile_count_vertical;

        // Top Left, Top Right, Bottom Left, Bottom Right
        let y_stamp = [0, 0, height as i32, height as i32];
        let x_stamp = [0, width as i32, 0, width as i32];

        let mut stepper = EdgeStepperCombined::new(
            &triangle,
            Vec2::ZERO,
            &x_stamp,
            &y_stamp,
            width as i32,
            height as i32,
        );

        for y in 0..vertical_count {
            stepper.reset_row();
            for x in 0..horizontal_count {
                let tile_index = (y * horizontal_count) + x;
                let tile = &mut self.render_tiles[tile_index];

                // We only care about triangles which overlap the tile's BB
                if tile.overlap(&triangle) {
                    let mask = stepper.points_inside_triangle_mask();

                    if mask.all() {
                        // Trivial Accept - All corners of the tile are within the triangle
                        tile.trivial_rasterize_triangle(&self.uniforms, triangle.clone(), ps);
                    } else {
                        // Overlapping triangle, Rasterize it normally
                        tile.rasterize_triangle(&self.uniforms, triangle.clone(), ps);
                    }
                }

                stepper.step_x();
            }
            stepper.step_y();
        }
    }
}
