use crate::color::{color_as_hex, parse_color, Gradient};
use eframe::{egui, epi};
use egui::color::*;
use egui::{pos2, vec2, Image, Rect, ScrollArea, Slider, TextureId, Ui, Vec2};
use std::collections::HashMap;

pub struct ColorPicker {
    pub hex_color: String,
    pub cur_color: Option<Color32>,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub tex_mngr: TextureManager,
    pub saved_colors: HashMap<String, Color32>,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            hex_color: String::new(),
            cur_color: None,
            r: 0,
            g: 0,
            b: 0,
            tex_mngr: Default::default(),
            saved_colors: HashMap::new(),
        }
    }
}

impl epi::App for ColorPicker {
    fn name(&self) -> &str {
        "Picked"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::SidePanel::left("colors", 400.).show(ctx, |ui| {
            self.side_ui(ui, &mut Some(frame.tex_allocator()));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.ui(ui, &mut Some(frame.tex_allocator()));
            })
        });

        frame.set_window_size(ctx.used_size());
    }
}

impl ColorPicker {
    pub fn side_ui(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        ui.horizontal(|ui| {
            if ui.button("clear colors").clicked() {
                self.saved_colors.clear();
            }
        });

        for (hex, color) in self.saved_colors.clone() {
            ui.horizontal(|ui| {
                ui.monospace(format!("#{}", hex));
                self.tex_color(
                    ui,
                    tex_allocator,
                    color.clone(),
                    vec2(100., 50.),
                    Some(&hex),
                );
            });
        }
    }

    fn set_cur_color(&mut self, color: Color32) {
        self.r = color.r();
        self.g = color.g();
        self.b = color.b();
        self.cur_color = Some(color);
    }

    pub fn ui(&mut self, ui: &mut Ui, tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>) {
        ui.horizontal(|ui| {
            ui.label("Enter a hex color: ");
            let resp = ui.text_edit_singleline(&mut self.hex_color);
            if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                || ui.button("enter").clicked()
            {
                if let Some(color) = parse_color(&self.hex_color) {
                    self.set_cur_color(color);
                }
            }
            if ui.button("save").clicked() {
                if let Some(color) = self.cur_color {
                    self.saved_colors
                        .insert(color_as_hex(&color).to_uppercase(), color);
                }
            }
        });

        if let Some(color) = self.cur_color {
            ui.add(Slider::new(&mut self.r, u8::MIN..=u8::MAX).text("red"));
            ui.add(Slider::new(&mut self.g, u8::MIN..=u8::MAX).text("green"));
            ui.add(Slider::new(&mut self.b, u8::MIN..=u8::MAX).text("blue"));

            if self.r != color.r() || self.g != color.g() || self.b != color.b() {
                self.cur_color = Some(Color32::from_rgb(self.r, self.g, self.b));
            }

            ui.scope(|ui| {
                self.tex_color(
                    ui,
                    tex_allocator,
                    self.cur_color.unwrap(),
                    vec2(300., 300.),
                    None,
                );
            });
        }
    }

    fn tex_color(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        color: Color32,
        size: Vec2,
        on_hover: Option<&str>,
    ) {
        let gradient = Gradient::one_color(color);
        self.tex_gradient(ui, tex_allocator, &gradient, size, on_hover);
    }

    fn tex_gradient(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        gradient: &Gradient,
        size: Vec2,
        on_hover: Option<&str>,
    ) {
        if let Some(tex_allocator) = tex_allocator {
            ui.horizontal(|ui| {
                let tex = self.tex_mngr.get(*tex_allocator, &gradient);
                let texel_offset = 0.5 / (gradient.0.len() as f32);
                let uv = Rect::from_min_max(pos2(texel_offset, 0.0), pos2(1.0 - texel_offset, 1.0));
                let image = Image::new(tex, size).uv(uv);
                let resp = ui.add(image);
                // not working?
                if resp.middle_clicked() {
                    if let Some(color) = gradient.as_hex() {
                        self.hex_color = color;
                    }
                }
                if let Some(on_hover) = on_hover {
                    resp.on_hover_text(on_hover);
                }
            });
        }
    }
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
