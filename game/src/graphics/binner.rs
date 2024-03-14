use glam::Vec2;

use crate::shaders::PixelShader;

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

        let min_tile_x = (triangle.min_x / width as f32).floor() as usize;
        let max_tile_x = (triangle.max_x / width as f32).ceil() as usize;

        let min_tile_y = (triangle.min_y / height as f32).floor() as usize;
        let max_tile_y = (triangle.max_y / height as f32).ceil() as usize;

        let min_tile_x = min_tile_x.clamp(0, horizontal_count);
        let max_tile_x = max_tile_x.clamp(0, horizontal_count);

        let min_tile_y = min_tile_y.clamp(0, vertical_count);
        let max_tile_y = max_tile_y.clamp(0, vertical_count);

        // Top Left, Top Right, Bottom Left, Bottom Right
        let x_stamp = [0, width as i32 - 1, 0, width as i32 - 1];
        let y_stamp = [0, 0, height as i32 - 1, height as i32 - 1];

        let top_left = Vec2::new((min_tile_x * width) as f32, (min_tile_y * height) as f32);

        let mut stepper = EdgeStepperCombined::new(
            &triangle,
            top_left,
            &x_stamp,
            &y_stamp,
            width as i32,
            height as i32,
        );

        for y in min_tile_y..max_tile_y {
            stepper.reset_row();
            for x in min_tile_x..max_tile_x {
                let tile_index = (y * horizontal_count) + x;
                let tile = &mut self.render_tiles[tile_index];

                // We only care about triangles which overlap the tile's BB
                if stepper.points_inside_triangle_mask().all() {
                    // Trivial Accept - All corners of the tile are within the triangle
                    tile.trivial_rasterize_triangle(&self.uniforms, triangle.clone(), ps);
                } else if tile.triangle_edges_intersect_aabb(&triangle) {
                    // Triangle is overlapping, but only render those whose edges intersect the AABB
                    tile.rasterize_triangle(&self.uniforms, triangle.clone(), ps);
                }

                // Tile shoulud just be skipped

                stepper.step_x();
            }
            stepper.step_y();
        }
    }
}
