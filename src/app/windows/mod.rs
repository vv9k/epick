mod export;
mod settings;

pub use export::ExportWindow;
pub use settings::SettingsWindow;

#[derive(Debug)]
pub struct ShadesWindow {
    pub num_of_shades: u8,
    pub shade_color_size: f32,
}

impl Default for ShadesWindow {
    fn default() -> Self {
        Self {
            num_of_shades: 6,
            shade_color_size: 100.,
        }
    }
}

#[derive(Debug)]
pub struct TintsWindow {
    pub num_of_tints: u8,
    pub tint_color_size: f32,
}

impl Default for TintsWindow {
    fn default() -> Self {
        Self {
            num_of_tints: 6,
            tint_color_size: 100.,
        }
    }
}

#[derive(Debug)]
pub struct HuesWindow {
    pub num_of_hues: u8,
    pub hue_color_size: f32,
    pub hues_step: f32,
}

impl Default for HuesWindow {
    fn default() -> Self {
        Self {
            num_of_hues: 4,
            hue_color_size: 100.,
            hues_step: 0.05,
        }
    }
}
