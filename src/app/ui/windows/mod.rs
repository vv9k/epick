mod export;
mod help;
mod settings;

use egui::{Slider, Ui};
pub use export::ExportWindow;
pub use help::HelpWindow;
pub use settings::SettingsWindow;

#[derive(Debug)]
pub struct ShadesWindow {
    pub is_open: bool,
    pub num_of_shades: u8,
    pub shade_color_size: f32,
}

impl Default for ShadesWindow {
    fn default() -> Self {
        Self {
            is_open: false,
            num_of_shades: 6,
            shade_color_size: 100.,
        }
    }
}

impl ShadesWindow {
    pub fn sliders(&mut self, ui: &mut Ui) {
        ui.add(
            Slider::new(&mut self.num_of_shades, u8::MIN..=50)
                .clamp_to_range(true)
                .text("# of shades"),
        );
        ui.add(
            Slider::new(&mut self.shade_color_size, 20.0..=200.)
                .clamp_to_range(true)
                .text("color size"),
        );
    }
}

#[derive(Debug)]
pub struct TintsWindow {
    pub is_open: bool,
    pub num_of_tints: u8,
    pub tint_color_size: f32,
}

impl Default for TintsWindow {
    fn default() -> Self {
        Self {
            is_open: false,
            num_of_tints: 6,
            tint_color_size: 100.,
        }
    }
}

impl TintsWindow {
    pub fn sliders(&mut self, ui: &mut Ui) {
        ui.add(
            Slider::new(&mut self.num_of_tints, u8::MIN..=50)
                .clamp_to_range(true)
                .text("# of tints"),
        );
        ui.add(
            Slider::new(&mut self.tint_color_size, 20.0..=200.)
                .clamp_to_range(true)
                .text("color size"),
        );
    }
}

#[derive(Debug)]
pub struct HuesWindow {
    pub is_open: bool,
    pub num_of_hues: u8,
    pub hue_color_size: f32,
    pub hues_step: f32,
}

impl Default for HuesWindow {
    fn default() -> Self {
        Self {
            is_open: false,
            num_of_hues: 4,
            hue_color_size: 100.,
            hues_step: 0.05,
        }
    }
}

impl HuesWindow {
    pub fn sliders(&mut self, ui: &mut Ui) {
        ui.add(
            Slider::new(&mut self.hues_step, 0.01..=0.1)
                .clamp_to_range(true)
                .text("step"),
        );
        let max_hues = (0.5 / self.hues_step).round() as u8;
        if self.num_of_hues > max_hues {
            self.num_of_hues = max_hues;
        }
        ui.add(
            Slider::new(&mut self.num_of_hues, u8::MIN..=max_hues)
                .clamp_to_range(true)
                .text("# of hues"),
        );
        ui.add(
            Slider::new(&mut self.hue_color_size, 20.0..=200.)
                .clamp_to_range(true)
                .text("color size"),
        );
    }
}
