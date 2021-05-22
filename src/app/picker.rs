use crate::app::render::{color_slider_1d, tex_color, TextureManager};
use crate::app::{color_tooltip, SavedColors};
use crate::color::{color_as_hex, parse_color, Cmyk};
use crate::save_to_clipboard;
use egui::{
    color::{Color32, Hsva, HsvaGamma},
    Rgba,
};
use egui::{vec2, Slider, Ui};

static MIN_COL_SIZE: f32 = 450.;

#[derive(Debug)]
pub struct ColorPicker {
    pub color_size: f32,
    pub hex_color: String,
    pub cur_color: Color32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
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
            color_size: 600.,
            hex_color: "".to_string(),
            cur_color: Color32::BLACK,
            r: 0.,
            g: 0.,
            b: 0.,
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
    pub fn set_cur_color(&mut self, color: Color32) {
        let c = egui::Rgba::from(color);
        self.r = c[0];
        self.g = c[1];
        self.b = c[2];
        let hsva = Hsva::from(c);
        self.hue = hsva.h;
        self.sat = hsva.s;
        self.val = hsva.v;
        let cmyk = Cmyk::from(color);
        self.c = if cmyk.c.is_nan() { 0. } else { cmyk.c };
        self.m = if cmyk.m.is_nan() { 0. } else { cmyk.m };
        self.y = if cmyk.y.is_nan() { 0. } else { cmyk.y };
        self.k = if cmyk.k.is_nan() { 0. } else { cmyk.k };
        self.cur_color = color;
    }

    fn check_color_change(&mut self) {
        let c = Rgba::from(self.cur_color);
        if self.r != c.r() || self.g != c.g() || self.b != c.b() {
            self.set_cur_color(Rgba::from_rgb(self.r, self.g, self.b).into());
        }

        // its ok to unwrap, cur_hsva is always set when cur_color is set
        let hsva = Hsva::from(c);
        if self.hue != hsva.h || self.sat != hsva.s || self.val != hsva.v {
            let new_hsva = Hsva::new(self.hue, self.sat, self.val, 0.);
            let srgb = new_hsva.to_srgb();
            self.set_cur_color(Color32::from_rgb(srgb[0], srgb[1], srgb[2]));
        }

        let cmyk = Cmyk::from(self.cur_color);
        if self.c != cmyk.c || self.m != cmyk.m || self.y != cmyk.y || self.k != cmyk.k {
            let new_cmyk = Cmyk::new(self.c, self.m, self.y, self.k);
            self.set_cur_color(Color32::from(new_cmyk));
        }
    }
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
    ) {
        let enter_bar = ui.horizontal(|ui| {
            ui.label("Enter a hex color: ");
            let resp = ui.text_edit_singleline(&mut self.hex_color);
            if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                || ui.button("▶").on_hover_text("Use this color").clicked()
            {
                if let Some(color) = parse_color(self.hex_color.trim_start_matches("#")) {
                    self.set_cur_color(color);
                }
            }
            if ui
                .button("➕")
                .on_hover_text("Add this color to saved colors")
                .clicked()
            {
                if let Some(color) = parse_color(self.hex_color.trim_start_matches("#")) {
                    saved_colors.add(color);
                }
            }
        });

        self.main_width = enter_bar.response.rect.width();

        ui.add_space(20.);

        let hex = color_as_hex(&self.cur_color);

        ui.horizontal(|ui| {
            ui.label("Current color: ");
            ui.monospace(format!("#{}", hex.to_uppercase()));
            ui.add_space(7.);
            ui.add(Slider::new(&mut self.color_size, MIN_COL_SIZE..=1000.).text("color size"));
        });

        self.check_color_change();

        ui.horizontal(|ui| {
            ui.scope(|ui| {
                let resp = tex_color(
                    ui,
                    tex_allocator,
                    &mut self.tex_mngr,
                    self.cur_color,
                    vec2(self.color_size / 2., self.color_size),
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
            ui.add_space(20.);
            self.sliders(ui);
        });
    }

    fn sliders(&mut self, ui: &mut Ui) {
        macro_rules! slider {
            ($ui:ident, $it:ident, $label:literal, $($tt:tt)+) => {
                $ui.add_space(7.);
                $ui.horizontal(|mut ui| {
                    color_slider_1d(&mut ui, &mut self.$it, $($tt)+).on_hover_text($label);
                    ui.label($label);
                });
            };
        }
        ui.vertical(|ui| {
            ui.add_space(7.);
            ui.heading("RGB");
            slider!(ui, r, "red", |r| Rgba::from_rgb(r, 0., 0.).into());
            slider!(ui, g, "green", |g| Rgba::from_rgb(0., g, 0.).into());
            slider!(ui, b, "blue", |b| Rgba::from_rgb(0., 0., b).into());

            ui.add_space(7.);
            ui.heading("CMYK");
            slider!(ui, c, "cyan", |c| Cmyk::new(c, 0., 0., 0.).into());
            slider!(ui, m, "magenta", |m| Cmyk::new(0., m, 0., 0.).into());
            slider!(ui, y, "yellow", |y| Cmyk::new(0., 0., y, 0.).into());
            slider!(ui, k, "key", |k| Cmyk::new(0., 0., 0., k).into());

            let mut opaque = HsvaGamma::from(self.cur_color);
            opaque.a = 1.;

            ui.add_space(7.);
            ui.heading("HSV");
            slider!(ui, hue, "hue", |h| HsvaGamma { h, ..opaque }.into());
            slider!(ui, sat, "saturation", |s| HsvaGamma { s, ..opaque }.into());
            slider!(ui, val, "value", |v| HsvaGamma { v, ..opaque }.into());
        });
    }
}
