use gamercade_rs::api::graphics_parameters::GraphicsParameters;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_graphics_params(self) -> GraphicsParameters {
        let a_level = self.r / 8;
        let g_level = self.g / 8;
        let b_level = self.b / 16;

        let g_palette = g_level / 4;
        let g_color = (g_level % 4) * 16;

        let r_palette = a_level * 8;

        GraphicsParameters::default()
            .palette_index(r_palette + g_palette)
            .color_index(g_color + b_level)
    }
}
