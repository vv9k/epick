use crate::app::settings::{DisplayFmtEnum, Settings};
use crate::app::ui::windows::{self, WINDOW_X_OFFSET, WINDOW_Y_OFFSET};
use crate::color::{ChromaticAdaptationMethod, ColorHarmony, Illuminant, RgbWorkingSpace};

use egui::{Color32, ComboBox, CursorIcon, Ui, Window};
use std::fmt::Display;

use crate::app::ui::{DOUBLE_SPACE, HALF_SPACE, SPACE};
use crate::app::{ADD_ICON, DELETE_ICON};
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

#[derive(Debug, Default)]
pub struct SettingsWindow {
    pub show: bool,
    pub error: Option<String>,
    pub message: Option<String>,
    pub settings: Settings,
    selected_display_fmt: String,
    selected_clipboard_fmt: String,
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
            let offset = ctx.style().spacing.slider_width * WINDOW_X_OFFSET;
            let mut show = true;
            let is_dark_mode = ctx.style().visuals.dark_mode;
            Window::new("settings")
                .frame(windows::default_frame(is_dark_mode))
                .open(&mut show)
                .default_pos((offset, WINDOW_Y_OFFSET))
                .show(ctx, |ui| {
                    windows::apply_default_style(ui, is_dark_mode);
                    if let Some(err) = &self.error {
                        ui.colored_label(Color32::RED, err);
                    }
                    if let Some(msg) = &self.message {
                        ui.colored_label(Color32::GREEN, msg);
                    }

                    self.color_display_format(ui);
                    ui.add_space(HALF_SPACE);
                    self.rgb_working_space(ui);
                    ui.add_space(HALF_SPACE);
                    self.illuminant(ui);
                    ui.add_space(HALF_SPACE);
                    self.chromatic_adaptation_method(ui);
                    ui.add_space(HALF_SPACE);
                    self.color_harmony(ui);
                    ui.add_space(HALF_SPACE);
                    ui.checkbox(&mut self.settings.cache_colors, "Cache colors");
                    ui.add_space(DOUBLE_SPACE);
                    self.color_spaces(ui);
                    ui.add_space(SPACE);
                    self.save_settings_btn(ui);
                });

            if !show {
                self.show = false;
                self.clear_error();
                self.clear_message();
            }
        }
    }

    fn save_settings_btn(&mut self, ui: &mut Ui) {
        #[cfg(not(target_arch = "wasm32"))]
        if ui
            .button("Save settings")
            .on_hover_cursor(CursorIcon::PointingHand)
            .clicked()
        {
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
                    self.set_message(format!("Successfully saved settings to {}", path.display()));
                }
            }
        }
    }

    fn color_harmony(&mut self, ui: &mut Ui) {
        ComboBox::from_label("Color harmony")
            .selected_text(self.settings.harmony.as_ref())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.settings.harmony,
                    ColorHarmony::Complementary,
                    ColorHarmony::Complementary.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.harmony,
                    ColorHarmony::Triadic,
                    ColorHarmony::Triadic.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.harmony,
                    ColorHarmony::Tetradic,
                    ColorHarmony::Tetradic.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.harmony,
                    ColorHarmony::Analogous,
                    ColorHarmony::Analogous.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.harmony,
                    ColorHarmony::SplitComplementary,
                    ColorHarmony::SplitComplementary.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.harmony,
                    ColorHarmony::Square,
                    ColorHarmony::Square.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.harmony,
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
        ui.add_space(SPACE);
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
                    DisplayFmtEnum::Hex,
                    DisplayFmtEnum::Hex.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_display_format,
                    DisplayFmtEnum::HexUppercase,
                    DisplayFmtEnum::HexUppercase.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_display_format,
                    DisplayFmtEnum::CssRgb,
                    DisplayFmtEnum::CssRgb.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_display_format,
                    DisplayFmtEnum::CssHsl,
                    DisplayFmtEnum::CssHsl.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings.color_display_format,
                    DisplayFmtEnum::Custom,
                    DisplayFmtEnum::Custom.as_ref(),
                );
            });
        if let DisplayFmtEnum::Custom = self.settings.color_display_format {
            macro_rules! display_fmt {
                ($label:literal, $id:literal, $selected:ident, $editable:ident) => {
                    ui.label($label);
                    if !self.settings.saved_color_formats.is_empty() {
                        ui.horizontal(|ui| {
                            ComboBox::from_id_source($id)
                                .selected_text(&self.$selected)
                                .show_ui(ui, |ui| {
                                    for fmt in &self.settings.saved_color_formats {
                                        if ui
                                            .selectable_label(&self.$selected == fmt, fmt)
                                            .clicked()
                                        {
                                            self.$selected = fmt.clone();
                                            self.settings.$editable = fmt.clone();
                                        }
                                    }
                                });
                            if ui
                                .button(DELETE_ICON)
                                .on_hover_text("Delete selected format string")
                                .on_hover_cursor(CursorIcon::Default)
                                .clicked()
                            {
                                let pos = self
                                    .settings
                                    .saved_color_formats
                                    .iter()
                                    .position(|fmt| fmt == &self.$selected);
                                if let Some(pos) = pos {
                                    self.settings.saved_color_formats.remove(pos);
                                    let fmt = if pos > 0 {
                                        Some(self.settings.saved_color_formats[pos - 1].clone())
                                    } else {
                                        self.settings.saved_color_formats.first().cloned()
                                    };
                                    if let Some(fmt) = fmt {
                                        self.$selected = fmt.clone();
                                        self.settings.$editable = fmt;
                                    }
                                }
                            }
                        });
                    }
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.settings.$editable);
                        if ui
                            .button(ADD_ICON)
                            .on_hover_text("Save current format string")
                            .on_hover_cursor(CursorIcon::Copy)
                            .clicked()
                        {
                            self.settings
                                .saved_color_formats
                                .push(self.settings.$editable.clone());
                        }
                    });
                };
            }

            ui.add_space(SPACE);
            display_fmt!(
                "Custom display format",
                "display saved formats",
                selected_display_fmt,
                custom_display_fmt_str
            );
            ui.add_space(SPACE);
            display_fmt!(
                "Custom clipboard format",
                "clipboard saved formats",
                selected_clipboard_fmt,
                custom_clipboard_fmt_str
            );
            ui.add_space(DOUBLE_SPACE);
        }
    }
}
