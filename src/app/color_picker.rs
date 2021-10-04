use crate::app::render::color_slider_1d;
use crate::app::sliders::ColorSliders;
use crate::color::{Cmyk, Color, Hsl, Lch, SchemeType};

use egui::Ui;
use egui::{
    color::{Hsva, HsvaGamma},
    DragValue, Rgba,
};

#[derive(Debug)]
pub struct ColorPicker {
    pub current_color: Color,
    pub hex_color: String,
    pub sliders: ColorSliders,
    pub saved_sliders: Option<ColorSliders>,
    pub scheme_color_size: f32,
    pub scheme_type: SchemeType,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            current_color: Color::black(),
            hex_color: "".to_string(),
            sliders: ColorSliders::default(),
            saved_sliders: None,
            scheme_color_size: 200.,
            scheme_type: SchemeType::Complementary,
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
        let r = self.sliders.r / u8::MAX as f32;
        let g = self.sliders.g / u8::MAX as f32;
        let b = self.sliders.b / u8::MAX as f32;
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

    pub fn sliders(&mut self, ui: &mut Ui) {
        macro_rules! slider {
            ($ui:ident, $it:ident, $label:literal, $range:expr, $($tt:tt)+) => {
                $ui.add_space(7.);
                $ui.horizontal(|mut ui| {
                    let resp = color_slider_1d(&mut ui, &mut self.sliders.$it, $range, $($tt)+).on_hover_text($label);
                    if resp.changed() {
                        self.check_color_change();
                    }
                    ui.add_space(7.);
                    ui.label(format!("{}: ", $label));
                    ui.add(DragValue::new(&mut self.sliders.$it).clamp_range($range));
                });
            };
        }
        ui.vertical(|ui| {
            ui.collapsing("RGB", |ui| {
                slider!(ui, r, "red", u8::MIN as f32..=u8::MAX as f32, |r| {
                    Rgba::from_rgb(r, 0., 0.).into()
                });
                slider!(ui, g, "green", u8::MIN as f32..=u8::MAX as f32, |g| {
                    Rgba::from_rgb(0., g, 0.).into()
                });
                slider!(ui, b, "blue", u8::MIN as f32..=u8::MAX as f32, |b| {
                    Rgba::from_rgb(0., 0., b).into()
                });
            });

            ui.collapsing("CMYK", |ui| {
                slider!(ui, c, "cyan", 0. ..=1., |c| Cmyk::new(c, 0., 0., 0.).into());
                slider!(ui, m, "magenta", 0. ..=1., |m| Cmyk::new(0., m, 0., 0.)
                    .into());
                slider!(ui, y, "yellow", 0. ..=1., |y| Cmyk::new(0., 0., y, 0.)
                    .into());
                slider!(ui, k, "key", 0. ..=1., |k| Cmyk::new(0., 0., 0., k).into());
            });

            let mut opaque = HsvaGamma::from(self.current_color);
            opaque.a = 1.;

            ui.collapsing("HSV", |ui| {
                slider!(ui, hue, "hue", 0. ..=1., |h| HsvaGamma { h, ..opaque }
                    .into());
                slider!(ui, sat, "saturation", 0. ..=1., |s| HsvaGamma {
                    s,
                    ..opaque
                }
                .into());
                slider!(ui, val, "value", 0. ..=1., |v| HsvaGamma { v, ..opaque }
                    .into());
            });

            let opaque = Hsl::from(self.current_color);

            ui.collapsing("HSL", |ui| {
                slider!(ui, hsl_h, "hue", 0. ..=1., |h| Hsl { h, ..opaque }.into());
                slider!(ui, hsl_s, "saturation", 0. ..=1., |s| Hsl { s, ..opaque }
                    .into());
                slider!(ui, hsl_l, "light", 0. ..=1., |l| Hsl { l, ..opaque }.into());
            });

            // let opaque = Lch::from(self.cur_color);
            // ui.collapsing("LCH", |ui| {
            //     slider!(ui, lch_l, "lightness", 0. ..=100., |l| Lch { l, ..opaque }
            //         .into());
            //     slider!(ui, lch_c, "colorfulness", 0. ..=600., |c| Lch {
            //         c,
            //         ..opaque
            //     }
            //     .into());
            //     slider!(ui, lch_h, "hue", 0. ..=360., |h| Lch { h, ..opaque }.into());
            // });
        });
    }
}
