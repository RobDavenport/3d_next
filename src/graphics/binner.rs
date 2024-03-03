// use glam::Vec2;

// use crate::types::Color;

// use super::{
//     rasterizer::{EdgeStepperCombined, RenderTriangle},
//     Gpu,
// };

// impl Gpu {
//     pub(super) fn bin_triangle<const P: usize>(&mut self, triangle: RenderTriangle<P>) {
//         let width = self.render_tiles.w();
//         let height = self.render_tiles.h();
//         let horizontal_count = self.render_tiles.tile_count_horizontal;
//         let vertical_count = self.render_tiles.tile_count_vertical;

//         // Top Left, Top Right, Bottom Left, Bottom Right
//         let y_stamp = [0, 0, height as i32, height as i32];
//         let x_stamp = [0, width as i32, 0, width as i32];

//         let mut stepper = EdgeStepperCombined::new(
//             &triangle,
//             Vec2::ZERO,
//             &x_stamp,
//             &y_stamp,
//             width as i32,
//             height as i32,
//         );
//         let mut counter = 0;
//         let colors = &[
//             Color::new(255, 0, 0),
//             Color::new(0, 255, 0),
//             Color::new(0, 0, 255),
//             Color::new(255, 255, 0),
//             Color::new(0, 255, 255),
//             Color::new(255, 255, 255),
//         ];

//         for y in 0..vertical_count {
//             stepper.reset_row();
//             for x in 0..horizontal_count {
//                 let mask = stepper.point_inside_triangle_mask();
//                 let tile_index = (y * horizontal_count) + x;
//                 let tile = &self.render_tiles[tile_index];

//                 if mask.all() {
//                     // Trivial Accept - can just draw the entire tile
//                     let start_x = tile.x;
//                     let start_y = tile.y;

//                     for y in 0..height {
//                         for x in 0..width {
//                             let pixel_x = start_x + x;
//                             let pixel_y = start_y + y;

//                             let index = (pixel_y * self.screen_width) + pixel_x;

//                             self.frame_buffer.frame_buffer[index] =
//                                 colors[counter].to_graphics_params();
//                         }
//                     }

//                     counter += 1;
//                     counter %= colors.len();
//                 } else if mask.any() {
//                     // Overlapping
//                 }
//                 // Tile can just be rejected

//                 stepper.step_x();
//             }
//             stepper.step_y();
//         }
//     }
// }
