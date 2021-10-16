use crate::color::{ChromaticAdaptationMethod, DisplayFormat, Illuminant, RgbWorkingSpace};

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
    pub color_spaces: ColorSpaceSettings,
    pub rgb_working_space: RgbWorkingSpace,
    pub chromatic_adaptation_method: ChromaticAdaptationMethod,
    pub illuminant: Illuminant,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        let ws = RgbWorkingSpace::default();
        Self {
            show: false,
            color_display_format: DisplayFormat::Hex,
            color_spaces: ColorSpaceSettings::default(),
            rgb_working_space: ws,
            chromatic_adaptation_method: ChromaticAdaptationMethod::default(),
            illuminant: ws.reference_illuminant(),
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
                ComboBox::from_label("Illuminant")
                    .selected_text(self.illuminant.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::A,
                            Illuminant::A.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::B,
                            Illuminant::B.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::C,
                            Illuminant::C.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::D50,
                            Illuminant::D50.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::D55,
                            Illuminant::D55.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::D65,
                            Illuminant::D65.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::D75,
                            Illuminant::D75.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::E,
                            Illuminant::E.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::F2,
                            Illuminant::F2.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::F7,
                            Illuminant::F7.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.illuminant,
                            Illuminant::F11,
                            Illuminant::F11.as_ref(),
                        );
                    });
                ComboBox::from_label("Chromatic adaptation method")
                    .selected_text(self.chromatic_adaptation_method.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.chromatic_adaptation_method,
                            ChromaticAdaptationMethod::Bradford,
                            ChromaticAdaptationMethod::Bradford.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.chromatic_adaptation_method,
                            ChromaticAdaptationMethod::VonKries,
                            ChromaticAdaptationMethod::VonKries.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.chromatic_adaptation_method,
                            ChromaticAdaptationMethod::XYZScaling,
                            ChromaticAdaptationMethod::XYZScaling.as_ref(),
                        );
                    });

                ui.label("Colors spaces:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.color_spaces.rgb, "RGB");
                    ui.checkbox(&mut self.color_spaces.cmyk, "CMYK");
                    ui.checkbox(&mut self.color_spaces.hsv, "HSV");
                    ui.checkbox(&mut self.color_spaces.hsl, "HSL");
                });
                ui.label("CIE Color spaces:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.color_spaces.luv, "Luv");
                    ui.checkbox(&mut self.color_spaces.lch_uv, "LCH(uv)");
                    ui.checkbox(&mut self.color_spaces.lab, "Lab");
                    ui.checkbox(&mut self.color_spaces.lch_ab, "LCH(ab)");
                });
            });

            if !show {
                self.show = false;
            }
        }
    }
}
