mod picker;
mod render;
mod scheme;

use picker::ColorPicker;
use render::tex_color;
use scheme::SchemeGenerator;

use crate::color::color_as_hex;
use crate::save_to_clipboard;

use egui::{color::Color32, Visuals};
use egui::{vec2, ScrollArea, Stroke, TextStyle, Ui};
use lazy_static::lazy_static;
use std::borrow::Cow;

lazy_static! {
    static ref D_BG_00: Color32 = Color32::from_rgb(0x11, 0x16, 0x1b);
    static ref D_BG_0: Color32 = Color32::from_rgb(0x16, 0x1c, 0x23);
    static ref D_BG_1: Color32 = Color32::from_rgb(0x23, 0x2d, 0x38);
    static ref D_BG_2: Color32 = Color32::from_rgb(0x31, 0x3f, 0x4e);
    static ref D_BG_3: Color32 = Color32::from_rgb(0x41, 0x53, 0x67);
    static ref D_FG_0: Color32 = Color32::from_rgb(0xe5, 0xde, 0xd6);
    static ref L_BG_0: Color32 = Color32::from_rgb(0xa7, 0xa6, 0xa7);
    static ref L_BG_1: Color32 = Color32::from_rgb(0xb6, 0xb5, 0xb6);
    static ref L_BG_2: Color32 = Color32::from_rgb(0xc5, 0xc4, 0xc5);
    static ref L_BG_3: Color32 = Color32::from_rgb(0xd4, 0xd3, 0xd4);
    static ref L_BG_4: Color32 = Color32::from_rgb(0xf2, 0xf1, 0xf2);
    static ref L_BG_5: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
    static ref L_FG_0: Color32 = *D_BG_0;
}

#[derive(Default, Debug)]
pub struct SavedColors(Vec<(String, Color32)>);

impl SavedColors {
    pub fn add(&mut self, color: Color32) {
        let color = (color_as_hex(&color), color);
        if !self.0.contains(&color) {
            self.0.push(color);
        }
    }

    pub fn remove(&mut self, color: &Color32) -> Option<(String, Color32)> {
        self.0
            .iter()
            .position(|(_, col)| col == color)
            .map(|i| self.0.remove(i))
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
            light_theme: Self::light_visuals(),
            dark_theme: Self::dark_visuals(),
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
        _ctx.set_visuals(Self::dark_visuals());
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
    fn light_visuals() -> Visuals {
        let mut vis = Visuals::default();
        vis.dark_mode = false;
        vis.override_text_color = Some(*L_FG_0);
        vis.extreme_bg_color = Color32::WHITE;
        vis.widgets.noninteractive.fg_stroke = Stroke::new(0., *L_FG_0);
        vis.widgets.noninteractive.bg_fill = *L_BG_5;
        vis.widgets.inactive.bg_fill = *L_BG_4;
        vis.widgets.inactive.bg_stroke = Stroke::new(0.7, *D_BG_3);
        vis.widgets.inactive.fg_stroke = Stroke::new(0.7, *D_BG_3);
        vis.widgets.hovered.bg_fill = *L_BG_5;
        vis.widgets.hovered.bg_stroke = Stroke::new(1., *D_BG_1);
        vis.widgets.hovered.fg_stroke = Stroke::new(1., *D_BG_1);
        vis.widgets.active.bg_fill = *L_BG_5;
        vis.widgets.active.fg_stroke = Stroke::new(0., *D_BG_0);
        vis.selection.bg_fill = *L_BG_5;
        vis.selection.stroke = Stroke::new(0.7, *D_BG_0);
        vis
    }

    fn dark_visuals() -> Visuals {
        let mut vis = Visuals::default();
        vis.dark_mode = true;
        vis.override_text_color = Some(*D_FG_0);
        vis.widgets.inactive.bg_fill = *D_BG_1;
        vis.widgets.hovered.bg_fill = *D_BG_2;
        vis.widgets.active.bg_fill = *D_BG_3;
        vis.selection.bg_fill = *D_BG_3;
        vis.selection.stroke = Stroke::new(0.7, *D_FG_0);
        vis
    }

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

        egui::SidePanel::left("colors", 150.)
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
            ui.label("switch ui color");
            ui.add_space(50.);

            ui.selectable_value(&mut self.current_tab, EpickApp::ColorPicker, "picker");
            ui.selectable_value(&mut self.current_tab, EpickApp::SchemeGenerator, "scheme");
            ui.selectable_value(&mut self.current_tab, EpickApp::GradientView, "gradient");
        });
    }

    pub fn dark_light_switch(&mut self, ui: &mut Ui) {
        let is_dark = ui.style().visuals.dark_mode;
        let btn = if is_dark { "☀" } else { "🌙" };

        if ui.button(btn).clicked() {
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

            for (idx, (hex, color)) in self.saved_colors.as_ref().to_vec().iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(ui.fonts().row_height(TextStyle::Monospace));
                        ui.monospace(format!("#{}", hex));
                        ui.horizontal(|ui| {
                            if ui.button("❌").clicked() {
                                self.saved_colors.remove(color);
                            }
                            ui.vertical(|ui| {
                                if ui.button("⏶").clicked() {
                                    if idx > 0 {
                                        self.saved_colors.swap(idx, idx - 1);
                                    }
                                }

                                if ui.button("⏷").clicked() {
                                    if idx < (self.saved_colors.as_ref().len() - 1) {
                                        self.saved_colors.swap(idx, idx + 1);
                                    }
                                }
                            });
                        });
                    });
                    let help = format!(
                        "#{}\n\nPrimary click: set current\nSecondary click: copy hex",
                        hex
                    );

                    let resp = tex_color(
                        ui,
                        tex_allocator,
                        &mut self.picker.tex_mngr,
                        color.clone(),
                        vec2(100., 50.),
                        Some(&help),
                    );

                    if let Some(resp) = resp {
                        match self.current_tab {
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
                            EpickApp::SchemeGenerator => {
                                if resp.clicked() {
                                    self.generator.set_cur_color(color.clone());
                                }
                            }
                        };
                    }
                });
            }
        });
    }
}
