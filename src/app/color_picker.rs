use crate::app::render::color_slider_1d;
use crate::app::sliders::ColorSliders;
use crate::color::{Cmyk, Color, ColorHarmony, Hsl, Lch, U8_MAX, U8_MIN};

use egui::Ui;
use egui::{
    color::{Hsva, HsvaGamma},
    DragValue, Rgba,
};

macro_rules! slider {
    ($it:ident, $ui:ident, $field:ident, $label:literal, $range:expr, $($tt:tt)+) => {
        $ui.add_space(7.);
        $ui.horizontal(|mut ui| {
            let resp = color_slider_1d(&mut ui, &mut $it.sliders.$field, $range, $($tt)+).on_hover_text($label);
            if resp.changed() {
                $it.check_color_change();
            }
            ui.add_space(7.);
            ui.label(format!("{}: ", $label));
            ui.add(DragValue::new(&mut $it.sliders.$field).clamp_range($range));
        });
    };
}

#[derive(Debug)]
pub struct ColorPicker {
    pub current_color: Color,
    pub hex_color: String,
    pub sliders: ColorSliders,
    pub saved_sliders: Option<ColorSliders>,
    pub scheme_color_size: f32,
    pub color_harmony: ColorHarmony,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            current_color: Color::black(),
            hex_color: "".to_string(),
            sliders: ColorSliders::default(),
            saved_sliders: None,
            scheme_color_size: 200.,
            color_harmony: ColorHarmony::Complementary,
        }
    }
}

impl ColorPicker {
    pub fn set_cur_color(&mut self, color: impl Into<Color>) {
        let color = color.into();
        self.sliders.set_color(color);
        self.current_color = color;
    }

    fn restore_sliders_if_saved(&mut self) {
        if let Some(saved) = std::mem::take(&mut self.saved_sliders) {
            self.sliders.restore(saved);
        }
    }

    fn save_sliders_if_unsaved(&mut self) {
        if self.saved_sliders.is_none() {
            self.saved_sliders = Some(self.sliders.clone());
        }
    }

    fn rgb_changed(&mut self) -> bool {
        let rgb = self.current_color.rgba();
        let r = self.sliders.r / U8_MAX;
        let g = self.sliders.g / U8_MAX;
        let b = self.sliders.b / U8_MAX;
        if (r - rgb.r()).abs() > f32::EPSILON
            || (g - rgb.g()).abs() > f32::EPSILON
            || (b - rgb.b()).abs() > f32::EPSILON
        {
            self.saved_sliders = None;
            self.set_cur_color(Rgba::from_rgb(r, g, b));
            true
        } else {
            false
        }
    }

    fn hsva_changed(&mut self) -> bool {
        let hsva = Hsva::from(self.current_color);
        if (self.sliders.hue - hsva.h).abs() > f32::EPSILON
            || (self.sliders.sat - hsva.s).abs() > f32::EPSILON
            || (self.sliders.val - hsva.v).abs() > f32::EPSILON
        {
            if self.sliders.val == 0. {
                self.save_sliders_if_unsaved();
            } else if self.sliders.val > 0. {
                self.restore_sliders_if_saved();
            }
            self.set_cur_color(Hsva::new(
                self.sliders.hue,
                self.sliders.sat,
                self.sliders.val,
                1.,
            ));
            true
        } else {
            false
        }
    }

    fn cmyk_changed(&mut self) -> bool {
        let cmyk = Cmyk::from(self.current_color);
        if (self.sliders.c - cmyk.c).abs() > f32::EPSILON
            || (self.sliders.m - cmyk.m).abs() > f32::EPSILON
            || (self.sliders.y - cmyk.y).abs() > f32::EPSILON
            || (self.sliders.k - cmyk.k).abs() > f32::EPSILON
        {
            if (self.sliders.k - 1.).abs() < f32::EPSILON {
                self.save_sliders_if_unsaved();
            } else if self.sliders.k < 1. {
                self.restore_sliders_if_saved();
            }
            self.set_cur_color(Cmyk::new(
                self.sliders.c,
                self.sliders.m,
                self.sliders.y,
                self.sliders.k,
            ));
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn lch_changed(&mut self) -> bool {
        let lch = Lch::from(self.current_color);
        if (self.sliders.lch_l - lch.l).abs() > f32::EPSILON
            || (self.sliders.lch_c - lch.c).abs() > f32::EPSILON
            || (self.sliders.lch_h - lch.h).abs() > f32::EPSILON
        {
            self.set_cur_color(Lch::new(
                self.sliders.lch_l,
                self.sliders.lch_c,
                self.sliders.lch_h,
            ));
            true
        } else {
            false
        }
    }

    fn hsl_changed(&mut self) -> bool {
        let hsl = Hsl::from(self.current_color);
        if (self.sliders.hsl_h - hsl.h).abs() > f32::EPSILON
            || (self.sliders.hsl_s - hsl.s).abs() > f32::EPSILON
            || (self.sliders.hsl_l - hsl.l).abs() > f32::EPSILON
        {
            self.set_cur_color(Hsl::new(
                self.sliders.hsl_h,
                self.sliders.hsl_s,
                self.sliders.hsl_l,
            ));
            true
        } else {
            false
        }
    }

    pub fn check_color_change(&mut self) {
        if self.rgb_changed() {
            return;
        }
        if self.hsva_changed() {
            return;
        }
        if self.cmyk_changed() {
            return;
        }
        self.hsl_changed();
    }

    pub fn rgb_sliders(&mut self, ui: &mut Ui) {
        let opaque = Rgba::from(self.current_color);
        ui.collapsing("RGB", |ui| {
            slider!(self, ui, r, "red", U8_MIN..=U8_MAX, |r| {
                Rgba::from_rgb(r, opaque.g(), opaque.b()).into()
            });
            slider!(self, ui, g, "green", U8_MIN..=U8_MAX, |g| {
                Rgba::from_rgb(opaque.r(), g, opaque.b()).into()
            });
            slider!(self, ui, b, "blue", U8_MIN..=U8_MAX, |b| {
                Rgba::from_rgb(opaque.r(), opaque.g(), b).into()
            });
        });
    }

    pub fn cmyk_sliders(&mut self, ui: &mut Ui) {
        let opaque = Cmyk::from(self.current_color);
        ui.collapsing("CMYK", |ui| {
            slider!(self, ui, c, "cyan", 0. ..=1., |c| Cmyk { c, ..opaque }
                .into());
            slider!(self, ui, m, "magenta", 0. ..=1., |m| Cmyk { m, ..opaque }
                .into());
            slider!(self, ui, y, "yellow", 0. ..=1., |y| Cmyk { y, ..opaque }
                .into());
            slider!(self, ui, k, "key", 0. ..=1., |k| Cmyk { k, ..opaque }
                .into());
        });
    }
    pub fn hsv_sliders(&mut self, ui: &mut Ui) {
        let mut opaque = HsvaGamma::from(self.current_color);
        opaque.a = 1.;
        ui.collapsing("HSV", |ui| {
            slider!(self, ui, hue, "hue", 0. ..=1., |h| HsvaGamma {
                h,
                ..opaque
            }
            .into());
            slider!(self, ui, sat, "saturation", 0. ..=1., |s| HsvaGamma {
                s,
                ..opaque
            }
            .into());
            slider!(self, ui, val, "value", 0. ..=1., |v| HsvaGamma {
                v,
                ..opaque
            }
            .into());
        });
    }
    pub fn hsl_sliders(&mut self, ui: &mut Ui) {
        let opaque = Hsl::from(self.current_color);
        ui.collapsing("HSL", |ui| {
            slider!(self, ui, hsl_h, "hue", 0. ..=1., |h| Hsl { h, ..opaque }
                .into());
            slider!(self, ui, hsl_s, "saturation", 0. ..=1., |s| Hsl {
                s,
                ..opaque
            }
            .into());
            slider!(self, ui, hsl_l, "light", 0. ..=1., |l| Hsl { l, ..opaque }
                .into());
        });
    }
}
