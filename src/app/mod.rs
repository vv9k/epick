mod render;
mod scheme;
mod ui;

use render::{color_slider_1d, tex_color, TextureManager};
use ui::{color_tooltip, colors::*, dark_visuals, drag_source, drop_target, light_visuals};

use crate::color::{Cmyk, Color, Hsl, Lch, Luv, Xyz};
use crate::save_to_clipboard;
use egui::{color::Color32, vec2, Ui};
use egui::{
    color::{Hsva, HsvaGamma},
    ComboBox, DragValue, Id, Rgba, ScrollArea, Vec2, Visuals, Window,
};
use std::borrow::Cow;
use std::path::PathBuf;
use std::{env, fs};

static ADD_ICON: &str = "➕";
static ADD_DESCR: &str = "Add this color to saved colors";

//####################################################################################################

#[derive(Default, Debug)]
pub struct SavedColors(Vec<(String, Color)>);

impl SavedColors {
    pub fn add(&mut self, color: Color) -> bool {
        let hex = color.as_hex();
        if !self.0.iter().any(|(_hex, _)| _hex == &hex) {
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

    pub fn as_gimp_palette(&self, name: &str) -> String {
        let mut gpl = format!("GIMP Palette\nName: {}.gpl\nColumns: 1\n#\n", name);
        for (i, (_, color)) in self.0.iter().enumerate() {
            let color = color.as_32();
            gpl.push_str(&format!(
                "{}\t{}\t{}\tcolor {}\n",
                color.r(),
                color.g(),
                color.b(),
                i
            ));
        }
        gpl
    }

    pub fn as_text_palette(&self) -> String {
        self.0.iter().fold(String::new(), |mut s, (hex, _)| {
            s.push('#');
            s.push_str(hex.as_str());
            s.push('\n');
            s
        })
    }
}

impl AsRef<[(String, Color)]> for SavedColors {
    fn as_ref(&self) -> &[(String, Color)] {
        self.0.as_ref()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PaletteFormat {
    Gimp,
    Text,
}

impl PaletteFormat {
    pub fn extension(&self) -> &str {
        match self {
            PaletteFormat::Gimp => "gpl",
            PaletteFormat::Text => "txt",
        }
    }
}

impl AsRef<str> for PaletteFormat {
    fn as_ref(&self) -> &str {
        match self {
            PaletteFormat::Gimp => "GIMP (gpl)",
            PaletteFormat::Text => "Hex list (txt)",
        }
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

#[derive(Debug, Clone)]
pub struct ColorSliders {
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
    pub lch_l: f32,
    pub lch_c: f32,
    pub lch_h: f32,
    pub xyz_x: f32,
    pub xyz_y: f32,
    pub xyz_z: f32,
    pub hsl_h: f32,
    pub hsl_s: f32,
    pub hsl_l: f32,
}

impl Default for ColorSliders {
    fn default() -> Self {
        Self {
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
            lch_l: 0.,
            lch_c: 0.,
            lch_h: 0.,
            xyz_x: 0.,
            xyz_y: 0.,
            xyz_z: 0.,
            hsl_h: 0.,
            hsl_s: 0.,
            hsl_l: 0.,
        }
    }
}

impl ColorSliders {
    fn set_color(&mut self, color: Color) {
        let rgba = color.rgba();
        self.r = rgba.r() * u8::MAX as f32;
        self.g = rgba.g() * u8::MAX as f32;
        self.b = rgba.b() * u8::MAX as f32;
        let hsva = color.hsva();
        self.hue = hsva.h;
        self.sat = hsva.s;
        self.val = hsva.v;
        let cmyk = color.cmyk();
        self.k = cmyk.k;
        self.c = cmyk.c;
        self.m = cmyk.m;
        self.y = cmyk.y;
        let lch = color.lch();
        self.lch_l = lch.l;
        self.lch_h = lch.c;
        self.lch_c = lch.h;
        let xyz = color.xyz();
        self.xyz_x = xyz.x;
        self.xyz_y = xyz.y;
        self.xyz_z = xyz.z;
        let hsl = color.hsl();
        self.hsl_h = hsl.h;
        self.hsl_s = hsl.s;
        self.hsl_l = hsl.l;
    }

    fn restore(&mut self, other: Self) {
        self.hue = other.hue;
        self.sat = other.sat;
        self.c = other.c;
        self.m = other.m;
        self.y = other.y;
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self.xyz_x = other.xyz_x;
        self.xyz_y = other.xyz_y;
        self.xyz_z = other.xyz_z;
        self.hsl_h = other.hsl_h;
        self.hsl_s = other.hsl_s;
        self.hsl_l = other.hsl_l;
    }
}

//####################################################################################################

#[derive(Debug)]
pub struct ColorPicker {
    // picker fields
    pub color_size: f32,
    pub hex_color: String,
    pub cur_color: Color,
    pub sliders: ColorSliders,
    pub saved_sliders: Option<ColorSliders>,

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
    pub show_settings: bool,
    pub show_export: bool,
    pub upper_hex: bool,

    pub export_path: String,
    pub export_name: String,
    pub export_status: Result<String, String>,
    pub export_format: PaletteFormat,
}

impl epi::App for ColorPicker {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let tex_allocator = &mut Some(frame.tex_allocator());

        self.top_panel(ctx);
        if self.saved_panel_visible {
            self.side_panel(ctx, tex_allocator);
        }
        self.central_panel(ctx, tex_allocator);

        frame.set_window_size(ctx.used_size());
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Firacode".to_string(),
            Cow::Borrowed(include_bytes!("../../assets/FiraCode-Regular.ttf")),
        );
        fonts
            .fonts_for_family
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "Firacode".to_owned());

        fonts.family_and_size.insert(
            egui::TextStyle::Monospace,
            (egui::FontFamily::Monospace, 16.),
        );
        _ctx.set_fonts(fonts);
        _ctx.set_visuals(dark_visuals());
    }

    fn name(&self) -> &str {
        "epick"
    }

    fn max_size_points(&self) -> egui::Vec2 {
        vec2(4096., 8192.)
    }
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            color_size: 300.,
            hex_color: "".to_string(),
            cur_color: Color::black(),
            sliders: ColorSliders::default(),
            saved_sliders: None,
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
            show_settings: false,
            show_export: false,
            upper_hex: false,

            export_format: PaletteFormat::Gimp,
            export_name: "".to_string(),
            export_status: Ok("".to_string()),
            export_path: env::current_dir()
                .map(|d| d.to_string_lossy().to_string())
                .unwrap_or_default(),
        }
    }
}

impl ColorPicker {
    fn set_cur_color(&mut self, color: impl Into<Color>) {
        let color = color.into();
        self.sliders.set_color(color);
        self.cur_color = color;
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
        let rgb = self.cur_color.rgba();
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
        let hsva = Hsva::from(self.cur_color);
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
        let cmyk = Cmyk::from(self.cur_color);
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

    fn lch_changed(&mut self) -> bool {
        let lch = Lch::from(self.cur_color);
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

    fn xyz_changed(&mut self) -> bool {
        let xyz = Xyz::from(self.cur_color);
        if (self.sliders.xyz_x - xyz.x).abs() > f32::EPSILON
            || (self.sliders.xyz_y - xyz.y).abs() > f32::EPSILON
            || (self.sliders.xyz_z - xyz.z).abs() > f32::EPSILON
        {
            self.set_cur_color(Xyz::new(
                self.sliders.xyz_x,
                self.sliders.xyz_y,
                self.sliders.xyz_z,
            ));
            true
        } else {
            false
        }
    }

    fn hsl_changed(&mut self) -> bool {
        let hsl = Hsl::from(self.cur_color);
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

    fn check_color_change(&mut self) {
        if self.rgb_changed() {
            return;
        }
        if self.hsva_changed() {
            return;
        }
        if self.cmyk_changed() {
            return;
        }
        if self.hsl_changed() {
            return;
        }
        self.xyz_changed();
    }

    fn add_color(&mut self, color: Color) {
        if !self.saved_colors.add(color) {
            let hex = self.color_hex(&color);
            self.err = Some(format!("Color {} already saved!", hex));
        } else {
            self.err = None;
            self.saved_panel_visible = true;
        }
    }

    fn add_cur_color(&mut self) {
        self.add_color(self.cur_color)
    }

    fn hex_input(&mut self, ui: &mut Ui) {
        ui.collapsing("Text input", |ui| {
            ui.label("Enter a hex color: ");
            let enter_bar = ui.horizontal(|ui| {
                let resp = ui.text_edit_singleline(&mut self.hex_color);
                if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                    || ui.button("▶").on_hover_text("Use this color").clicked()
                {
                    if self.hex_color.len() < 6 {
                        self.err = Some("Enter a color first (ex. ab12ff #1200ff)".to_owned());
                    } else if let Some(color) =
                        Color::from_hex(self.hex_color.trim_start_matches('#'))
                    {
                        self.set_cur_color(color);
                        self.err = None;
                    } else {
                        self.err = Some("The entered hex color is not valid".to_owned());
                    }
                }
                if ui.button(ADD_ICON).on_hover_text(ADD_DESCR).clicked() {
                    self.add_cur_color()
                }
            });
            self.main_width = enter_bar.response.rect.width();
        });
    }

    fn color_hex(&self, color: &Color) -> String {
        if self.upper_hex {
            color.as_hex().to_uppercase()
        } else {
            color.as_hex()
        }
    }

    fn sliders(&mut self, ui: &mut Ui) {
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

            let mut opaque = HsvaGamma::from(self.cur_color);
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

            let opaque = Hsl::from(self.cur_color);

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

            ui.collapsing("XYZ", |ui| {
                slider!(ui, xyz_x, "x", 0. ..=1., |x| Xyz { x, y: 0., z: 0. }.into());
                slider!(ui, xyz_y, "y", 0. ..=1., |y| Xyz { x: 0., y, z: 0. }.into());
                slider!(ui, xyz_z, "z", 0. ..=1., |z| Xyz { x: 0., y: 0., z }.into());
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
        let hex = self.color_hex(color);
        let color_box = tex_color(
            ui,
            tex_allocator,
            &mut self.tex_mngr,
            color.as_32(),
            size,
            Some(&color_tooltip(color, self.upper_hex)),
        );
        if let Some(color_box) = color_box {
            if with_label {
                ui.monospace(format!("#{}", hex));
            }

            if color_box.clicked() {
                self.set_cur_color(*color);
            }

            if color_box.middle_clicked() {
                self.add_color(*color);
            }

            if color_box.secondary_clicked() {
                let _ = save_to_clipboard(hex);
            }
        }
    }

    fn top_panel(&mut self, ctx: &egui::CtxRef) {
        let frame = egui::Frame {
            fill: if ctx.style().visuals.dark_mode {
                *D_BG_00
            } else {
                *L_BG_0
            },
            margin: vec2(5., 5.),
            ..Default::default()
        };
        egui::TopBottomPanel::top("top panel")
            .frame(frame)
            .show(ctx, |ui| {
                self.top_ui(ui);
            });
    }

    fn side_panel(
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

        egui::SidePanel::left("colors")
            .frame(frame)
            .show(ctx, |ui| {
                ScrollArea::auto_sized().show(ui, |ui| {
                    self.side_ui(ui, tex_allocator);
                })
            });
    }

    fn central_panel(
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
            margin: vec2(10., 5.),
            ..Default::default()
        };
        egui::CentralPanel::default().frame(_frame).show(ctx, |ui| {
            self.ui(ctx, ui, tex_allocator);
        });
    }

    fn top_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.dark_light_switch(ui);
            if ui.button("⚙").on_hover_text("Settings").clicked() {
                self.show_settings = true;
            }
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

    fn export_window(&mut self, ctx: &egui::CtxRef) {
        if self.show_export {
            let mut show = true;
            Window::new("export").open(&mut show).show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ComboBox::from_label("format")
                            .selected_text(self.export_format.as_ref())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.export_format,
                                    PaletteFormat::Gimp,
                                    PaletteFormat::Gimp.as_ref(),
                                );
                                ui.selectable_value(
                                    &mut self.export_format,
                                    PaletteFormat::Text,
                                    PaletteFormat::Text.as_ref(),
                                );
                            });
                    });
                    ui.label("Export path:");
                    ui.text_edit_singleline(&mut self.export_path);
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.export_name);

                    match &self.export_status {
                        Ok(msg) => ui.colored_label(Color32::GREEN, msg),
                        Err(msg) => ui.colored_label(Color32::RED, msg),
                    };

                    if ui.button("export").clicked() {
                        let palette = match self.export_format {
                            PaletteFormat::Gimp => {
                                self.saved_colors.as_gimp_palette(&self.export_name)
                            }
                            PaletteFormat::Text => self.saved_colors.as_text_palette(),
                        };
                        let p = PathBuf::from(&self.export_path);
                        let filename =
                            format!("{}.{}", &self.export_name, self.export_format.extension());
                        if let Err(e) = fs::write(p.join(&filename), palette) {
                            self.export_status = Err(e.to_string());
                        } else {
                            self.export_status = Ok("export succesful".to_string());
                        }
                    }
                });
            });

            if !show {
                self.show_export = false;
            }
        }
    }

    fn settings_window(&mut self, ctx: &egui::CtxRef) {
        if self.show_settings {
            let mut show = true;
            Window::new("settings").open(&mut show).show(ctx, |ui| {
                ui.checkbox(&mut self.upper_hex, "Show hex as uppercase");
            });

            if !show {
                self.show_settings = false;
            }
        }
    }

    fn dark_light_switch(&mut self, ui: &mut Ui) {
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

    fn side_ui(&mut self, ui: &mut Ui, tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Saved colors");
                ui.add_space(7.);
                if ui.button("🗑").on_hover_text("Clear colors").clicked() {
                    self.saved_colors.clear();
                }
                if ui.button("🖹").on_hover_text("Export").clicked() {
                    self.show_export = true;
                }
                if ui
                    .button("📋")
                    .on_hover_text("Copy all colors to clipboard")
                    .clicked()
                {
                    let _ = save_to_clipboard(self.saved_colors.as_text_palette());
                }
            });

            let mut src_row = None;
            let mut dst_row = None;

            for (idx, (_, color)) in self.saved_colors.as_ref().to_vec().iter().enumerate() {
                let resp = drop_target(ui, true, |ui| {
                    let color_id = Id::new("side-color").with(idx);
                    let hex = self.color_hex(color);
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

    fn ui(
        &mut self,
        ctx: &egui::CtxRef,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(err) = &self.err {
            ui.colored_label(Color32::RED, err);
        }
        self.settings_window(ctx);
        self.export_window(ctx);

        let hex = self.color_hex(&self.cur_color);

        ui.horizontal(|ui| {
            ui.label("Current color: ");
            ui.monospace(format!("#{}", hex));
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
                self.add_cur_color();
            }
        });

        self.check_color_change();
        ui.add_space(7.);

        ScrollArea::auto_sized()
            .id_source("picker scroll")
            .show(ui, |ui| {
                self.sliders(ui);
                self.hex_input(ui);
                self.schemes(ui, tex_allocator);
            });

        self.shades(ctx, tex_allocator);
        self.tints(ctx, tex_allocator);
        self.hues(ctx, tex_allocator);
    }
}
