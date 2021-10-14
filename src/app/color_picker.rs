use crate::app::render::color_slider_1d;
use crate::app::sliders::ColorSliders;
use crate::color::{
    CIEColor, Cmyk, Color, ColorHarmony, Hsl, Hsv, Lch, Luv, Rgb, Xyz, U8_MAX, U8_MIN,
};

use egui::Ui;
use egui::{color::Hsva, DragValue};

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

    pub fn set_cie_color(&mut self, color: impl CIEColor) {
        let color = color.to_rgb(self.sliders.rgb_working_space).into();
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
        let rgb = self.current_color.rgb();
        let r = self.sliders.r;
        let g = self.sliders.g;
        let b = self.sliders.b;
        if (r - rgb.r_scaled()).abs() > f32::EPSILON
            || (g - rgb.g_scaled()).abs() > f32::EPSILON
            || (b - rgb.b_scaled()).abs() > f32::EPSILON
        {
            self.saved_sliders = None;
            self.set_cur_color(Rgb::new(r, g, b));
            true
        } else {
            false
        }
    }

    fn cmyk_changed(&mut self) -> bool {
        let cmyk = Cmyk::from(self.current_color);
        if (self.sliders.c - cmyk.c_scaled()).abs() > f32::EPSILON
            || (self.sliders.m - cmyk.m_scaled()).abs() > f32::EPSILON
            || (self.sliders.y - cmyk.y_scaled()).abs() > f32::EPSILON
            || (self.sliders.k - cmyk.k_scaled()).abs() > f32::EPSILON
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

    fn hsv_changed(&mut self) -> bool {
        let hsv = Hsv::from(self.current_color);
        if (self.sliders.hue - hsv.h_scaled()).abs() > f32::EPSILON
            || (self.sliders.sat - hsv.s_scaled()).abs() > f32::EPSILON
            || (self.sliders.val - hsv.v_scaled()).abs() > f32::EPSILON
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

    fn hsl_changed(&mut self) -> bool {
        let hsl = Hsl::from(self.current_color);
        if (self.sliders.hsl_h - hsl.h_scaled()).abs() > f32::EPSILON
            || (self.sliders.hsl_s - hsl.s_scaled()).abs() > f32::EPSILON
            || (self.sliders.hsl_l - hsl.l_scaled()).abs() > f32::EPSILON
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

    fn luv_changed(&mut self) -> bool {
        let luv = Luv::from(self.current_color.xyz(self.sliders.rgb_working_space));
        if (self.sliders.luv_l - luv.l()).abs() > f32::EPSILON
            || (self.sliders.luv_u - luv.u()).abs() > f32::EPSILON
            || (self.sliders.luv_v - luv.v()).abs() > f32::EPSILON
        {
            self.set_cie_color(Luv::new(
                self.sliders.luv_l,
                self.sliders.luv_u,
                self.sliders.luv_v,
            ));
            true
        } else {
            false
        }
    }

    fn lch_changed(&mut self) -> bool {
        let lch = Lch::from(self.current_color.xyz(self.sliders.rgb_working_space));
        if (self.sliders.lch_l - lch.l()).abs() > f32::EPSILON
            || (self.sliders.lch_c - lch.c()).abs() > f32::EPSILON
            || (self.sliders.lch_h - lch.h()).abs() > f32::EPSILON
        {
            self.set_cie_color(Lch::new(
                self.sliders.lch_l,
                self.sliders.lch_c,
                self.sliders.lch_h,
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
        if self.cmyk_changed() {
            return;
        }
        if self.hsv_changed() {
            return;
        }
        if self.hsl_changed() {
            return;
        }
        if self.luv_changed() {
            return;
        }
        self.lch_changed();
    }

    pub fn rgb_sliders(&mut self, ui: &mut Ui) {
        let opaque = self.current_color.rgb();
        ui.collapsing("RGB", |ui| {
            slider!(self, ui, r, "red", U8_MIN..=U8_MAX, |r| {
                Rgb::new(r, opaque.g(), opaque.b()).into()
            });
            slider!(self, ui, g, "green", U8_MIN..=U8_MAX, |g| {
                Rgb::new(opaque.r(), g, opaque.b()).into()
            });
            slider!(self, ui, b, "blue", U8_MIN..=U8_MAX, |b| {
                Rgb::new(opaque.r(), opaque.g(), b).into()
            });
        });
    }

    pub fn cmyk_sliders(&mut self, ui: &mut Ui) {
        let opaque = self.current_color.cmyk();
        ui.collapsing("CMYK", |ui| {
            slider!(self, ui, c, "cyan", 0. ..=100., |c| {
                Cmyk::new(c, opaque.m(), opaque.y(), opaque.k()).into()
            });
            slider!(self, ui, m, "magenta", 0. ..=100., |m| {
                Cmyk::new(opaque.c(), m, opaque.y(), opaque.k()).into()
            });
            slider!(self, ui, y, "yellow", 0. ..=100., |y| {
                Cmyk::new(opaque.c(), opaque.m(), y, opaque.k()).into()
            });
            slider!(self, ui, k, "key", 0. ..=100., |k| Cmyk::new(
                opaque.c(),
                opaque.m(),
                opaque.y(),
                k
            )
            .into());
        });
    }

    pub fn hsv_sliders(&mut self, ui: &mut Ui) {
        let opaque = self.current_color.hsv();
        ui.collapsing("HSV", |ui| {
            slider!(self, ui, hue, "hue", 0. ..=360., |h| {
                Hsv::new(h, opaque.s(), opaque.v()).into()
            });
            slider!(self, ui, sat, "saturation", 0. ..=100., |s| {
                Hsv::new(opaque.h(), s, opaque.v()).into()
            });
            slider!(self, ui, val, "value", 0. ..=100., |v| {
                Hsv::new(opaque.h(), opaque.s(), v).into()
            });
        });
    }

    pub fn hsl_sliders(&mut self, ui: &mut Ui) {
        let opaque = self.current_color.hsl();
        ui.collapsing("HSL", |ui| {
            slider!(self, ui, hsl_h, "hue", 0. ..=360., |h| {
                Hsl::new(h, opaque.s(), opaque.l()).into()
            });
            slider!(self, ui, hsl_s, "saturation", 0. ..=100., |s| {
                Hsl::new(opaque.h(), s, opaque.l()).into()
            });
            slider!(self, ui, hsl_l, "light", 0. ..=100., |l| {
                Hsl::new(opaque.h(), opaque.s(), l).into()
            });
        });
    }

    pub fn luv_sliders(&mut self, ui: &mut Ui) {
        let ws = self.sliders.rgb_working_space;
        let opaque = Luv::from(Xyz::from_rgb(self.current_color.rgb(), ws));
        ui.collapsing("Luv", |ui| {
            slider!(self, ui, luv_l, "light", 0. ..=100., |l| {
                Luv::new(l, opaque.u(), opaque.v()).to_rgb(ws).into()
            });
            slider!(self, ui, luv_u, "u", -134. ..=220., |u| {
                Luv::new(opaque.l(), u, opaque.v()).to_rgb(ws).into()
            });
            slider!(self, ui, luv_v, "v", -140. ..=122., |v| {
                Luv::new(opaque.l(), opaque.u(), v).to_rgb(ws).into()
            });
        });
    }

    pub fn lch_sliders(&mut self, ui: &mut Ui) {
        let ws = self.sliders.rgb_working_space;
        let opaque = Lch::from(Luv::from(Xyz::from_rgb(self.current_color.rgb(), ws)));
        ui.collapsing("LCH(uv)", |ui| {
            slider!(self, ui, lch_l, "light", 0. ..=100., |l| {
                Lch::new(l, opaque.c(), opaque.h()).to_rgb(ws).into()
            });
            slider!(self, ui, lch_c, "c", 0. ..=270., |c| {
                Lch::new(opaque.l(), c, opaque.h()).to_rgb(ws).into()
            });
            slider!(self, ui, lch_h, "h", 0. ..=360., |h| {
                Lch::new(opaque.l(), opaque.c(), h).to_rgb(ws).into()
            });
        });
    }
}
