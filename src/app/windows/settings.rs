use crate::color::{DisplayFormat, RgbWorkingSpace};

use egui::{ComboBox, Window};

#[derive(Debug)]
pub struct ColorSpaceSettings {
    pub rgb: bool,
    pub cmyk: bool,
    pub hsv: bool,
    pub hsl: bool,
    pub luv: bool,
    pub lch_uv: bool,
    pub lab: bool,
    pub lch_ab: bool,
}

impl Default for ColorSpaceSettings {
    fn default() -> Self {
        Self {
            rgb: true,
            cmyk: true,
            hsv: true,
            hsl: true,
            luv: false,
            lch_uv: false,
            lab: false,
            lch_ab: false,
        }
    }
}

#[derive(Debug)]
pub struct SettingsWindow {
    pub show: bool,
    pub color_display_format: DisplayFormat,
    pub colorspaces: ColorSpaceSettings,
    pub rgb_working_space: RgbWorkingSpace,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self {
            show: false,
            color_display_format: DisplayFormat::Hex,
            colorspaces: ColorSpaceSettings::default(),
            rgb_working_space: RgbWorkingSpace::default(),
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
                            DisplayFormat::CssHsl {
                                degree_symbol: true,
                            },
                            DisplayFormat::CssHsl {
                                degree_symbol: true,
                            }
                            .as_ref(),
                        );
                    });

                ComboBox::from_label("RGB Working Space")
                    .selected_text(self.rgb_working_space.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::Adobe,
                            RgbWorkingSpace::Adobe.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::Apple,
                            RgbWorkingSpace::Apple.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::CIE,
                            RgbWorkingSpace::CIE.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::ECI,
                            RgbWorkingSpace::ECI.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::NTSC,
                            RgbWorkingSpace::NTSC.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::PAL,
                            RgbWorkingSpace::PAL.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::ProPhoto,
                            RgbWorkingSpace::ProPhoto.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::SRGB,
                            RgbWorkingSpace::SRGB.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.rgb_working_space,
                            RgbWorkingSpace::WideGamut,
                            RgbWorkingSpace::WideGamut.as_ref(),
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
                    ui.checkbox(&mut self.colorspaces.lch_uv, "LCH(uv)");
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.colorspaces.lab, "Lab");
                    ui.checkbox(&mut self.colorspaces.lch_ab, "LCH(ab)");
                });
            });

            if !show {
                self.show = false;
            }
        }
    }
}
