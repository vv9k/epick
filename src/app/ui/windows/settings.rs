use crate::app::settings::Settings;
use crate::color::{ChromaticAdaptationMethod, DisplayFormat, Illuminant, RgbWorkingSpace};

use egui::{ComboBox, Window};

#[derive(Debug, Default)]
pub struct SettingsWindow {
    pub show: bool,
    pub settings: Settings,
}

impl SettingsWindow {
    pub fn display(&mut self, ctx: &egui::CtxRef) {
        if self.show {
            let mut show = true;
            Window::new("settings").open(&mut show).show(ctx, |ui| {
                ComboBox::from_label("Color display format")
                    .selected_text(self.settings.color_display_format.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.settings.color_display_format,
                            DisplayFormat::Hex,
                            DisplayFormat::Hex.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.color_display_format,
                            DisplayFormat::HexUpercase,
                            DisplayFormat::HexUpercase.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.color_display_format,
                            DisplayFormat::CssRgb,
                            DisplayFormat::CssRgb.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.color_display_format,
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
                    .selected_text(self.settings.rgb_working_space.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::Adobe,
                            RgbWorkingSpace::Adobe.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::Apple,
                            RgbWorkingSpace::Apple.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::CIE,
                            RgbWorkingSpace::CIE.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::ECI,
                            RgbWorkingSpace::ECI.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::NTSC,
                            RgbWorkingSpace::NTSC.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::PAL,
                            RgbWorkingSpace::PAL.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::ProPhoto,
                            RgbWorkingSpace::ProPhoto.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::SRGB,
                            RgbWorkingSpace::SRGB.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.rgb_working_space,
                            RgbWorkingSpace::WideGamut,
                            RgbWorkingSpace::WideGamut.as_ref(),
                        );
                    });
                ComboBox::from_label("Illuminant")
                    .selected_text(self.settings.illuminant.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::A,
                            Illuminant::A.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::B,
                            Illuminant::B.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::C,
                            Illuminant::C.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::D50,
                            Illuminant::D50.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::D55,
                            Illuminant::D55.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::D65,
                            Illuminant::D65.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::D75,
                            Illuminant::D75.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::E,
                            Illuminant::E.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::F2,
                            Illuminant::F2.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::F7,
                            Illuminant::F7.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.illuminant,
                            Illuminant::F11,
                            Illuminant::F11.as_ref(),
                        );
                    });
                ComboBox::from_label("Chromatic adaptation method")
                    .selected_text(self.settings.chromatic_adaptation_method.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.settings.chromatic_adaptation_method,
                            ChromaticAdaptationMethod::Bradford,
                            ChromaticAdaptationMethod::Bradford.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.chromatic_adaptation_method,
                            ChromaticAdaptationMethod::VonKries,
                            ChromaticAdaptationMethod::VonKries.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.settings.chromatic_adaptation_method,
                            ChromaticAdaptationMethod::XYZScaling,
                            ChromaticAdaptationMethod::XYZScaling.as_ref(),
                        );
                    });

                ui.label("Colors spaces:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.settings.color_spaces.rgb, "RGB");
                    ui.checkbox(&mut self.settings.color_spaces.cmyk, "CMYK");
                    ui.checkbox(&mut self.settings.color_spaces.hsv, "HSV");
                    ui.checkbox(&mut self.settings.color_spaces.hsl, "HSL");
                });
                ui.label("CIE Color spaces:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.settings.color_spaces.luv, "Luv");
                    ui.checkbox(&mut self.settings.color_spaces.lch_uv, "LCH(uv)");
                    ui.checkbox(&mut self.settings.color_spaces.lab, "Lab");
                    ui.checkbox(&mut self.settings.color_spaces.lch_ab, "LCH(ab)");
                });
            });

            if !show {
                self.show = false;
            }
        }
    }
}
