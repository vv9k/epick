use crate::color::{color_as_hex, parse_color, Cmyk, Gradient};
use egui::color::*;
use egui::{
    pos2, vec2, Button, ImageButton, Rect, Response, ScrollArea, Slider, TextStyle, TextureId, Ui,
    Vec2,
};
use std::collections::HashMap;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

#[derive(Default)]
pub struct Epick {
    pub tab: EpickApp,
    pub picker: ColorPicker,
    pub saved_colors: Vec<(String, Color32)>,
}

pub enum EpickApp {
    ColorPicker,
    GradientView,
}

impl Default for EpickApp {
    fn default() -> Self {
        Self::ColorPicker
    }
}

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
            cur_hsva: Some(Hsva::new(0., 0., 0., 1.)),
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

fn save_to_clipboard(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text)
}

impl epi::App for Epick {
    fn name(&self) -> &str {
        "epick"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let _frame = egui::Frame {
            fill: Color32::from_rgb(22, 28, 35),
            margin: vec2(20., 20.),
            ..Default::default()
        };

        let _dark_frame_side = egui::Frame {
            fill: Color32::from_rgb(17, 22, 27),
            margin: vec2(15., 10.),
            ..Default::default()
        };

        let _dark_frame_top = egui::Frame {
            fill: Color32::from_rgb(17, 22, 27),
            margin: vec2(5., 5.),
            ..Default::default()
        };

        egui::TopPanel::top("top panel")
            .frame(_dark_frame_top.clone())
            .show(ctx, |ui| {
                self.top_ui(ui);
            });

        egui::SidePanel::left("colors", 150.)
            .frame(_dark_frame_side)
            .show(ctx, |ui| {
                ScrollArea::auto_sized().show(ui, |ui| {
                    self.side_ui(ui, &mut Some(frame.tex_allocator()));
                })
            });

        egui::CentralPanel::default()
            .frame(_frame)
            .show(ctx, |ui| match self.tab {
                EpickApp::ColorPicker => {
                    self.picker
                        .ui(ui, &mut Some(frame.tex_allocator()), &mut self.saved_colors);
                }
                EpickApp::GradientView => {}
            });

        frame.set_window_size(ctx.used_size());
    }
}

enum EpickButton {
    Active,
    Inactive,
}

impl EpickButton {
    fn text_color() -> Color32 {
        Color32::from_rgb(229, 222, 214)
    }

    fn bg_color(&self) -> Color32 {
        match self {
            EpickButton::Active => Color32::from_rgb(49, 63, 78),
            EpickButton::Inactive => Color32::from_rgb(35, 45, 56),
        }
    }

    fn btn<T: AsRef<str>>(&self, text: T) -> Button {
        Button::new(text.as_ref())
            .text_color(Self::text_color())
            .fill(Some(self.bg_color()))
    }
}

impl Epick {
    pub fn top_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.dark_light_switch(ui);
            ui.label("switch ui color");
            ui.add_space(50.);
            let picker_tab;
            const PICKER_TITLE: &str = "picker";
            match self.tab {
                EpickApp::ColorPicker => {
                    picker_tab = EpickButton::Active.btn(PICKER_TITLE);
                }
                EpickApp::GradientView => {
                    picker_tab = EpickButton::Inactive.btn(PICKER_TITLE);
                }
            }
            let picker_resp = ui.add(picker_tab);
            if picker_resp.clicked() {
                self.tab = EpickApp::ColorPicker;
            }

            let gradient_tab;
            const GRADIENT_TITLE: &str = "gradient";
            match self.tab {
                EpickApp::GradientView => {
                    gradient_tab = EpickButton::Active.btn(GRADIENT_TITLE);
                }
                EpickApp::ColorPicker => {
                    gradient_tab = EpickButton::Inactive.btn(GRADIENT_TITLE);
                }
            }
            let gradient_resp = ui.add(gradient_tab);
            if gradient_resp.clicked() {
                self.tab = EpickApp::GradientView;
            }
        });
    }

    pub fn dark_light_switch(&mut self, ui: &mut Ui) {
        let style = (*ui.ctx().style()).clone();
        let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
        if let Some(visuals) = new_visuals {
            ui.ctx().set_visuals(visuals);
        }
    }

    pub fn side_ui(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Saved colors");
                ui.add_space(7.);
                let clear_btn = ui.add(EpickButton::Inactive.btn("clear"));
                if clear_btn.clicked() {
                    self.saved_colors.clear();
                }
            });
            ui.add_space(7.);
            ui.label("Left click: set current");
            ui.add_space(3.5);
            ui.label("Right click: copy hex");
            ui.add_space(7.);

            for (idx, (hex, color)) in self.saved_colors.clone().iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(ui.fonts().row_height(TextStyle::Monospace));
                        ui.monospace(format!("#{}", hex));
                        ui.horizontal(|ui| {
                            if ui.button("❌").clicked() {
                                self.saved_colors
                                    .iter()
                                    .position(|(_hex, _)| _hex == hex)
                                    .map(|i| self.saved_colors.remove(i));
                            }
                            ui.vertical(|ui| {
                                if ui.button("⏶").clicked() {
                                    if idx > 0 {
                                        self.saved_colors.swap(idx, idx - 1);
                                    }
                                }

                                if ui.button("⏷").clicked() {
                                    if idx < (self.saved_colors.len() - 1) {
                                        self.saved_colors.swap(idx, idx + 1);
                                    }
                                }
                            });
                        });
                    });

                    let resp = tex_color(
                        ui,
                        tex_allocator,
                        &mut self.picker.tex_mngr,
                        color.clone(),
                        vec2(100., 50.),
                        Some(&hex),
                    );

                    if let Some(resp) = resp {
                        match self.tab {
                            EpickApp::ColorPicker => {
                                let hex = color_as_hex(&color);
                                if resp.clicked() {
                                    self.picker.set_cur_color(color.clone());
                                }

                                if resp.secondary_clicked() {
                                    let _ = save_to_clipboard(format!("#{}", hex));
                                }
                            }
                            EpickApp::GradientView => {}
                        };
                    }
                });
            }
        });
    }
}

impl ColorPicker {
    fn set_cur_color(&mut self, color: Color32) {
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
        saved_colors: &mut Vec<(String, Color32)>,
    ) {
        ui.horizontal(|ui| {
            ui.label("Enter a hex color: ");
            let resp = ui.text_edit_singleline(&mut self.hex_color);
            if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                || ui.button("▶ go").clicked()
            {
                if let Some(color) = parse_color(self.hex_color.trim_start_matches("#")) {
                    self.set_cur_color(color);
                }
            }
            if ui.button("➕ save").clicked() {
                if let Some(color) = self.cur_color {
                    let color = (color_as_hex(&color), color);
                    if !saved_colors.contains(&color) {
                        saved_colors.push(color);
                    }
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
                    Some(&color_as_hex(&color)),
                );
                if let Some(resp) = resp {
                    let hex = color_as_hex(&color);
                    if resp.clicked() {
                        self.set_cur_color(color);
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
fn tex_color(
    ui: &mut Ui,
    tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    tex_mngr: &mut TextureManager,
    color: Color32,
    size: Vec2,
    on_hover: Option<&str>,
) -> Option<Response> {
    let gradient = Gradient::one_color(color);
    let resp = tex_gradient(ui, tex_allocator, tex_mngr, &gradient, size, on_hover);

    resp
}

//self.set_cur_color(parse_color(&color).unwrap());
fn tex_gradient(
    ui: &mut Ui,
    tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    tex_mngr: &mut TextureManager,
    gradient: &Gradient,
    size: Vec2,
    on_hover: Option<&str>,
) -> Option<Response> {
    if let Some(tex_allocator) = tex_allocator {
        let resp = ui.horizontal(|ui| {
            let tex = tex_mngr.get(*tex_allocator, &gradient);
            let texel_offset = 0.5 / (gradient.0.len() as f32);
            let uv = Rect::from_min_max(pos2(texel_offset, 0.0), pos2(1.0 - texel_offset, 1.0));
            let image = ImageButton::new(tex, size).uv(uv);
            let mut resp = ui.add(image);

            if let Some(on_hover) = on_hover {
                resp = resp.on_hover_text(on_hover);
            }

            resp
        });
        return Some(resp.inner);
    }
    None
}

#[derive(Default)]
pub struct TextureManager(HashMap<Gradient, TextureId>);

impl TextureManager {
    fn get(
        &mut self,
        tex_allocator: &mut dyn epi::TextureAllocator,
        gradient: &Gradient,
    ) -> TextureId {
        *self.0.entry(gradient.clone()).or_insert_with(|| {
            let pixels = gradient.to_pixel_row();
            let width = pixels.len();
            let height = 1;
            tex_allocator.alloc_srgba_premultiplied((width, height), &pixels)
        })
    }
}
