use crate::app::settings::Settings;
use crate::color::{
    ChromaticAdaptationMethod, ColorHarmony, DisplayFormat, Illuminant, RgbWorkingSpace,
};

use egui::{Color32, ComboBox, Ui, Window};
use std::fmt::Display;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;

#[derive(Debug, Default)]
pub struct SettingsWindow {
    pub show: bool,
    pub error: Option<String>,
    pub message: Option<String>,
    pub settings: Settings,
}

impl SettingsWindow {
    fn set_error(&mut self, error: impl Display) {
        self.clear_message();
        self.error = Some(error.to_string());
    }

    fn clear_error(&mut self) {
        self.error = None;
    }

    fn set_message(&mut self, message: impl Display) {
        self.clear_error();
        self.message = Some(message.to_string());
    }

    fn clear_message(&mut self) {
        self.message = None;
    }

    pub fn display(&mut self, ctx: &egui::CtxRef) {
        if self.show {
            let mut show = true;
            Window::new("settings").open(&mut show).show(ctx, |ui| {
                if let Some(err) = &self.error {
                    ui.colored_label(Color32::RED, err);
                }
                if let Some(msg) = &self.message {
                    ui.colored_label(Color32::GREEN, msg);
                }

                self.color_display_format(ui);
                self.rgb_working_space(ui);
                self.illuminant(ui);
                self.chromatic_adaptation_method(ui);
                self.color_harmony(ui);
                self.color_spaces(ui);

                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Save settings").clicked() {
                    if let Some(dir) = Settings::dir("epick") {
                        if !dir.exists() {
                            if let Err(e) = fs::create_dir_all(&dir) {
                                self.set_error(e);
                            }
                        }
                        let path = dir.join("config.yaml");
                        if let Err(e) = self.settings.save(&path) {
                            self.set_error(e);
                        } else {
                            self.set_message(format!(
                                "Successfully saved settings to {}",
                                path.display()
                            ));
                        }
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                ui.checkbox(&mut self.settings.cache_colors, "Cache colors");
            });

            if !show {
                self.show = false;
                self.clear_error();
                self.clear_message();
            }
        }
    }

    fn color_harmony(&mut self, ui: &mut Ui) {
        ComboBox::from_label("Color harmony")
            .selected_text(self.settings.color_harmony.as_ref())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.settings.color_harmony,
                    ColorHarmony::Complementary,
                    ColorHarmony::Complementary.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_harmony,
                    ColorHarmony::Triadic,
                    ColorHarmony::Triadic.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_harmony,
                    ColorHarmony::Tetradic,
                    ColorHarmony::Tetradic.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_harmony,
                    ColorHarmony::Analogous,
                    ColorHarmony::Analogous.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_harmony,
                    ColorHarmony::SplitComplementary,
                    ColorHarmony::SplitComplementary.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_harmony,
                    ColorHarmony::Square,
                    ColorHarmony::Square.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_harmony,
                    ColorHarmony::Monochromatic,
                    ColorHarmony::Monochromatic.as_ref(),
                );
            });
    }

    fn color_spaces(&mut self, ui: &mut Ui) {
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
    }

    fn illuminant(&mut self, ui: &mut Ui) {
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
    }

    fn chromatic_adaptation_method(&mut self, ui: &mut Ui) {
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
    }

    fn rgb_working_space(&mut self, ui: &mut Ui) {
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
    }

    fn color_display_format(&mut self, ui: &mut Ui) {
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
    }
}
