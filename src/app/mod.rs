mod render;
mod scheme;
mod ui;

use render::{color_slider_1d, tex_color, TextureManager};
use ui::{color_tooltip, colors::*, dark_visuals, drag_source, drop_target, light_visuals};

use crate::color::{Cmyk, Color};
use crate::save_to_clipboard;
use egui::{color::Color32, vec2, Slider, Ui};
use egui::{
    color::{Hsva, HsvaGamma},
    DragValue, Id, Rgba, ScrollArea, Vec2, Visuals,
};
use std::borrow::Cow;

static MIN_COL_SIZE: f32 = 50.;
static ADD_ICON: &str = "➕";
static ADD_DESCR: &str = "Add this color to saved colors";

//####################################################################################################

#[derive(Default, Debug)]
pub struct SavedColors(Vec<(String, Color)>);

impl SavedColors {
    pub fn add(&mut self, color: Color) -> bool {
        let hex = color.as_hex();
        if self.0.iter().find(|(_hex, _)| _hex == &hex).is_none() {
            self.0.push((hex, color));
            return true;
        }
        false
    }

    pub fn insert(&mut self, i: usize, color: Color) {
        let color = (color.as_hex(), color);
        if !self.0.contains(&color) {
            self.0.insert(i, color);
        }
    }

    pub fn remove(&mut self, color: &Color) -> Option<(String, Color)> {
        self.0
            .iter()
            .position(|(_, col)| col == color)
            .map(|i| self.0.remove(i))
    }

    pub fn remove_pos(&mut self, i: usize) -> Option<(String, Color)> {
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

impl AsRef<[(String, Color)]> for SavedColors {
    fn as_ref(&self) -> &[(String, Color)] {
        self.0.as_ref()
    }
}

//####################################################################################################

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SideTab {
    Hues,
    Shades,
    Tints,
    NoTab,
}

//####################################################################################################

#[derive(Debug, PartialEq)]
pub enum SchemeType {
    Complementary,
    Triadic,
    Tetradic,
    Analogous,
    SplitComplementary,
}

impl AsRef<str> for SchemeType {
    fn as_ref(&self) -> &str {
        match &self {
            SchemeType::Complementary => "complementary",
            SchemeType::Triadic => "triadic",
            SchemeType::Tetradic => "tetradic",
            SchemeType::Analogous => "analogous",
            SchemeType::SplitComplementary => "split complementary",
        }
    }
}

//####################################################################################################

#[derive(Debug)]
pub struct ColorPicker {
    // picker fields
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

    pub scheme_ty: SchemeType,

    // side panel
    pub numof_shades: u8,
    pub numof_tints: u8,
    pub numof_hues: u8,
    pub shade_color_size: f32,
    pub tint_color_size: f32,
    pub hue_color_size: f32,
    pub scheme_color_size: f32,
    pub hues_step: f32,
    pub side_panel_visible: Option<SideTab>,

    pub tex_mngr: TextureManager,
    pub main_width: f32,
    pub err: Option<String>,
    pub saved_panel_visible: bool,
    pub saved_colors: SavedColors,
    pub light_theme: Visuals,
    pub dark_theme: Visuals,
}

impl epi::App for ColorPicker {
    fn name(&self) -> &str {
        "epick"
    }

    fn max_size_points(&self) -> egui::Vec2 {
        vec2(4096., 8192.)
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
        if self.saved_panel_visible {
            self.side_panel(ctx, tex_allocator);
        }
        self.central_panel(ctx, tex_allocator);

        frame.set_window_size(ctx.used_size());
    }
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
            numof_shades: 6,
            numof_tints: 6,
            numof_hues: 4,
            shade_color_size: 100.,
            tint_color_size: 100.,
            hue_color_size: 100.,
            hues_step: 0.05,
            scheme_color_size: 200.,
            scheme_ty: SchemeType::Complementary,
            tex_mngr: TextureManager::default(),
            main_width: 0.,
            err: None,
            side_panel_visible: None,
            saved_panel_visible: false,
            saved_colors: SavedColors::default(),
            light_theme: light_visuals(),
            dark_theme: dark_visuals(),
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
        if hsva.s != 0. {
            self.hue = hsva.h;
        }
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
        ctx: &egui::CtxRef,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(err) = &self.err {
            ui.colored_label(Color32::RED, err);
        }
        ui.label("Enter a hex color: ");
        let enter_bar = ui.horizontal(|ui| {
            let resp = ui.text_edit_singleline(&mut self.hex_color);
            if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                || ui.button("▶").on_hover_text("Use this color").clicked()
            {
                if self.hex_color.len() < 6 {
                    self.err = Some("Enter a color first (ex. ab12ff #1200ff)".to_owned());
                } else if let Some(color) = Color::from_hex(self.hex_color.trim_start_matches('#'))
                {
                    self.set_cur_color(color);
                    self.err = None;
                } else {
                    self.err = Some("The entered hex color is not valid".to_owned());
                }
            }
            if ui.button(ADD_ICON).on_hover_text(ADD_DESCR).clicked() {
                if !self.saved_colors.add(self.cur_color) {
                    self.err = Some(format!("Color #{} already saved!", self.cur_color.as_hex()));
                } else {
                    self.err = None;
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
                if let Err(e) = save_to_clipboard(format!("#{}", hex)) {
                    self.err = Some(format!("Failed to save color to clipboard - {}", e));
                } else {
                    self.err = None;
                }
            }
            if ui.button(ADD_ICON).on_hover_text(ADD_DESCR).clicked() {
                if !self.saved_colors.add(self.cur_color) {
                    self.err = Some(format!("Color {} already saved!", self.cur_color.as_hex()));
                } else {
                    self.err = None;
                }
            }
        });

        self.check_color_change();

        ScrollArea::auto_sized()
            .id_source("picker scroll")
            .show(ui, |mut ui| {
                self.sliders(ui);
                self.schemes(&mut ui, tex_allocator);
                self.shades(ctx, tex_allocator);
                self.tints(ctx, tex_allocator);
                self.hues(ctx, tex_allocator);
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

    fn color_box_label_under(
        &mut self,
        color: &Color,
        size: Vec2,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        ui.vertical(|ui| {
            self._color_box(color, size, ui, tex_allocator, true);
        });
    }

    fn color_box_label_side(
        &mut self,
        color: &Color,
        size: Vec2,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        ui.horizontal(|ui| {
            self._color_box(color, size, ui, tex_allocator, true);
        });
    }

    #[allow(dead_code)]
    fn color_box_no_label(
        &mut self,
        color: &Color,
        size: Vec2,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        self._color_box(color, size, ui, tex_allocator, false);
    }

    fn _color_box(
        &mut self,
        color: &Color,
        size: Vec2,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        with_label: bool,
    ) {
        let hex = color.as_hex();
        let color_box = tex_color(
            ui,
            tex_allocator,
            &mut self.tex_mngr,
            color.as_32(),
            size,
            Some(&color_tooltip(&color)),
        );
        if let Some(color_box) = color_box {
            if with_label {
                ui.monospace(format!("#{}", hex));
            }

            if color_box.clicked() {
                self.set_cur_color(*color);
            }

            if color_box.middle_clicked() {
                self.saved_colors.add(*color);
            }

            if color_box.secondary_clicked() {
                let _ = save_to_clipboard(hex);
            }
        }
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
        egui::CentralPanel::default().frame(_frame).show(ctx, |ui| {
            self.ui(ctx, ui, tex_allocator);
        });
    }

    pub fn top_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.dark_light_switch(ui);
            if ui
                .button("↔")
                .on_hover_text("Show/hide side panel")
                .clicked()
            {
                self.saved_panel_visible = !self.saved_panel_visible;
            }
            ui.add_space(50.);

            ui.selectable_value(&mut self.side_panel_visible, Some(SideTab::Hues), "hues");
            ui.selectable_value(&mut self.side_panel_visible, Some(SideTab::Tints), "tints");
            ui.selectable_value(
                &mut self.side_panel_visible,
                Some(SideTab::Shades),
                "shades",
            );
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
                if ui.button("🗑").on_hover_text("Clear colors").clicked() {
                    self.saved_colors.clear();
                }
            });

            let mut src_row = None;
            let mut dst_row = None;

            for (idx, (hex, color)) in self.saved_colors.as_ref().to_vec().iter().enumerate() {
                let resp = drop_target(ui, true, |ui| {
                    let color_id = Id::new("side-color").with(idx);
                    ui.vertical(|mut ui| {
                        let fst = ui.horizontal(|ui| {
                            ui.monospace(format!("#{}", hex));
                            if ui.button("❌").on_hover_text("Delete this color").clicked() {
                                self.saved_colors.remove(color);
                            }
                            if ui.button("📋").on_hover_text("Copy hex color").clicked() {
                                let _ = save_to_clipboard(hex.clone());
                            }
                            if ui.button("▶").on_hover_text("Use this color").clicked() {
                                self.set_cur_color(*color);
                            }
                        });
                        let help =
                            format!("#{}\n\nDrag and drop to change the order of colors", hex);

                        let w = fst.response.rect.width();
                        let size = vec2(w, w / 2.);
                        drag_source(&mut ui, color_id, |ui| {
                            tex_color(
                                ui,
                                tex_allocator,
                                &mut self.tex_mngr,
                                color.as_32(),
                                size,
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
