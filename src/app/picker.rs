use crate::app::render::{color_slider_1d, tex_color, TextureManager};
use crate::app::{color_tooltip, SavedColors};
use crate::color::{Cmyk, Color};
use crate::save_to_clipboard;
use egui::{
    color::{Hsva, HsvaGamma},
    DragValue, Rgba, ScrollArea,
};
use egui::{vec2, Slider, Ui};

static MIN_COL_SIZE: f32 = 50.;
static ADD_ICON: &str = "➕";
static ADD_DESCR: &str = "Add this color to saved colors";

#[derive(Debug)]
pub struct ColorPicker {
    pub color_size: f32,
    pub hex_color: String,
    pub cur_color: Color,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub hue: f32,
    pub sat: f32,
    pub val: f32,
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
    pub tex_mngr: TextureManager,
    pub main_width: f32,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            color_size: 300.,
            hex_color: "".to_string(),
            cur_color: Color::black(),
            red: 0.,
            green: 0.,
            blue: 0.,
            hue: 0.,
            sat: 0.,
            val: 0.,
            c: 0.,
            m: 0.,
            y: 0.,
            k: 1.,
            tex_mngr: TextureManager::default(),
            main_width: 0.,
        }
    }
}

impl ColorPicker {
    pub fn set_cur_color(&mut self, color: Color) {
        let _color = Rgba::from(color);
        self.red = _color.r() * 255.;
        self.green = _color.g() * 255.;
        self.blue = _color.b() * 255.;
        let hsva = Hsva::from(_color);
        self.hue = hsva.h;
        self.sat = hsva.s;
        self.val = hsva.v;
        let cmyk = Cmyk::from(_color);
        self.c = cmyk.c;
        self.m = cmyk.m;
        self.y = cmyk.y;
        self.k = cmyk.k;
        self.cur_color = color;
    }

    fn check_color_change(&mut self) {
        let rgb = Rgba::from(self.cur_color);
        let r = self.red / 255.;
        let g = self.green / 255.;
        let b = self.blue / 255.;
        if (r - rgb.r()).abs() > f32::EPSILON
            || (g - rgb.g()).abs() > f32::EPSILON
            || (b - rgb.b()).abs() > f32::EPSILON
        {
            self.set_cur_color(Rgba::from_rgb(r, g, b).into());
            return;
        }

        let hsva = Hsva::from(self.cur_color);
        if (self.hue - hsva.h).abs() > f32::EPSILON
            || (self.sat - hsva.s).abs() > f32::EPSILON
            || (self.val - hsva.v).abs() > f32::EPSILON
        {
            let new_hsva = Hsva::new(self.hue, self.sat, self.val, 1.);
            self.set_cur_color(new_hsva.into());
            return;
        }

        let cmyk = Cmyk::from(self.cur_color);
        if (self.c - cmyk.c).abs() > f32::EPSILON
            || (self.m - cmyk.m).abs() > f32::EPSILON
            || (self.y - cmyk.y).abs() > f32::EPSILON
            || (self.k - cmyk.k).abs() > f32::EPSILON
        {
            let new_cmyk = Cmyk::new(self.c, self.m, self.y, self.k);
            self.set_cur_color(new_cmyk.into());
        }
    }
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
    ) {
        ui.label("Enter a hex color: ");
        let enter_bar = ui.horizontal(|ui| {
            let resp = ui.text_edit_singleline(&mut self.hex_color);
            if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                || ui.button("▶").on_hover_text("Use this color").clicked()
            {
                if let Some(color) = Color::from_hex(self.hex_color.trim_start_matches('#')) {
                    self.set_cur_color(color);
                }
            }
            if ui.button(ADD_ICON).on_hover_text(ADD_DESCR).clicked() {
                if let Some(color) = Color::from_hex(self.hex_color.trim_start_matches('#')) {
                    self.set_cur_color(color);
                }
            }
        });

        self.main_width = enter_bar.response.rect.width();

        ui.add_space(20.);

        let hex = self.cur_color.as_hex();

        ui.horizontal(|ui| {
            ui.label("Current color: ");
            ui.monospace(format!("#{}", hex.to_uppercase()));
            ui.add_space(7.);
            ui.add(Slider::new(&mut self.color_size, MIN_COL_SIZE..=1000.).text("color size"));
        });
        ui.horizontal(|ui| {
            if ui
                .button("📋")
                .on_hover_text("Copy hex color to clipboard")
                .clicked()
            {
                let _ = save_to_clipboard(hex.clone());
            }
            if ui.button(ADD_ICON).on_hover_text(ADD_DESCR).clicked() {
                saved_colors.add(self.cur_color);
            }
        });

        self.check_color_change();

        ScrollArea::auto_sized()
            .id_source("picker scroll")
            .show(ui, |ui| {
                self.sliders(ui);
                let resp = tex_color(
                    ui,
                    tex_allocator,
                    &mut self.tex_mngr,
                    self.cur_color.into(),
                    vec2(self.color_size, self.color_size),
                    Some(&color_tooltip(&self.cur_color)),
                );
                if let Some(resp) = resp {
                    if resp.clicked() {
                        self.set_cur_color(self.cur_color);
                    }

                    if resp.middle_clicked() {
                        saved_colors.add(self.cur_color);
                    }

                    if resp.secondary_clicked() {
                        let _ = save_to_clipboard(format!("#{}", hex));
                    }
                }
            });
    }

    fn sliders(&mut self, ui: &mut Ui) {
        macro_rules! slider {
            ($ui:ident, $it:ident, $label:literal, $range:expr, $($tt:tt)+) => {
                $ui.add_space(7.);
                $ui.horizontal(|mut ui| {
                    let resp = color_slider_1d(&mut ui, &mut self.$it, $range, $($tt)+).on_hover_text($label);
                    if resp.changed() {
                        self.check_color_change();
                    }
                    ui.add_space(7.);
                    ui.label(format!("{}: ", $label));
                    ui.add(DragValue::new(&mut self.$it));
                });
            };
        }
        ui.vertical(|ui| {
            ui.add_space(7.);
            ui.collapsing("RGB", |ui| {
                slider!(ui, red, "red", u8::MIN as f32..=u8::MAX as f32, |r| {
                    Rgba::from_rgb(r, 0., 0.).into()
                });
                slider!(ui, green, "green", u8::MIN as f32..=u8::MAX as f32, |g| {
                    Rgba::from_rgb(0., g, 0.).into()
                });
                slider!(ui, blue, "blue", u8::MIN as f32..=u8::MAX as f32, |b| {
                    Rgba::from_rgb(0., 0., b).into()
                });
            });

            ui.add_space(7.);
            ui.collapsing("CMYK", |ui| {
                slider!(ui, c, "cyan", 0. ..=1., |c| Cmyk::new(c, 0., 0., 0.).into());
                slider!(ui, m, "magenta", 0. ..=1., |m| Cmyk::new(0., m, 0., 0.)
                    .into());
                slider!(ui, y, "yellow", 0. ..=1., |y| Cmyk::new(0., 0., y, 0.)
                    .into());
                slider!(ui, k, "key", 0. ..=1., |k| Cmyk::new(0., 0., 0., k).into());
            });

            let mut opaque = HsvaGamma::from(self.cur_color);
            opaque.a = 1.;

            ui.add_space(7.);
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
        });
    }
}
