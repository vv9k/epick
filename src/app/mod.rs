mod picker;
mod render;
mod scheme;
mod ui;

use picker::ColorPicker;
use render::tex_color;
use scheme::SchemeGenerator;
use ui::Tab;

use crate::color::color_as_hex;
use crate::save_to_clipboard;

use egui::color::Color32;
use egui::{vec2, ScrollArea, TextStyle, Ui};

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

#[derive(Default)]
pub struct Epick {
    pub tab: EpickApp,
    pub picker: ColorPicker,
    pub generator: SchemeGenerator,
    pub saved_colors: SavedColors,
}

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

impl epi::App for Epick {
    fn name(&self) -> &str {
        "epick"
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
            fill: Color32::from_rgb(17, 22, 27),
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
            fill: Color32::from_rgb(17, 22, 27),
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
            fill: Color32::from_rgb(22, 28, 35),
            margin: vec2(20., 20.),
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(_frame)
            .show(ctx, |ui| match self.tab {
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
            let picker_tab;
            const PICKER_TITLE: &str = "picker";
            match self.tab {
                EpickApp::ColorPicker => {
                    picker_tab = Tab::Active.btn(PICKER_TITLE);
                }
                EpickApp::GradientView => {
                    picker_tab = Tab::Inactive.btn(PICKER_TITLE);
                }
                EpickApp::SchemeGenerator => {
                    picker_tab = Tab::Inactive.btn(PICKER_TITLE);
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
                    gradient_tab = Tab::Active.btn(GRADIENT_TITLE);
                }
                EpickApp::ColorPicker => {
                    gradient_tab = Tab::Inactive.btn(GRADIENT_TITLE);
                }
                EpickApp::SchemeGenerator => {
                    gradient_tab = Tab::Inactive.btn(GRADIENT_TITLE);
                }
            }
            let gradient_resp = ui.add(gradient_tab);
            if gradient_resp.clicked() {
                self.tab = EpickApp::GradientView;
            }

            let scheme_tab;
            const SCHEME_TITLE: &str = "scheme";
            match self.tab {
                EpickApp::GradientView => {
                    scheme_tab = Tab::Active.btn(SCHEME_TITLE);
                }
                EpickApp::ColorPicker => {
                    scheme_tab = Tab::Inactive.btn(SCHEME_TITLE);
                }
                EpickApp::SchemeGenerator => {
                    scheme_tab = Tab::Inactive.btn(SCHEME_TITLE);
                }
            }
            let scheme_resp = ui.add(scheme_tab);
            if scheme_resp.clicked() {
                self.tab = EpickApp::SchemeGenerator;
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
                        "#{}\n\nLeft click: set current\nsecondary click: copy hex",
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
