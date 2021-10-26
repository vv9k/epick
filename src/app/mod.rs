#![allow(dead_code)]
mod color_picker;
mod render;
mod saved_colors;
mod scheme;
mod screen_size;
mod settings;
mod ui;

use crate::app::settings::{DisplayFmtEnum, Settings};
use crate::app::ui::windows::{
    ExportWindow, HelpWindow, HuesWindow, SettingsWindow, ShadesWindow, TintsWindow,
};
use crate::color::{Color, ColorHarmony, DisplayFormat, Gradient};
use crate::display_picker::{self, DisplayPickerExt};
use crate::save_to_clipboard;
use color_picker::ColorPicker;
use render::{tex_color, TextureManager};
use saved_colors::SavedColors;
use screen_size::ScreenSize;
use ui::{color_tooltip, colors::*, dark_visuals, drag_source, drop_target, light_visuals};

use egui::{
    color::Color32, vec2, Button, CollapsingHeader, CursorIcon, Label, Layout, Rgba, TextStyle, Ui,
};
use egui::{Id, ScrollArea, Vec2, Visuals};
use std::borrow::Cow;
use std::rc::Rc;

#[cfg(target_os = "linux")]
use x11rb::protocol::xproto;

use crate::app::render::tex_gradient;
use crate::app::ui::SPACE;
#[cfg(windows)]
use crate::display_picker::windows::{HWND, SW_SHOWDEFAULT, WS_BORDER, WS_POPUP};
use epi::Storage;

pub static ADD_ICON: &str = "\u{2795}";
pub static COPY_ICON: &str = "\u{1F3F7}";
pub static ZOOM_PICKER_ICON: &str = "\u{1F489}";
pub static SETTINGS_ICON: &str = "\u{2699}";
pub static EXPAND_ICON: &str = "\u{2B0C}";
pub static EXPORT_ICON: &str = "\u{1F5B9}";
pub static CLEAR_ICON: &str = "\u{1F5D1}";
pub static DELETE_ICON: &str = "\u{1F5D9}";
pub static PLAY_ICON: &str = "\u{25B6}";
pub static DARK_MODE_ICON: &str = "\u{1F319}";
pub static LIGHT_MODE_ICON: &str = "\u{2600}";
pub static HELP_ICON: &str = "\u{FF1F}";

static ADD_DESCR: &str = "Add this color to saved colors";
static CURSOR_PICKER_WINDOW_NAME: &str = "epick - cursor picker";

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
    pub screen_size: ScreenSize,
    pub cursor_icon: CursorIcon,

    pub show_side_panel: bool,
    pub side_panel_box_width: f32,

    pub settings_window: SettingsWindow,
    pub export_window: ExportWindow,
    pub help_window: HelpWindow,
    pub hues_window: HuesWindow,
    pub tints_window: TintsWindow,
    pub shades_window: ShadesWindow,

    #[cfg(target_os = "linux")]
    pub picker_window: Option<(xproto::Window, xproto::Gcontext)>,
    #[cfg(windows)]
    pub picker_window: Option<HWND>,
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        ctx.output().cursor_icon = self.cursor_icon;
        let tex_allocator = &mut Some(frame.tex_allocator());

        let screen_size = ScreenSize::from(ctx.available_rect());
        if self.screen_size != screen_size {
            self.set_styles(ctx, screen_size);
        }

        self.check_keys_pressed(ctx);

        self.check_settings_change();

        self.top_panel(ctx);

        self.central_panel(ctx, tex_allocator);

        if self.show_side_panel {
            self.side_panel(ctx, tex_allocator);
        }

        self.display_windows(ctx, tex_allocator);

        frame.set_window_size(ctx.used_size());

        self.picker.check_for_change();

        // No need to repaint in wasm, there is no way to pick color from under the cursor anyway
        #[cfg(not(target_arch = "wasm32"))]
        if !ctx.is_pointer_over_area() {
            // This paint request makes sure that the color displayed as color under cursor
            // gets updated even when the pointer is not in the egui window area.
            ctx.request_repaint();

            const SLEEP_DURATION: u64 = 100; // ms
            #[cfg(any(target_os = "linux", windows))]
            let sleep_duration = if self.picker_window.is_some() {
                // Quicker repaints so that the zoomed window doesn't lag behind
                SLEEP_DURATION / 4
            } else {
                SLEEP_DURATION
            };
            #[cfg(not(any(target_os = "linux", windows)))]
            let sleep_duration = SLEEP_DURATION;

            std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        self.save_colors(storage);
        self.save_settings(storage);
        storage.flush();
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        self.load_settings(storage);
        self.load_colors(storage);

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Firacode".to_string(),
            Cow::Borrowed(include_bytes!(
                "../../assets/fonts/FiraCode/FiraCode-Regular.ttf"
            )),
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
        ctx.set_fonts(fonts);

        let prefer_dark = frame.info().prefer_dark_mode.unwrap_or(true);

        if prefer_dark {
            self.set_dark_theme(ctx);
        } else {
            self.set_light_theme(ctx);
        }
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
            display_picker: display_picker::init_display_picker(),
            light_theme: light_visuals(),
            dark_theme: dark_visuals(),
            saved_colors: SavedColors::default(),
            error_message: None,
            screen_size: ScreenSize::Desktop(0., 0.),
            cursor_icon: CursorIcon::default(),

            show_side_panel: false,
            side_panel_box_width: 0.,

            settings_window: SettingsWindow::default(),
            export_window: ExportWindow::default(),
            help_window: HelpWindow::default(),
            hues_window: HuesWindow::default(),
            tints_window: TintsWindow::default(),
            shades_window: ShadesWindow::default(),

            #[cfg(target_os = "linux")]
            picker_window: None,
            #[cfg(windows)]
            picker_window: None,
        }
    }
}

impl App {
    fn set_error(&mut self, error: impl std::fmt::Display) {
        self.error_message = Some(error.to_string());
    }

    fn clear_error(&mut self) {
        self.error_message = None;
    }

    fn set_dark_theme(&mut self, ctx: &egui::CtxRef) {
        self.settings_window.settings.is_dark_mode = true;
        ctx.set_visuals(self.dark_theme.clone());
    }

    fn set_light_theme(&mut self, ctx: &egui::CtxRef) {
        self.settings_window.settings.is_dark_mode = false;
        ctx.set_visuals(self.light_theme.clone());
    }

    fn is_dark_mode(&self) -> bool {
        self.settings_window.settings.is_dark_mode
    }

    fn load_colors(&mut self, _storage: Option<&dyn Storage>) {
        if self.settings_window.settings.cache_colors {
            #[cfg(target_arch = "wasm32")]
            if let Some(storage) = _storage {
                if let Some(yaml) = storage.get_string(SavedColors::STORAGE_KEY) {
                    if let Ok(colors) = SavedColors::from_yaml_str(&yaml) {
                        self.saved_colors = colors;
                        if !self.saved_colors.is_empty() {
                            self.show_side_panel = true;
                        }
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(path) = SavedColors::dir("epick") {
                if let Ok(colors) = SavedColors::load(path.join(SavedColors::FILE_NAME)) {
                    self.saved_colors = colors;
                    if !self.saved_colors.is_empty() {
                        self.show_side_panel = true;
                    }
                }
            }
        }
    }

    fn save_colors(&self, _storage: &mut dyn Storage) {
        #[cfg(target_arch = "wasm32")]
        if self.settings_window.settings.cache_colors {
            if let Ok(yaml) = self.saved_colors.as_yaml_str() {
                _storage.set_string(SavedColors::STORAGE_KEY, yaml);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(dir) = SavedColors::dir("epick") {
            let _ = self.saved_colors.save(dir.join(SavedColors::FILE_NAME));
        }
    }

    fn save_settings(&self, _storage: &mut dyn Storage) {
        #[cfg(target_arch = "wasm32")]
        if let Ok(yaml) = self.settings_window.settings.as_yaml_str() {
            _storage.set_string(Settings::STORAGE_KEY, yaml);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(dir) = Settings::dir("epick") {
            let _ = self
                .settings_window
                .settings
                .save(dir.join(Settings::FILE_NAME));
        }
    }

    fn load_settings(&mut self, _storage: Option<&dyn Storage>) {
        #[cfg(target_arch = "wasm32")]
        if let Some(storage) = _storage {
            if let Some(yaml) = storage.get_string(Settings::STORAGE_KEY) {
                if let Ok(settings) = Settings::from_yaml_str(&yaml) {
                    self.settings_window.settings = settings;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(config_dir) = Settings::dir("epick") {
            let path = config_dir.join(Settings::FILE_NAME);

            if let Ok(settings) = Settings::load(&path) {
                self.settings_window.settings = settings;
            }
        }
    }

    fn set_styles(&mut self, ctx: &egui::CtxRef, screen_size: ScreenSize) {
        self.screen_size = screen_size;

        let slider_size = match screen_size {
            ScreenSize::Phone(w, _) => w * 0.5,
            ScreenSize::Desktop(w, _) if w > 1500. => w * 0.2,
            ScreenSize::Tablet(w, _) | ScreenSize::Laptop(w, _) | ScreenSize::Desktop(w, _) => {
                w * 0.35
            }
        };

        let mut style = (*ctx.style()).clone();
        style.spacing.slider_width = slider_size / 2.;
        ctx.set_style(style);
    }

    fn check_settings_change(&mut self) {
        if self.settings_window.settings.chromatic_adaptation_method
            != self.picker.sliders.chromatic_adaptation_method
        {
            self.picker.sliders.chromatic_adaptation_method =
                self.settings_window.settings.chromatic_adaptation_method;
        }
        if self.settings_window.settings.rgb_working_space != self.picker.sliders.rgb_working_space
        {
            self.picker.new_workspace = Some(self.settings_window.settings.rgb_working_space);
        }
        if self.settings_window.settings.illuminant != self.picker.sliders.illuminant {
            self.picker.new_illuminant = Some(self.settings_window.settings.illuminant);
        }
    }

    fn check_keys_pressed(&mut self, ctx: &egui::CtxRef) {
        if ctx.input().key_pressed(egui::Key::H) {
            self.show_side_panel = !self.show_side_panel;
        }
    }

    fn add_color(&mut self, color: Color) {
        if !self.saved_colors.add(color) {
            let color_str = self.display_color(&color);
            self.set_error(format!("Color {} already saved!", color_str));
        } else {
            self.clear_error();
            self.show_side_panel = true;
        }
    }

    fn add_cur_color(&mut self) {
        self.add_color(self.picker.current_color)
    }

    fn hex_input(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("Text input")
            .text_style(TextStyle::Heading)
            .show(ui, |ui| {
                ui.label("Enter a hex color: ");
                ui.horizontal(|ui| {
                    let resp = ui.text_edit_singleline(&mut self.picker.hex_color);
                    if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                        || ui
                            .button(PLAY_ICON)
                            .on_hover_text("Use this color")
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                    {
                        if self.picker.hex_color.len() < 6 {
                            self.set_error("Enter a color first (ex. ab12ff #1200ff)".to_owned());
                        } else if let Some(color) =
                            Color::from_hex(self.picker.hex_color.trim_start_matches('#'))
                        {
                            self.picker.set_cur_color(color);
                            self.clear_error();
                        } else {
                            self.set_error("The entered hex color is not valid".to_owned());
                        }
                    }
                    if ui
                        .button(ADD_ICON)
                        .on_hover_text(ADD_DESCR)
                        .on_hover_cursor(CursorIcon::Copy)
                        .clicked()
                    {
                        self.add_cur_color()
                    }
                });
            });
    }

    fn display_format(&self) -> DisplayFormat {
        match self.settings_window.settings.color_display_format {
            DisplayFmtEnum::Hex => DisplayFormat::Hex,
            DisplayFmtEnum::HexUppercase => DisplayFormat::HexUpercase,
            DisplayFmtEnum::CssRgb => DisplayFormat::CssRgb,
            DisplayFmtEnum::CssHsl => DisplayFormat::CssHsl {
                degree_symbol: true,
            },
            DisplayFmtEnum::Custom => {
                DisplayFormat::Custom(&self.settings_window.settings.custom_display_fmt_str)
            }
        }
    }

    fn display_color(&self, color: &Color) -> String {
        color.display(
            self.display_format(),
            self.settings_window.settings.rgb_working_space,
            self.settings_window.settings.illuminant,
        )
    }

    fn clipboard_color(&self, color: &Color) -> String {
        let format = match &self.settings_window.settings.color_display_format {
            DisplayFmtEnum::Hex => DisplayFormat::Hex,
            DisplayFmtEnum::HexUppercase => DisplayFormat::HexUpercase,
            DisplayFmtEnum::CssRgb => DisplayFormat::CssRgb,
            DisplayFmtEnum::CssHsl => DisplayFormat::CssHsl {
                degree_symbol: false,
            },
            DisplayFmtEnum::Custom => {
                DisplayFormat::Custom(&self.settings_window.settings.custom_clipboard_fmt_str)
            }
        };
        color.display(
            format,
            self.settings_window.settings.rgb_working_space,
            self.settings_window.settings.illuminant,
        )
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
        let color_str = self.display_color(color);
        let format = self.display_format();
        let on_hover = color_tooltip(
            color,
            format,
            self.settings_window.settings.rgb_working_space,
            self.settings_window.settings.illuminant,
        );
        let color_box = tex_color(
            ui,
            texture_allocator,
            &mut self.texture_manager,
            color.color32(),
            size,
            Some(&on_hover),
        );
        if let Some(color_box) = color_box {
            if with_label {
                ui.monospace(&color_str);
            }

            if color_box.clicked() {
                self.picker.set_cur_color(*color);
            }

            if color_box.middle_clicked() {
                self.add_color(*color);
            }

            if color_box.secondary_clicked() {
                let _ = save_to_clipboard(color_str);
            }
        }
    }

    fn gradient_box(
        &mut self,
        gradient: &Gradient,
        size: Vec2,
        ui: &mut Ui,
        texture_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        let _ = tex_gradient(
            ui,
            texture_allocator,
            &mut self.texture_manager,
            gradient,
            size,
            None,
        );
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

        egui::SidePanel::right("colors")
            .frame(frame)
            .resizable(false)
            .max_width(self.side_panel_box_width * 1.2)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
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
            self.ui(ui, tex_allocator);
        });
    }

    fn top_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if self.hues_window.is_open {
                if ui
                    .button("hues")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.hues_window.is_open = false;
                }
            } else {
                let btn = Button::new("hues").fill(Rgba::from_black_alpha(0.));
                if ui
                    .add(btn)
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.hues_window.is_open = true;
                }
            }
            if self.tints_window.is_open {
                if ui
                    .button("tints")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.tints_window.is_open = false;
                }
            } else {
                let btn = Button::new("tints").fill(Rgba::from_black_alpha(0.));
                if ui
                    .add(btn)
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.tints_window.is_open = true;
                }
            }

            if self.shades_window.is_open {
                if ui
                    .button("shades")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.shades_window.is_open = false;
                }
            } else {
                let btn = Button::new("shades").fill(Rgba::from_black_alpha(0.));
                if ui
                    .add(btn)
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.shades_window.is_open = true;
                }
            }

            ui.with_layout(Layout::right_to_left(), |ui| {
                if ui
                    .button(HELP_ICON)
                    .on_hover_text("Show help")
                    .on_hover_cursor(CursorIcon::Help)
                    .clicked()
                {
                    self.help_window.toggle_window();
                }
                if ui
                    .button(EXPAND_ICON)
                    .on_hover_text("Show/hide side panel")
                    .on_hover_cursor(CursorIcon::ResizeHorizontal)
                    .clicked()
                {
                    self.show_side_panel = !self.show_side_panel;
                }
                if ui
                    .button(SETTINGS_ICON)
                    .on_hover_text("Settings")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.settings_window.show = true;
                }
                self.dark_light_switch(ui);
            });
        });
    }

    fn dark_light_switch(&mut self, ui: &mut Ui) {
        let btn = if self.is_dark_mode() {
            LIGHT_MODE_ICON
        } else {
            DARK_MODE_ICON
        };
        if ui
            .button(btn)
            .on_hover_text("Switch ui color theme")
            .on_hover_cursor(CursorIcon::PointingHand)
            .clicked()
        {
            if self.is_dark_mode() {
                self.set_light_theme(ui.ctx());
            } else {
                self.set_dark_theme(ui.ctx());
            }
        }
    }

    fn side_ui(&mut self, ui: &mut Ui, tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>) {
        ui.vertical(|ui| {
            let resp = ui.horizontal(|ui| {
                let heading = Label::new("Saved colors")
                    .text_style(TextStyle::Heading)
                    .strong();
                ui.add(heading);
                ui.add_space(SPACE);
                if ui
                    .button(CLEAR_ICON)
                    .on_hover_text("Clear colors")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.saved_colors.clear();
                }
                if ui
                    .button(EXPORT_ICON)
                    .on_hover_text("Export")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.export_window.show = true;
                }
                if ui
                    .button(COPY_ICON)
                    .on_hover_text("Copy all colors to clipboard")
                    .on_hover_cursor(CursorIcon::Alias)
                    .clicked()
                {
                    let _ = save_to_clipboard(self.saved_colors.as_hex_list());
                }
            });
            let sidebar_w = resp.response.rect.width();

            let mut src_row = None;
            let mut dst_row = None;

            let saved_colors = self.saved_colors.as_ref().to_vec();
            let display_strings: Vec<_> = saved_colors
                .iter()
                .map(|(_, c)| self.display_color(c))
                .collect();
            let max_len = display_strings
                .iter()
                .map(|s| s.len())
                .max()
                .unwrap_or_default();
            let box_width = (max_len * 11).max((sidebar_w * 0.8) as usize).min(220) as f32;

            for (idx, (_, color)) in saved_colors.iter().enumerate() {
                let resp = drop_target(ui, true, |ui| {
                    let color_id = Id::new("side-color").with(idx);
                    let color_str = &display_strings[idx];
                    ui.vertical(|ui| {
                        let box_response = ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                if ui
                                    .button(PLAY_ICON)
                                    .on_hover_text("Use this color")
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    self.picker.set_cur_color(*color);
                                }
                                if ui
                                    .button(COPY_ICON)
                                    .on_hover_text("Copy color")
                                    .on_hover_cursor(CursorIcon::Alias)
                                    .clicked()
                                {
                                    let _ = save_to_clipboard(
                                        self.clipboard_color(&self.picker.current_color),
                                    );
                                }
                                if ui
                                    .button(DELETE_ICON)
                                    .on_hover_text("Delete this color")
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    self.saved_colors.remove(color);
                                }
                            });
                            ui.vertical(|ui| {
                                ui.monospace(color_str);
                                let help = format!(
                                    "{}\n\nDrag and drop to change the order of colors",
                                    color_str
                                );

                                let size = vec2(box_width, box_width / 2.);
                                drag_source(ui, color_id, |ui| {
                                    tex_color(
                                        ui,
                                        tex_allocator,
                                        &mut self.texture_manager,
                                        color.color32(),
                                        size,
                                        Some(&help),
                                    );
                                });
                            });
                        });
                        self.side_panel_box_width = box_response.response.rect.width();
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

    fn display_windows(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        self.settings_window.display(ctx);
        if let Err(e) = self.export_window.display(ctx, &self.saved_colors) {
            self.set_error(e);
        }

        self.shades_window(ctx, tex_allocator);
        self.tints_window(ctx, tex_allocator);
        self.hues_window(ctx, tex_allocator);
        self.help_window.display(ctx);
    }

    fn ui(&mut self, ui: &mut Ui, tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>) {
        if let Some(err) = &self.error_message {
            ui.colored_label(Color32::RED, err);
        }

        let color_str = self.display_color(&self.picker.current_color);

        ui.horizontal(|ui| {
            ui.label("Current color: ");
            if ui
                .button(COPY_ICON)
                .on_hover_text("Copy color to clipboard")
                .on_hover_cursor(CursorIcon::Alias)
                .clicked()
            {
                if let Err(e) = save_to_clipboard(self.clipboard_color(&self.picker.current_color))
                {
                    self.set_error(format!("Failed to save color to clipboard - {}", e));
                } else {
                    self.clear_error();
                }
            }
            if ui
                .button(ADD_ICON)
                .on_hover_text(ADD_DESCR)
                .on_hover_cursor(CursorIcon::Copy)
                .clicked()
            {
                self.add_cur_color();
            }
            ui.monospace(&color_str);
        });

        self.handle_display_picker(ui, tex_allocator);

        ui.add_space(SPACE);
        ScrollArea::vertical()
            .id_source("picker scroll")
            .show(ui, |ui| {
                self.harmonies(ui, tex_allocator);
                self.sliders(ui);
                self.hex_input(ui);
            });
    }

    fn sliders(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            if self.settings_window.settings.color_spaces.rgb {
                self.picker.rgb_sliders(ui);
            }
            if self.settings_window.settings.color_spaces.cmyk {
                self.picker.cmyk_sliders(ui);
            }
            if self.settings_window.settings.color_spaces.hsv {
                self.picker.hsv_sliders(ui);
            }
            if self.settings_window.settings.color_spaces.hsl {
                self.picker.hsl_sliders(ui);
            }
            if self.settings_window.settings.color_spaces.luv {
                self.picker.luv_sliders(ui);
            }
            if self.settings_window.settings.color_spaces.lch_uv {
                self.picker.lch_uv_sliders(ui);
            }
            if self.settings_window.settings.color_spaces.lab {
                self.picker.lab_sliders(ui);
            }
            if self.settings_window.settings.color_spaces.lch_ab {
                self.picker.lch_ab_sliders(ui);
            }
        });
    }

    fn handle_display_picker(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(picker) = self.display_picker.clone() {
            if let Ok(color) = picker.get_color_under_cursor() {
                if ui.ctx().input().key_pressed(egui::Key::P) {
                    self.picker.set_cur_color(color);
                }
                if ui.ctx().input().key_pressed(egui::Key::Z) {
                    self.toggle_zoom_window(&picker);
                }
                if ui.ctx().input().key_pressed(egui::Key::S) {
                    self.saved_colors.add(color);
                }
                ui.horizontal(|ui| {
                    ui.label("Color at cursor: ");
                    #[cfg(target_os = "linux")]
                    self.handle_zoom_picker(ui, picker);
                    #[cfg(windows)]
                    self.handle_zoom_picker(ui, picker);
                    self.color_box_label_side(&color, vec2(25., 25.), ui, tex_allocator);
                });
            }
        };
    }

    #[cfg(not(any(target_os = "linux", windows)))]
    fn toggle_zoom_window(&mut self, _: &Rc<dyn DisplayPickerExt>) {}

    fn toggle_mouse(&mut self, icon: CursorIcon) {
        self.cursor_icon = if icon == self.cursor_icon {
            CursorIcon::default()
        } else {
            icon
        }
    }

    #[cfg(target_os = "linux")]
    fn toggle_zoom_window(&mut self, picker: &Rc<dyn DisplayPickerExt>) {
        self.toggle_mouse(CursorIcon::Crosshair);
        let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
        if self.picker_window.is_none() {
            if let Ok(window) = picker.spawn_window(
                CURSOR_PICKER_WINDOW_NAME,
                (cursor_pos.0 + ZOOM_WIN_OFFSET) as i16,
                (cursor_pos.1 + ZOOM_WIN_OFFSET) as i16,
                ZOOM_WIN_WIDTH,
                ZOOM_WIN_HEIGHT,
                picker.screen_num(),
                crate::display_picker::x11::WindowType::Dialog,
            ) {
                self.picker_window = Some(window);
            }
        } else {
            // Close the window on second click
            let _ = picker.destroy_window(self.picker_window.unwrap().0);
            self.picker_window = None;
        }
    }

    #[cfg(target_os = "linux")]
    fn handle_zoom_picker(&mut self, ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
        if ui
            .button(ZOOM_PICKER_ICON)
            .on_hover_cursor(CursorIcon::ZoomIn)
            .clicked()
        {
            self.toggle_zoom_window(&picker);
        } else if let Some((window, gc)) = self.picker_window {
            if let Ok(img) = picker.get_image(
                picker.screen().root,
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i16,
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i16,
                ZOOM_IMAGE_WIDTH,
                ZOOM_IMAGE_HEIGHT,
            ) {
                let img = crate::display_picker::x11::resize_image(&img, ZOOM_SCALE);
                if let Err(e) = img.put(picker.conn(), window, gc, 0, 0) {
                    self.set_error(e);
                    return;
                };

                if let Err(e) = picker.draw_circle(
                    window,
                    gc,
                    (ZOOM_WIN_WIDTH / 2) as i16,
                    (ZOOM_WIN_HEIGHT / 2) as i16,
                    ZOOM_WIN_POINTER_DIAMETER,
                ) {
                    self.set_error(e);
                };
            }
            if let Err(e) = picker.update_window_pos(
                window,
                cursor_pos.0 + ZOOM_WIN_OFFSET,
                cursor_pos.1 + ZOOM_WIN_OFFSET,
            ) {
                self.set_error(e);
                return;
            }
            if let Err(e) = picker.flush() {
                self.set_error(e);
            }
        }
    }

    #[cfg(windows)]
    fn toggle_zoom_window(&mut self, picker: &Rc<dyn DisplayPickerExt>) {
        self.toggle_mouse(CursorIcon::Crosshair);
        let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
        if self.picker_window.is_none() {
            if let Ok(window) = picker.spawn_window(
                "EPICK_DIALOG",
                CURSOR_PICKER_WINDOW_NAME,
                (cursor_pos.0 + ZOOM_WIN_OFFSET) as i32,
                (cursor_pos.1 + ZOOM_WIN_OFFSET) as i32,
                ZOOM_WIN_WIDTH as i32,
                ZOOM_WIN_HEIGHT as i32,
                WS_POPUP | WS_BORDER,
            ) {
                self.picker_window = Some(window);
                if let Err(e) = picker.show_window(window, SW_SHOWDEFAULT) {
                    self.set_error(e);
                }
            }
        } else {
            // Close the window on second click
            if let Err(e) = picker.destroy_window(self.picker_window.unwrap()) {
                self.set_error(e);
            }
            self.picker_window = None;
        }
    }

    #[cfg(windows)]
    fn handle_zoom_picker(&mut self, ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
        if ui
            .button(ZOOM_PICKER_ICON)
            .on_hover_cursor(CursorIcon::ZoomIn)
            .clicked()
        {
            self.toggle_zoom_window(&picker);
        } else if let Some(window) = self.picker_window {
            match picker.get_screenshot(
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i32,
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i32,
                (ZOOM_WIN_WIDTH as f32 / ZOOM_SCALE) as i32,
                (ZOOM_WIN_HEIGHT as f32 / ZOOM_SCALE) as i32,
            ) {
                Ok(bitmap) => {
                    if let Err(e) = picker.render_bitmap(&bitmap, window, 0, 0, ZOOM_SCALE) {
                        self.set_error(e);
                    }
                    let left = ((ZOOM_WIN_WIDTH / 2) - ZOOM_WIN_POINTER_RADIUS) as i32;
                    let top = ((ZOOM_WIN_HEIGHT / 2) - ZOOM_WIN_POINTER_RADIUS) as i32;
                    if let Err(e) = picker.draw_rectangle(
                        window,
                        left,
                        top,
                        left + ZOOM_WIN_POINTER_DIAMETER as i32,
                        top + ZOOM_WIN_POINTER_DIAMETER as i32,
                        true,
                    ) {
                        self.set_error(e);
                    }
                }
                Err(e) => {
                    self.set_error(e);
                }
            }
            if let Err(e) = picker.move_window(
                window,
                (cursor_pos.0 + ZOOM_WIN_OFFSET) as i32,
                (cursor_pos.1 + ZOOM_WIN_OFFSET) as i32,
                ZOOM_WIN_WIDTH as i32,
                ZOOM_WIN_HEIGHT as i32,
            ) {
                self.set_error(e);
            }
        }
    }
}
