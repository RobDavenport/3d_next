use super::render_tile::RenderTile;

pub struct TileManager<const W: usize, const H: usize> {
    tiles: Box<[RenderTile<W, H>]>,
}

impl<const W: usize, const H: usize> TileManager<W, H> {
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
                let y_tile = i / H;
                let x_tile = i % W;

                RenderTile::new(x_tile * W, y_tile * H)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { tiles }
    }

    pub const fn w(&self) -> usize {
        W
    }

    pub const fn h(&self) -> usize {
        H
    }
}
