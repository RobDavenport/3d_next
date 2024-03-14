use std::ops::{Index, IndexMut};

use super::render_tile::RenderTile;

pub(super) struct TileManager<const W: usize, const H: usize, const PC: usize> {
    pub(super) tiles: Box<[RenderTile<W, H, PC>]>,
    pub(super) tile_count_horizontal: usize,
    pub(super) tile_count_vertical: usize,
}

impl<const W: usize, const H: usize, const PC: usize> Index<usize> for TileManager<W, H, PC> {
    type Output = RenderTile<W, H, PC>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl<const W: usize, const H: usize, const PC: usize> IndexMut<usize> for TileManager<W, H, PC> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

impl<const W: usize, const H: usize, const PC: usize> TileManager<W, H, PC> {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        if screen_width % W != 0 {
            panic!("Invalid tile width: {W} for screen width {screen_width}");
        } else if screen_height % H != 0 {
            panic!("Invalid tile height: {H} for screen height {screen_height}");
        }

        let tile_count_horizontal = screen_width / W;
        let tile_count_vertical = screen_height / H;
        let total_tile_count = tile_count_horizontal * tile_count_vertical;

        let tiles = (0..total_tile_count)
            .map(|i| {
                let y_tile = i / tile_count_horizontal;
                let x_tile = i % tile_count_horizontal;

                RenderTile::new(x_tile * W, y_tile * H)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            tiles,
            tile_count_horizontal,
            tile_count_vertical,
        }
    }

    pub const fn w(&self) -> usize {
        W
    }

    pub const fn h(&self) -> usize {
        H
    }

    pub fn reset_frame(&mut self) {
        self.tiles.iter_mut().for_each(|tile| {
            tile.frame_buffer.clear();
            tile.z_buffer.clear();
        });
    }
}
