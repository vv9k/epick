use crate::color::DisplayFormat;

use egui::{ComboBox, Window};

#[derive(Debug)]
pub struct SettingsWindow {
    pub show: bool,
    pub color_display_format: DisplayFormat,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self {
            show: false,
            color_display_format: DisplayFormat::Hex,
        }
    }
}

impl SettingsWindow {
    pub fn display(&mut self, ctx: &egui::CtxRef) {
        if self.show {
            let mut show = true;
            Window::new("settings").open(&mut show).show(ctx, |ui| {
                ComboBox::from_label("Color display format")
                    .selected_text(self.color_display_format.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.color_display_format,
                            DisplayFormat::Hex,
                            DisplayFormat::Hex.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.color_display_format,
                            DisplayFormat::HexUpercase,
                            DisplayFormat::HexUpercase.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.color_display_format,
                            DisplayFormat::CssRgb,
                            DisplayFormat::CssRgb.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.color_display_format,
                            DisplayFormat::CssHsl,
                            DisplayFormat::CssHsl.as_ref(),
                        );
                    });
            });

            if !show {
                self.show = false;
            }
        }
    }
}
