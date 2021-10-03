mod render;
mod scheme;
mod ui;

use crate::color::{Cmyk, Color, Hsl, Lch};
use crate::picker::{self, DisplayPickerExt};
use crate::save_to_clipboard;
use render::{color_slider_1d, tex_color, TextureManager};
use ui::{color_tooltip, colors::*, dark_visuals, drag_source, drop_target, light_visuals};

use egui::{color::Color32, vec2, Ui};
use egui::{
    color::{Hsva, HsvaGamma},
    ComboBox, DragValue, Id, Rgba, ScrollArea, Vec2, Visuals, Window,
};
use std::borrow::Cow;
use std::path::PathBuf;
use std::rc::Rc;
use std::{env, fs};

#[cfg(unix)]
use x11rb::protocol::xproto;

#[cfg(not(target_arch = "wasm32"))]
use egui::TextEdit;

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
pub enum TopMenuTab {
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
        self.hsl_h = other.hsl_h;
        self.hsl_s = other.hsl_s;
        self.hsl_l = other.hsl_l;
    }
}

//####################################################################################################

#[derive(Default, Debug)]
pub struct SettingsWindow {
    pub show: bool,
    pub upper_hex: bool,
}

#[derive(Debug)]
pub struct ExportWindow {
    pub show: bool,
    pub path: String,
    pub name: String,
    pub export_status: Result<String, String>,
    pub format: PaletteFormat,
    pub export_path_editable: bool,
}

impl Default for ExportWindow {
    fn default() -> Self {
        Self {
            show: false,
            format: PaletteFormat::Gimp,
            name: "".to_string(),
            export_status: Ok("".to_string()),
            path: env::current_dir()
                .map(|d| d.to_string_lossy().to_string())
                .unwrap_or_default(),
            export_path_editable: false,
        }
    }
}

#[derive(Debug)]
pub struct ShadesWindow {
    pub num_of_shades: u8,
    pub shade_color_size: f32,
}

impl Default for ShadesWindow {
    fn default() -> Self {
        Self {
            num_of_shades: 6,
            shade_color_size: 100.,
        }
    }
}

#[derive(Debug)]
pub struct TintsWindow {
    pub num_of_tints: u8,
    pub tint_color_size: f32,
}

impl Default for TintsWindow {
    fn default() -> Self {
        Self {
            num_of_tints: 6,
            tint_color_size: 100.,
        }
    }
}

#[derive(Debug)]
pub struct HuesWindow {
    pub num_of_hues: u8,
    pub hue_color_size: f32,
    pub hues_step: f32,
}

impl Default for HuesWindow {
    fn default() -> Self {
        Self {
            num_of_hues: 4,
            hue_color_size: 100.,
            hues_step: 0.05,
        }
    }
}

//####################################################################################################

#[derive(Debug)]
pub struct ColorPicker {
    pub current_color: Color,
    pub hex_color: String,
    pub sliders: ColorSliders,
    pub saved_sliders: Option<ColorSliders>,
    pub scheme_color_size: f32,
    pub scheme_type: SchemeType,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            current_color: Color::black(),
            hex_color: "".to_string(),
            sliders: ColorSliders::default(),
            saved_sliders: None,
            scheme_color_size: 200.,
            scheme_type: SchemeType::Complementary,
        }
    }
}

//####################################################################################################

#[derive(Debug)]
pub struct App {
    pub picker: ColorPicker,
    pub texture_manager: TextureManager,
    pub display_picker: Option<Rc<dyn DisplayPickerExt>>,
    pub light_theme: Visuals,
    pub dark_theme: Visuals,
    pub saved_colors: SavedColors,
    pub error_message: Option<String>,

    pub current_tab: Option<TopMenuTab>,
    pub show_sidepanel: bool,

    pub settings_window: SettingsWindow,
    pub export_window: ExportWindow,
    pub hues_window: HuesWindow,
    pub tints_window: TintsWindow,
    pub shades_window: ShadesWindow,

    #[cfg(unix)]
    pub picker_window: Option<(xproto::Window, xproto::Gcontext)>,
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let tex_allocator = &mut Some(frame.tex_allocator());

        self.top_panel(ctx);
        if self.show_sidepanel {
            self.side_panel(ctx, tex_allocator);
        }
        self.central_panel(ctx, tex_allocator);

        frame.set_window_size(ctx.used_size());

        // No need to repaint in wasm, there is no way to pick color from under the cursor anyway
        #[cfg(not(target_arch = "wasm32"))]
        if !ctx.is_pointer_over_area() {
            // This paint request makes sure that the color displayed as color under cursor
            // gets updated even when the pointer is not in the egui window area.
            ctx.request_repaint();

            const SLEEP_DURATION: u64 = 100; // ms
            let sleep_duration = if cfg!(unix) {
                if self.picker_window.is_some() {
                    // Quicker repaints so that the zoomed window doesn't lag behind
                    SLEEP_DURATION / 4
                } else {
                    SLEEP_DURATION
                }
            } else {
                SLEEP_DURATION
            };

            std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
        }
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

impl Default for App {
    fn default() -> Self {
        Self {
            picker: ColorPicker::default(),
            texture_manager: TextureManager::default(),
            display_picker: picker::init_display_picker(),
            light_theme: light_visuals(),
            dark_theme: dark_visuals(),
            saved_colors: SavedColors::default(),
            error_message: None,

            current_tab: None,
            show_sidepanel: false,

            settings_window: SettingsWindow::default(),
            export_window: ExportWindow::default(),
            hues_window: HuesWindow::default(),
            tints_window: TintsWindow::default(),
            shades_window: ShadesWindow::default(),

            #[cfg(unix)]
            picker_window: None,
        }
    }
}

impl App {
    fn set_cur_color(&mut self, color: impl Into<Color>) {
        let color = color.into();
        self.picker.sliders.set_color(color);
        self.picker.current_color = color;
    }

    fn restore_sliders_if_saved(&mut self) {
        if let Some(saved) = std::mem::take(&mut self.picker.saved_sliders) {
            self.picker.sliders.restore(saved);
        }
    }

    fn save_sliders_if_unsaved(&mut self) {
        if self.picker.saved_sliders.is_none() {
            self.picker.saved_sliders = Some(self.picker.sliders.clone());
        }
    }

    fn rgb_changed(&mut self) -> bool {
        let rgb = self.picker.current_color.rgba();
        let r = self.picker.sliders.r / u8::MAX as f32;
        let g = self.picker.sliders.g / u8::MAX as f32;
        let b = self.picker.sliders.b / u8::MAX as f32;
        if (r - rgb.r()).abs() > f32::EPSILON
            || (g - rgb.g()).abs() > f32::EPSILON
            || (b - rgb.b()).abs() > f32::EPSILON
        {
            self.picker.saved_sliders = None;
            self.set_cur_color(Rgba::from_rgb(r, g, b));
            true
        } else {
            false
        }
    }

    fn hsva_changed(&mut self) -> bool {
        let hsva = Hsva::from(self.picker.current_color);
        if (self.picker.sliders.hue - hsva.h).abs() > f32::EPSILON
            || (self.picker.sliders.sat - hsva.s).abs() > f32::EPSILON
            || (self.picker.sliders.val - hsva.v).abs() > f32::EPSILON
        {
            if self.picker.sliders.val == 0. {
                self.save_sliders_if_unsaved();
            } else if self.picker.sliders.val > 0. {
                self.restore_sliders_if_saved();
            }
            self.set_cur_color(Hsva::new(
                self.picker.sliders.hue,
                self.picker.sliders.sat,
                self.picker.sliders.val,
                1.,
            ));
            true
        } else {
            false
        }
    }

    fn cmyk_changed(&mut self) -> bool {
        let cmyk = Cmyk::from(self.picker.current_color);
        if (self.picker.sliders.c - cmyk.c).abs() > f32::EPSILON
            || (self.picker.sliders.m - cmyk.m).abs() > f32::EPSILON
            || (self.picker.sliders.y - cmyk.y).abs() > f32::EPSILON
            || (self.picker.sliders.k - cmyk.k).abs() > f32::EPSILON
        {
            if (self.picker.sliders.k - 1.).abs() < f32::EPSILON {
                self.save_sliders_if_unsaved();
            } else if self.picker.sliders.k < 1. {
                self.restore_sliders_if_saved();
            }
            self.set_cur_color(Cmyk::new(
                self.picker.sliders.c,
                self.picker.sliders.m,
                self.picker.sliders.y,
                self.picker.sliders.k,
            ));
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn lch_changed(&mut self) -> bool {
        let lch = Lch::from(self.picker.current_color);
        if (self.picker.sliders.lch_l - lch.l).abs() > f32::EPSILON
            || (self.picker.sliders.lch_c - lch.c).abs() > f32::EPSILON
            || (self.picker.sliders.lch_h - lch.h).abs() > f32::EPSILON
        {
            self.set_cur_color(Lch::new(
                self.picker.sliders.lch_l,
                self.picker.sliders.lch_c,
                self.picker.sliders.lch_h,
            ));
            true
        } else {
            false
        }
    }

    fn hsl_changed(&mut self) -> bool {
        let hsl = Hsl::from(self.picker.current_color);
        if (self.picker.sliders.hsl_h - hsl.h).abs() > f32::EPSILON
            || (self.picker.sliders.hsl_s - hsl.s).abs() > f32::EPSILON
            || (self.picker.sliders.hsl_l - hsl.l).abs() > f32::EPSILON
        {
            self.set_cur_color(Hsl::new(
                self.picker.sliders.hsl_h,
                self.picker.sliders.hsl_s,
                self.picker.sliders.hsl_l,
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
        self.hsl_changed();
    }

    fn add_color(&mut self, color: Color) {
        if !self.saved_colors.add(color) {
            let hex = self.color_hex(&color);
            self.error_message = Some(format!("Color {} already saved!", hex));
        } else {
            self.error_message = None;
            self.show_sidepanel = true;
        }
    }

    fn add_cur_color(&mut self) {
        self.add_color(self.picker.current_color)
    }

    fn hex_input(&mut self, ui: &mut Ui) {
        ui.collapsing("Text input", |ui| {
            ui.label("Enter a hex color: ");
            ui.horizontal(|ui| {
                let resp = ui.text_edit_singleline(&mut self.picker.hex_color);
                if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                    || ui.button("▶").on_hover_text("Use this color").clicked()
                {
                    if self.picker.hex_color.len() < 6 {
                        self.error_message =
                            Some("Enter a color first (ex. ab12ff #1200ff)".to_owned());
                    } else if let Some(color) =
                        Color::from_hex(self.picker.hex_color.trim_start_matches('#'))
                    {
                        self.set_cur_color(color);
                        self.error_message = None;
                    } else {
                        self.error_message = Some("The entered hex color is not valid".to_owned());
                    }
                }
                if ui.button(ADD_ICON).on_hover_text(ADD_DESCR).clicked() {
                    self.add_cur_color()
                }
            });
        });
    }

    fn color_hex(&self, color: &Color) -> String {
        if self.settings_window.upper_hex {
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
                    let resp = color_slider_1d(&mut ui, &mut self.picker.sliders.$it, $range, $($tt)+).on_hover_text($label);
                    if resp.changed() {
                        self.check_color_change();
                    }
                    ui.add_space(7.);
                    ui.label(format!("{}: ", $label));
                    ui.add(DragValue::new(&mut self.picker.sliders.$it).clamp_range($range));
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

            let mut opaque = HsvaGamma::from(self.picker.current_color);
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

            let opaque = Hsl::from(self.picker.current_color);

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
        texture_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        with_label: bool,
    ) {
        let hex = self.color_hex(color);
        let color_box = tex_color(
            ui,
            texture_allocator,
            &mut self.texture_manager,
            color.as_32(),
            size,
            Some(&color_tooltip(color, self.settings_window.upper_hex)),
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
                self.settings_window.show = true;
            }
            if ui
                .button("↔")
                .on_hover_text("Show/hide side panel")
                .clicked()
            {
                self.show_sidepanel = !self.show_sidepanel;
            }
            ui.add_space(50.);

            ui.selectable_value(&mut self.current_tab, Some(TopMenuTab::Hues), "hues");
            ui.selectable_value(&mut self.current_tab, Some(TopMenuTab::Tints), "tints");
            ui.selectable_value(&mut self.current_tab, Some(TopMenuTab::Shades), "shades");
        });
    }

    fn export_window(&mut self, ctx: &egui::CtxRef) {
        if self.export_window.show {
            let mut show = true;
            Window::new("export").open(&mut show).show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ComboBox::from_label("format")
                            .selected_text(self.export_window.format.as_ref())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.export_window.format,
                                    PaletteFormat::Gimp,
                                    PaletteFormat::Gimp.as_ref(),
                                );
                                ui.selectable_value(
                                    &mut self.export_window.format,
                                    PaletteFormat::Text,
                                    PaletteFormat::Text.as_ref(),
                                );
                            });
                    });
                    ui.label("Export path:");
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if ui
                            .add(
                                TextEdit::singleline(&mut self.export_window.path)
                                    .enabled(self.export_window.export_path_editable),
                            )
                            .clicked()
                            && !self.export_window.export_path_editable
                        {
                            let location = if let Ok(path) = std::env::current_dir() {
                                path.to_string_lossy().to_string()
                            } else {
                                "".into()
                            };

                            match native_dialog::FileDialog::new()
                                .set_location(&location)
                                .add_filter("GIMP Palette", &["gpl"])
                                .add_filter("Text file", &["txt"])
                                .show_save_single_file()
                            {
                                Ok(Some(path)) => {
                                    self.export_window.path = path.to_string_lossy().to_string()
                                }
                                Err(e) => {
                                    self.export_window.export_path_editable = true;
                                    self.error_message = Some(e.to_string())
                                }
                                Ok(None) => {}
                            }
                        };
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        ui.text_edit_singleline(&mut self.export_window.path);
                    }

                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.export_window.name);

                    match &self.export_window.export_status {
                        Ok(msg) => ui.colored_label(Color32::GREEN, msg),
                        Err(msg) => ui.colored_label(Color32::RED, msg),
                    };

                    if ui.button("export").clicked() {
                        let palette = match self.export_window.format {
                            PaletteFormat::Gimp => {
                                self.saved_colors.as_gimp_palette(&self.export_window.name)
                            }
                            PaletteFormat::Text => self.saved_colors.as_text_palette(),
                        };
                        let p = PathBuf::from(&self.export_window.path);
                        let filename = format!(
                            "{}.{}",
                            &self.export_window.name,
                            self.export_window.format.extension()
                        );
                        if let Err(e) = fs::write(p.join(&filename), palette) {
                            self.export_window.export_status = Err(e.to_string());
                        } else {
                            self.export_window.export_status = Ok("export succesful".to_string());
                        }
                    }
                });
            });

            if !show {
                self.export_window.show = false;
            }
        }
    }

    fn settings_window(&mut self, ctx: &egui::CtxRef) {
        if self.settings_window.show {
            let mut show = true;
            Window::new("settings").open(&mut show).show(ctx, |ui| {
                ui.checkbox(&mut self.settings_window.upper_hex, "Show hex as uppercase");
            });

            if !show {
                self.settings_window.show = false;
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
                    self.export_window.show = true;
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
                                &mut self.texture_manager,
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
        if let Some(err) = &self.error_message {
            ui.colored_label(Color32::RED, err);
        }
        self.settings_window(ctx);
        self.export_window(ctx);

        let hex = self.color_hex(&self.picker.current_color);

        ui.horizontal(|ui| {
            ui.label("Current color: ");
            ui.monospace(format!("#{}", hex));
            if ui
                .button("📋")
                .on_hover_text("Copy hex color to clipboard")
                .clicked()
            {
                if let Err(e) = save_to_clipboard(format!("#{}", hex)) {
                    self.error_message = Some(format!("Failed to save color to clipboard - {}", e));
                } else {
                    self.error_message = None;
                }
            }
            if ui.button(ADD_ICON).on_hover_text(ADD_DESCR).clicked() {
                self.add_cur_color();
            }
        });

        self.handle_display_picker(ui, tex_allocator);

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

    fn handle_display_picker(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(picker) = &self.display_picker {
            const ZOOM_SCALE: f32 = 10.;
            const ZOOM_WIN_WIDTH: u16 = 160;
            const ZOOM_WIN_HEIGHT: u16 = 160;
            const ZOOM_IMAGE_WIDTH: u16 = ZOOM_WIN_WIDTH / ZOOM_SCALE as u16;
            const ZOOM_IMAGE_HEIGHT: u16 = ZOOM_WIN_HEIGHT / ZOOM_SCALE as u16;
            const ZOOM_WIN_OFFSET: i32 = 50;
            const ZOOM_WIN_POINTER_DIAMETER: u16 = 10;
            const ZOOM_WIN_POINTER_RADIUS: u16 = ZOOM_WIN_POINTER_DIAMETER / 2;
            const ZOOM_IMAGE_X_OFFSET: i32 = ((ZOOM_WIN_WIDTH / 2) as f32 / ZOOM_SCALE) as i32;
            const ZOOM_IMAGE_Y_OFFSET: i32 = ((ZOOM_WIN_HEIGHT / 2) as f32 / ZOOM_SCALE) as i32;
            let picker = Rc::clone(picker);
            let cursor_pos = picker.get_cursor_pos().unwrap_or_default();

            if let Ok(color) = picker.get_color_under_cursor() {
                ui.horizontal(|mut ui| {
                    ui.label("Color at cursor: ");
                    self.color_box_label_side(&color, vec2(25., 25.), &mut ui, tex_allocator);

                    #[cfg(unix)]
                    if ui.button("💉").clicked() {
                        if self.picker_window.is_none() {
                            if let Ok(window) = picker.spawn_window(
                                "epick - cursor picker",
                                (cursor_pos.0 + ZOOM_WIN_OFFSET) as i16,
                                (cursor_pos.1 + ZOOM_WIN_OFFSET) as i16,
                                ZOOM_WIN_WIDTH,
                                ZOOM_WIN_HEIGHT,
                                picker.screen_num(),
                                crate::picker::x11::WindowType::Dialog,
                            ) {
                                self.picker_window = Some(window);
                            }
                        } else {
                            // Close the window on second click
                            let _ = picker.destroy_window(self.picker_window.unwrap().0);
                            self.picker_window = None;
                        }
                    } else if let Some((window, gc)) = self.picker_window {
                        if let Ok(img) = picker.get_image(
                            picker.screen().root,
                            (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i16,
                            (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i16,
                            ZOOM_IMAGE_WIDTH,
                            ZOOM_IMAGE_HEIGHT,
                        ) {
                            let img = crate::picker::x11::resize_image(&img, ZOOM_SCALE);
                            if let Err(e) = img.put(picker.conn(), window, gc, 0, 0) {
                                self.error_message = Some(e.to_string());
                                return;
                            };
                            if let Err(e) = picker.draw_circle(
                                window,
                                gc,
                                ((ZOOM_WIN_WIDTH / 2) - ZOOM_WIN_POINTER_RADIUS) as i16,
                                ((ZOOM_WIN_HEIGHT / 2) - ZOOM_WIN_POINTER_RADIUS) as i16,
                                ZOOM_WIN_POINTER_DIAMETER,
                            ) {
                                self.error_message = Some(e.to_string());
                            };
                        }
                        if let Err(e) = picker.update_window_pos(
                            window,
                            cursor_pos.0 + ZOOM_WIN_OFFSET,
                            cursor_pos.1 + ZOOM_WIN_OFFSET,
                        ) {
                            self.error_message = Some(e.to_string());
                            return;
                        }
                        if let Err(e) = picker.flush() {
                            self.error_message = Some(e.to_string());
                        }
                    }
                });
            }
        };
    }
}
