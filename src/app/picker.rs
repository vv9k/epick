use crate::app::render::{tex_color, TextureManager};
use crate::app::{color_tooltip, SavedColors};
use crate::color::{color_as_hex, parse_color, Cmyk};
use crate::save_to_clipboard;
use egui::color::{Color32, Hsva};
use egui::{vec2, Slider, Ui};

pub struct ColorPicker {
    pub hex_color: String,
    pub cur_color: Option<Color32>,
    pub cur_hsva: Option<Hsva>,
    pub cur_cmyk: Option<Cmyk>,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hue: f32,
    pub sat: f32,
    pub val: f32,
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
    pub tex_mngr: TextureManager,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            hex_color: "".to_string(),
            cur_color: Some(Color32::BLACK),
            cur_hsva: Some(Hsva::from_srgb([0, 0, 0])),
            cur_cmyk: Some(Cmyk::from(Color32::BLACK)),
            r: 0,
            g: 0,
            b: 0,
            hue: 0.,
            sat: 0.,
            val: 0.,
            c: 0.,
            m: 0.,
            y: 0.,
            k: 0.,
            tex_mngr: TextureManager::default(),
        }
    }
}

impl ColorPicker {
    pub fn set_cur_color(&mut self, color: Color32) {
        self.r = color.r();
        self.g = color.g();
        self.b = color.b();
        let hsva = Hsva::from_srgb([self.r, self.g, self.b]);
        self.hue = hsva.h;
        self.sat = hsva.s;
        self.val = hsva.v;
        let cmyk = Cmyk::from(color);
        self.c = cmyk.c;
        self.m = cmyk.m;
        self.y = cmyk.y;
        self.k = cmyk.k;
        self.cur_color = Some(color);
        self.cur_hsva = Some(hsva);
        self.cur_cmyk = Some(cmyk);
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
    ) {
        ui.horizontal(|ui| {
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
                if let Some(color) = self.cur_color {
                    saved_colors.add(color);
                }
            }
        });

        ui.add_space(15.);

        if let Some(color) = self.cur_color {
            ui.horizontal(|ui| {
                ui.label("Current color: ");
                ui.monospace(format!("#{}", color_as_hex(&color).to_uppercase()));
            });
            self.sliders(ui);
            ui.add_space(15.);

            if self.r != color.r() || self.g != color.g() || self.b != color.b() {
                self.set_cur_color(Color32::from_rgb(self.r, self.g, self.b));
            }

            // its ok to unwrap, cur_hsva is always set when cur_color is set
            let hsva = self.cur_hsva.unwrap();
            if self.hue != hsva.h || self.sat != hsva.s || self.val != hsva.v {
                let new_hsva = Hsva::new(self.hue, self.sat, self.val, 0.);
                let srgb = new_hsva.to_srgb();
                self.set_cur_color(Color32::from_rgb(srgb[0], srgb[1], srgb[2]));
            }

            let cmyk = self.cur_cmyk.clone().unwrap();
            if self.c != cmyk.c || self.m != cmyk.m || self.y != cmyk.y || self.k != cmyk.k {
                let new_cmyk = Cmyk::new(self.c, self.m, self.y, self.k);
                self.set_cur_color(Color32::from(new_cmyk));
            }

            ui.scope(|ui| {
                let resp = tex_color(
                    ui,
                    tex_allocator,
                    &mut self.tex_mngr,
                    color,
                    vec2(500., 500.),
                    Some(&color_tooltip(&color)),
                );
                if let Some(resp) = resp {
                    let hex = color_as_hex(&color);
                    if resp.clicked() {
                        self.set_cur_color(color);
                    }

                    if resp.middle_clicked() {
                        saved_colors.add(color);
                    }

                    if resp.secondary_clicked() {
                        let _ = save_to_clipboard(format!("#{}", hex));
                    }
                }
            });
        }
    }

    fn sliders(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.r, u8::MIN..=u8::MAX).text("red"));
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.g, u8::MIN..=u8::MAX).text("green"));
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.b, u8::MIN..=u8::MAX).text("blue"));
            });
            ui.vertical(|ui| {
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.hue, 0. ..=1.).text("hue"));
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.sat, 0. ..=1.).text("saturation"));
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.val, 0. ..=1.).text("value"));
            });
            ui.vertical(|ui| {
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.c, 0. ..=1.).text("cyan"));
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.m, 0. ..=1.).text("magenta"));
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.y, 0. ..=1.).text("yellow"));
                ui.add_space(7.);
                ui.add(Slider::new(&mut self.k, 0. ..=1.).text("key"));
            });
        });
    }
}
