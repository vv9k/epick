mod picker;
mod render;
mod scheme;
mod ui;

use picker::ColorPicker;
use render::tex_color;
use scheme::SchemeGenerator;
use ui::colors::*;
use ui::{color_tooltip, dark_visuals, drag_source, drop_target, light_visuals};

use crate::color::color_as_hex;
use crate::save_to_clipboard;

use egui::color::Color32;
use egui::{vec2, Id, ScrollArea, Ui, Visuals};
use std::borrow::Cow;

//####################################################################################################

#[derive(Default, Debug)]
pub struct SavedColors(Vec<(String, Color32)>);

impl SavedColors {
    pub fn add(&mut self, color: Color32) {
        let color = (color_as_hex(&color), color);
        if !self.0.contains(&color) {
            self.0.push(color);
        }
    }

    pub fn insert(&mut self, i: usize, color: Color32) {
        let color = (color_as_hex(&color), color);
        if !self.0.contains(&color) {
            self.0.insert(i, color);
        }
    }

    pub fn remove(&mut self, color: &Color32) -> Option<(String, Color32)> {
        self.0
            .iter()
            .position(|(_, col)| col == color)
            .map(|i| self.0.remove(i))
    }

    pub fn remove_pos(&mut self, i: usize) -> Option<(String, Color32)> {
        if i < self.0.len() {
            Some(self.0.remove(i))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }
}

impl AsRef<[(String, Color32)]> for SavedColors {
    fn as_ref(&self) -> &[(String, Color32)] {
        self.0.as_ref()
    }
}

//####################################################################################################

#[derive(Debug, PartialEq)]
pub enum EpickApp {
    ColorPicker,
    GradientView,
    SchemeGenerator,
}

impl Default for EpickApp {
    fn default() -> Self {
        Self::ColorPicker
    }
}

//####################################################################################################

pub struct Epick {
    pub current_tab: EpickApp,
    pub picker: ColorPicker,
    pub generator: SchemeGenerator,
    pub saved_colors: SavedColors,
    pub light_theme: Visuals,
    pub dark_theme: Visuals,
}

impl Default for Epick {
    fn default() -> Self {
        Self {
            current_tab: EpickApp::default(),
            picker: ColorPicker::default(),
            generator: SchemeGenerator::default(),
            saved_colors: SavedColors::default(),
            light_theme: light_visuals(),
            dark_theme: dark_visuals(),
        }
    }
}

impl epi::App for Epick {
    fn name(&self) -> &str {
        "epick"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Firacode".to_string(),
            Cow::Borrowed(include_bytes!("../../assets/FiraCode-Regular.ttf")),
        );
        let mut def = fonts
            .fonts_for_family
            .get_mut(&egui::FontFamily::Monospace)
            .map(|v| v.clone())
            .unwrap_or_default();
        def.push("Firacode".to_string());
        fonts
            .fonts_for_family
            .insert(egui::FontFamily::Monospace, def);
        fonts.family_and_size.insert(
            egui::TextStyle::Monospace,
            (egui::FontFamily::Monospace, 16.),
        );
        _ctx.set_fonts(fonts);
        _ctx.set_visuals(dark_visuals());
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let tex_allocator = &mut Some(frame.tex_allocator());

        self.top_panel(ctx);
        self.side_panel(ctx, tex_allocator);
        self.central_panel(ctx, tex_allocator);

        frame.set_window_size(ctx.used_size());
    }
}

impl Epick {
    pub fn top_panel(&mut self, ctx: &egui::CtxRef) {
        let frame = egui::Frame {
            fill: if ctx.style().visuals.dark_mode {
                *D_BG_00
            } else {
                *L_BG_0
            },
            margin: vec2(5., 5.),
            ..Default::default()
        };
        egui::TopPanel::top("top panel")
            .frame(frame)
            .show(ctx, |ui| {
                self.top_ui(ui);
            });
    }

    pub fn side_panel(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        let frame = egui::Frame {
            fill: if ctx.style().visuals.dark_mode {
                *D_BG_00
            } else {
                *L_BG_0
            },
            margin: vec2(15., 10.),
            ..Default::default()
        };

        egui::SidePanel::left("colors", 200.)
            .frame(frame)
            .show(ctx, |ui| {
                ScrollArea::auto_sized().show(ui, |ui| {
                    self.side_ui(ui, tex_allocator);
                })
            });
    }

    pub fn central_panel(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        let _frame = egui::Frame {
            fill: if ctx.style().visuals.dark_mode {
                *D_BG_0
            } else {
                *L_BG_2
            },
            margin: vec2(20., 20.),
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(_frame)
            .show(ctx, |ui| match self.current_tab {
                EpickApp::ColorPicker => {
                    self.picker.ui(ui, tex_allocator, &mut self.saved_colors);
                }
                EpickApp::GradientView => {}
                EpickApp::SchemeGenerator => {
                    self.generator.ui(ui, tex_allocator, &mut self.saved_colors);
                }
            });
    }

    pub fn top_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.dark_light_switch(ui);
            ui.add_space(50.);

            ui.selectable_value(&mut self.current_tab, EpickApp::ColorPicker, "picker");
            ui.selectable_value(&mut self.current_tab, EpickApp::SchemeGenerator, "scheme");
        });
    }

    pub fn dark_light_switch(&mut self, ui: &mut Ui) {
        let is_dark = ui.style().visuals.dark_mode;
        let btn = if is_dark { "☀" } else { "🌙" };

        if ui
            .button(btn)
            .on_hover_text("Switch ui color theme")
            .clicked()
        {
            if is_dark {
                ui.ctx().set_visuals(self.light_theme.clone());
            } else {
                ui.ctx().set_visuals(self.dark_theme.clone());
            }
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
                if ui.button("clear").clicked() {
                    self.saved_colors.clear();
                }
            });

            let mut src_row = None;
            let mut dst_row = None;

            for (idx, (hex, color)) in self.saved_colors.as_ref().to_vec().iter().enumerate() {
                let resp = drop_target(ui, true, |ui| {
                    let color_id = Id::new("side-color").with(idx);
                    ui.columns(2, |cols| {
                        cols[0].vertical(|ui| {
                            ui.monospace(format!("#{}", hex));
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    if ui.button("❌").on_hover_text("Delete this color").clicked()
                                    {
                                        self.saved_colors.remove(color);
                                    }
                                    if ui.button("▶").on_hover_text("Use this color").clicked() {
                                        self.picker.set_cur_color(color.clone());
                                        self.generator.set_cur_color(color.clone());
                                    }
                                });
                                ui.vertical(|ui| {
                                    if ui.button("📋").on_hover_text("Copy hex color").clicked() {
                                        let _ = save_to_clipboard(hex.clone());
                                    }
                                });
                            });
                        });
                        let help =
                            format!("#{}\n\nDrag and drop to change the order of colors", hex);

                        drag_source(&mut cols[1], color_id, |ui| {
                            tex_color(
                                ui,
                                tex_allocator,
                                &mut self.picker.tex_mngr,
                                color.clone(),
                                vec2(80., 50.),
                                Some(&help),
                            );
                        });
                    });
                    if ui.memory().is_being_dragged(color_id) {
                        src_row = Some(idx);
                    }
                })
                .response;
                let is_being_dragged = ui.memory().is_anything_being_dragged();
                if is_being_dragged && resp.hovered() {
                    dst_row = Some(idx);
                }
            }

            if let Some(src_row) = src_row {
                if let Some(dst_row) = dst_row {
                    if ui.input().pointer.any_released() {
                        if let Some(it) = self.saved_colors.remove_pos(src_row) {
                            self.saved_colors.insert(dst_row, it.1);
                        }
                    }
                }
            }
        });
    }
}
