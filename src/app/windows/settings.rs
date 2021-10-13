use crate::color::DisplayFormat;

use egui::{ComboBox, Window};

#[derive(Debug)]
pub struct ColorSpaceSettings {
    pub rgb: bool,
    pub cmyk: bool,
    pub hsv: bool,
    pub hsl: bool,
    pub luv: bool,
    pub lch: bool,
}

impl Default for ColorSpaceSettings {
    fn default() -> Self {
        Self {
            rgb: true,
            cmyk: true,
            hsv: true,
            hsl: true,
            luv: false,
            lch: false,
        }
    }
}

#[derive(Debug)]
pub struct SettingsWindow {
    pub show: bool,
    pub color_display_format: DisplayFormat,
    pub colorspaces: ColorSpaceSettings,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self {
            show: false,
            color_display_format: DisplayFormat::Hex,
            colorspaces: ColorSpaceSettings::default(),
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

                ui.label("Colorspaces:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.colorspaces.rgb, "RGB");
                    ui.checkbox(&mut self.colorspaces.cmyk, "CMYK");
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.colorspaces.hsv, "HSV");
                    ui.checkbox(&mut self.colorspaces.hsl, "HSL");
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.colorspaces.luv, "Luv");
                    ui.checkbox(&mut self.colorspaces.lch, "LCH(uv)");
                });
            });

            if !show {
                self.show = false;
            }
        }
    }
}
