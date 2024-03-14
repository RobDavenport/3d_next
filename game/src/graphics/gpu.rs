use gamercade_rs::api::graphics_parameters::GraphicsParameters;
use gamercade_rs::prelude as gc;
use glam::{Mat3, Vec4, Vec4Swizzles};
use shared::{mesh::ArchivedMesh, types::Color};

use crate::{
    animation::Animator,
    shaders::{PixelShader, VertexShader},
};

use super::{
    clipping::ClipResult,
    rasterizer::RenderTriangle,
    render_tile::{TILE_HEIGHT, TILE_PIXELS, TILE_WIDTH},
    tile_manager::TileManager,
    Triangle, Uniforms,
};

pub struct Gpu {
    pub(super) screen_width: usize,
    pub(super) screen_height: usize,
    frame_buffer: Box<[GraphicsParameters]>,
    pub uniforms: Uniforms,
    pub(super) render_tiles: TileManager<TILE_WIDTH, TILE_HEIGHT, TILE_PIXELS>,
}

impl Gpu {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            screen_height,
            screen_width,
            frame_buffer: vec![GraphicsParameters::default(); screen_height * screen_width]
                .into_boxed_slice(),
            uniforms: Uniforms::default(),
            render_tiles: TileManager::new(screen_width, screen_height),
        }
    }

    pub fn reset_frame(&mut self) {
        self.render_tiles.reset_frame();
    }

    pub fn render_mesh<VS, const VSIN: usize, PS, const PSIN: usize>(
        &mut self,
        mesh: &ArchivedMesh<VSIN>,
        vs: VS,
        ps: PS,
    ) where
        VS: VertexShader<VSIN, PSIN>,
        PS: PixelShader<PSIN>,
    {
        let vertex_list = &mesh.vertices.0;
        let indices = &mesh.indices.0;

        // Iterate each triangle of the mesh
        for triangle_indices in indices.iter() {
            let a = vertex_list[triangle_indices.0 as usize];
            let b = vertex_list[triangle_indices.1 as usize];
            let c = vertex_list[triangle_indices.2 as usize];
            let params = &mesh.parameters;

            // Run Vertex shader on every vertexs
            // This should output them into clip space
            let a_clip = vs.run(
                triangle_indices.0 as usize,
                &self.uniforms,
                a,
                params.0[triangle_indices.0 as usize].0,
            );
            let b_clip = vs.run(
                triangle_indices.1 as usize,
                &self.uniforms,
                b,
                params.0[triangle_indices.1 as usize].0,
            );
            let c_clip = vs.run(
                triangle_indices.2 as usize,
                &self.uniforms,
                c,
                params.0[triangle_indices.2 as usize].0,
            );

            // Culling Stage
            if is_backfacing(a_clip.position, b_clip.position, c_clip.position) {
                continue; // Skip this triangle if it's a backface
            }

            // Clipping Stage - Triangle is being re-wound here as the later
            // Y-flip for NDC -> Screen space reverses winding order
            let triangle = Triangle {
                positions: [a_clip.position, c_clip.position, b_clip.position],
                parameters: [a_clip.parameters, c_clip.parameters, b_clip.parameters],
            };
            let clip_result = self.clip_stage(triangle);

            // Triangle Setup -> Pass to binner
            match clip_result {
                ClipResult::Culled => continue,
                ClipResult::One(triangle) => {
                    let triangle = self.tri_clip_to_screen_space(triangle);
                    let triangle = RenderTriangle::setup(triangle);

                    self.bin_triangle(triangle, ps);
                }
                ClipResult::Two((first, second)) => {
                    let first = self.tri_clip_to_screen_space(first);
                    let second = self.tri_clip_to_screen_space(second);

                    let first = RenderTriangle::setup(first);
                    let second = RenderTriangle::setup(second);

                    self.bin_triangle(first, ps);
                    self.bin_triangle(second, ps);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn render_animator<const B: usize, const I: usize>(&self, animator: &Animator<B, I>) {
        let mvp = self.uniforms.projection * (self.uniforms.view * self.uniforms.model);

        animator
            .skeleton
            .0
            .iter()
            .enumerate()
            .for_each(|(bone_index, bone)| {
                let local = animator.current_pose[bone_index];
                let local_pos = local.w_axis;
                let local_world = mvp * local_pos;

                let local_world = self.clip_to_screen(local_world);

                if bone.parent_index.is_negative() {
                    gc::set_pixel(
                        Color::new(255, 0, 0).to_graphics_params(),
                        local_world.x as i32,
                        local_world.y as i32,
                    );
                } else {
                    let parent = &animator.current_pose[bone.parent_index as usize];
                    let parent_pos = parent.w_axis;
                    let parent_world = mvp * parent_pos;
                    let parent_world = self.clip_to_screen(parent_world);
                    gc::line(
                        Color::new(0, 255, 0).to_graphics_params(),
                        local_world.x as i32,
                        local_world.y as i32,
                        parent_world.x as i32,
                        parent_world.y as i32,
                    )
                }
            })
    }

    // Converts a triangle from clip space into screen space
    fn tri_clip_to_screen_space<const P: usize>(
        &self,
        mut clip_space_triangle: Triangle<P>,
    ) -> Triangle<P> {
        clip_space_triangle.positions[0] = self.clip_to_screen(clip_space_triangle.positions[0]);
        clip_space_triangle.positions[1] = self.clip_to_screen(clip_space_triangle.positions[1]);
        clip_space_triangle.positions[2] = self.clip_to_screen(clip_space_triangle.positions[2]);

        clip_space_triangle
    }

    // Stitches together a frame buffer
    pub fn generate_frame_buffer(&mut self) -> &[GraphicsParameters] {
        let tile_width = self.render_tiles.w();
        let tile_height = self.render_tiles.h();
        let tile_count_horizontal = self.render_tiles.tile_count_horizontal;

        for (chunk_index, target) in self.frame_buffer.chunks_exact_mut(tile_width).enumerate() {
            let tile_column = chunk_index % tile_count_horizontal;
            let tile_row = chunk_index / (tile_count_horizontal * tile_height);

            let tile_index = tile_row * tile_count_horizontal + tile_column;

            let source_start = ((chunk_index / tile_count_horizontal) % tile_height) * tile_width;
            let source_end = source_start + tile_width;

            let source = &self.render_tiles.tiles[tile_index]
                .frame_buffer
                .frame_buffer[source_start..source_end];
            target.copy_from_slice(source);
        }

        &self.frame_buffer
    }

    fn clip_to_screen(&self, clip_space_vertex: Vec4) -> Vec4 {
        // Move to cartesian coordinates
        // Sace the recip of W for perspective correction later
        let w_recip = clip_space_vertex.w.recip();
        let clip_space_vertex = clip_space_vertex / clip_space_vertex.w;

        // Convert NDC coordinates to screen space
        let screen_x = (clip_space_vertex.x + 1.0) * (self.screen_width as f32 / 2.0);
        let screen_y = (1.0 - clip_space_vertex.y) * (self.screen_height as f32 / 2.0);

        Vec4::new(screen_x, screen_y, clip_space_vertex.z, w_recip)
    }
}

fn is_backfacing(a: Vec4, b: Vec4, c: Vec4) -> bool {
    Mat3::from_cols(a.xyw(), b.xyw(), c.xyw()).determinant() < 0.0
}
