#![allow(dead_code)]
mod color_picker;
mod display_picker;
mod error;
mod palettes;
mod render;
mod scheme;
mod screen_size;
mod settings;
mod ui;

use crate::color::{Color, ColorHarmony, DisplayFormat, Gradient};
use crate::{save_to_clipboard, TextureAllocator};
use color_picker::ColorPicker;
use display_picker::DisplayPickerExt;
use error::{append_global_error, DisplayError, ERROR_STACK};
use palettes::Palettes;
use render::tex_color;
use screen_size::ScreenSize;
use settings::{DisplayFmtEnum, Settings};
use ui::{
    color_tooltip,
    colors::*,
    dark_visuals, drag_source, drop_target, light_visuals,
    windows::{ExportWindow, HelpWindow, HuesWindow, SettingsWindow, ShadesWindow, TintsWindow},
};

use eframe::{CreationContext, Storage};
use egui::{
    color::Color32, style::Margin, vec2, Button, CollapsingHeader, CursorIcon, Id, Label, Layout,
    Rgba, RichText, ScrollArea, Ui, Vec2, Visuals,
};
use std::rc::Rc;
use std::time::Duration;

#[cfg(target_os = "linux")]
use x11rb::protocol::xproto;

#[cfg(windows)]
use crate::app::display_picker::windows::{HWND, SW_SHOWDEFAULT, WS_BORDER, WS_POPUP};
use crate::app::render::tex_gradient;
use crate::app::ui::{HALF_SPACE, SPACE};

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
pub static EDIT_ICON: &str = "\u{270F}";
pub static APPLY_ICON: &str = "\u{2714}";

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
const ZOOM_WIN_BORDER_WIDTH: u32 = 2;
const ZOOM_IMAGE_X_OFFSET: i32 = ((ZOOM_WIN_WIDTH / 2) as f32 / ZOOM_SCALE) as i32;
const ZOOM_IMAGE_Y_OFFSET: i32 = ((ZOOM_WIN_HEIGHT / 2) as f32 / ZOOM_SCALE) as i32;

const ERROR_DISPLAY_DURATION: Duration = Duration::new(20, 0);

//####################################################################################################

#[derive(Debug)]
pub struct App {
    pub picker: ColorPicker,
    pub texture_manager: render::TextureManager,
    pub display_picker: Option<Rc<dyn DisplayPickerExt>>,
    pub light_theme: Visuals,
    pub dark_theme: Visuals,
    pub palettes: Palettes,
    pub error_message: Option<String>,
    pub screen_size: ScreenSize,
    pub cursor_icon: CursorIcon,

    pub show_side_panel: bool,
    pub edit_side_palette_name: bool,
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
    pub zoom_window_dragged: bool,

    pub display_errors: Vec<DisplayError>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.output().cursor_icon = self.cursor_icon;
        let tex_allocator = &mut Some(ctx.tex_manager());

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

        // populate display errors from the global error stack
        if let Ok(mut stack) = ERROR_STACK.try_lock() {
            while let Some(error) = stack.errors.pop_front() {
                self.display_errors.push(error);
            }
        }

        // No need to repaint in wasm, there is no way to pick color from under the cursor anyway
        #[cfg(not(target_arch = "wasm32"))]
        if !ctx.is_pointer_over_area() {
            // This paint request makes sure that the color displayed as color under cursor
            // gets updated even when the pointer is not in the egui window area.
            ctx.request_repaint();

            if self.zoom_window_dragged {
                // When zooming we want to continually repaint for smooth experience
                // even if the pointer is not over main window area
                return;
            }

            // Otherwise sleep to save some cycles
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        self.save_colors(storage);
        self.save_settings(storage);
        storage.flush();
    }

    fn max_size_points(&self) -> egui::Vec2 {
        vec2(4096., 8192.)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            picker: ColorPicker::default(),
            texture_manager: render::TextureManager::default(),
            display_picker: display_picker::init_display_picker(),
            light_theme: light_visuals(),
            dark_theme: dark_visuals(),
            palettes: Palettes::default(),
            error_message: None,
            screen_size: ScreenSize::Desktop(0., 0.),
            cursor_icon: CursorIcon::default(),

            show_side_panel: false,
            edit_side_palette_name: false,
            side_panel_box_width: 0.,

            settings_window: SettingsWindow::default(),
            export_window: ExportWindow::default(),
            help_window: HelpWindow::default(),
            hues_window: HuesWindow::default(),
            tints_window: TintsWindow::default(),
            shades_window: ShadesWindow::default(),

            display_errors: Default::default(),

            #[cfg(target_os = "linux")]
            picker_window: None,
            #[cfg(windows)]
            picker_window: None,
            zoom_window_dragged: false,
        }
    }
}

impl App {
    pub fn init(context: &CreationContext) -> Box<dyn eframe::App + 'static> {
        let mut app = Box::new(App::default());
        app.load_settings(context.storage);
        app.load_colors(context.storage);

        let prefer_dark = context.integration_info.prefer_dark_mode.unwrap_or(true);

        if prefer_dark {
            app.set_dark_theme(&context.egui_ctx);
        } else {
            app.set_light_theme(&context.egui_ctx);
        }

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Firacode".to_string(),
            egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/FiraCode/FiraCode-Regular.ttf"
            )),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "Firacode".to_owned());

        context.egui_ctx.set_fonts(fonts);

        app
    }

    fn set_dark_theme(&mut self, ctx: &egui::Context) {
        self.settings_window.settings.is_dark_mode = true;
        ctx.set_visuals(self.dark_theme.clone());
    }

    fn set_light_theme(&mut self, ctx: &egui::Context) {
        self.settings_window.settings.is_dark_mode = false;
        ctx.set_visuals(self.light_theme.clone());
    }

    fn is_dark_mode(&self) -> bool {
        self.settings_window.settings.is_dark_mode
    }

    fn set_theme(&mut self, ctx: &egui::Context) {
        if self.is_dark_mode() {
            self.set_light_theme(ctx);
        } else {
            self.set_dark_theme(ctx);
        }
    }

    fn load_colors(&mut self, _storage: Option<&dyn Storage>) {
        if self.settings_window.settings.cache_colors {
            #[cfg(target_arch = "wasm32")]
            if let Some(storage) = _storage {
                if let Some(yaml) = storage.get_string(Palettes::STORAGE_KEY) {
                    if let Ok(colors) = Palettes::from_yaml_str(&yaml) {
                        self.saved_colors = colors;
                        if !self.saved_colors.is_empty() {
                            self.show_side_panel = true;
                        }
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(path) = Palettes::dir("epick") {
                if let Ok(palettes) = Palettes::load(path.join(Palettes::FILE_NAME)) {
                    self.palettes = palettes;
                }
            }
        }
    }

    fn save_colors(&self, _storage: &mut dyn Storage) {
        #[cfg(target_arch = "wasm32")]
        if self.settings_window.settings.cache_colors {
            if let Ok(yaml) = self.saved_colors.as_yaml_str() {
                _storage.set_string(Palettes::STORAGE_KEY, yaml);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(dir) = Palettes::dir("epick") {
            let _ = self.palettes.save(dir.join(Palettes::FILE_NAME));
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

    fn set_styles(&mut self, ctx: &egui::Context, screen_size: ScreenSize) {
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

    fn check_keys_pressed(&mut self, ctx: &egui::Context) {
        if ctx.input().key_pressed(egui::Key::H) {
            self.show_side_panel = !self.show_side_panel;
        }
    }

    fn add_color(&mut self, color: Color) {
        if !self.palettes.current_mut().palette.add(color) {
            let color_str = self.display_color(&color);
            append_global_error(format!("Color {} already saved!", color_str));
        } else {
            self.show_side_panel = true;
        }
    }

    fn add_cur_color(&mut self) {
        self.add_color(self.picker.current_color)
    }

    fn hex_input(&mut self, ui: &mut Ui) {
        CollapsingHeader::new("Text input").show(ui, |ui| {
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
                        append_global_error("Enter a color first (ex. ab12ff #1200ff)".to_owned());
                    } else if let Some(color) =
                        Color::from_hex(self.picker.hex_color.trim_start_matches('#'))
                    {
                        self.picker.set_cur_color(color);
                    } else {
                        append_global_error("The entered hex color is not valid".to_owned());
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
            DisplayFmtEnum::Custom(ref name) => {
                if self
                    .settings_window
                    .settings
                    .saved_color_formats
                    .get(name)
                    .is_some()
                {
                    DisplayFormat::Custom(&self.settings_window.settings.saved_color_formats[name])
                } else {
                    append_global_error(format!("Custom color format `{name}` not found"));
                    DisplayFmtEnum::default_display_format()
                }
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
        let format = match self
            .settings_window
            .settings
            .color_clipboard_format
            .as_ref()
            .unwrap_or(&self.settings_window.settings.color_display_format)
        {
            DisplayFmtEnum::Hex => DisplayFormat::Hex,
            DisplayFmtEnum::HexUppercase => DisplayFormat::HexUpercase,
            DisplayFmtEnum::CssRgb => DisplayFormat::CssRgb,
            DisplayFmtEnum::CssHsl => DisplayFormat::CssHsl {
                degree_symbol: false,
            },
            DisplayFmtEnum::Custom(name) => {
                if self
                    .settings_window
                    .settings
                    .saved_color_formats
                    .get(name)
                    .is_some()
                {
                    DisplayFormat::Custom(&self.settings_window.settings.saved_color_formats[name])
                } else {
                    append_global_error(format!("Custom color format `{name}` not found"));
                    DisplayFmtEnum::default_display_format()
                }
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
        tex_allocator: &mut TextureAllocator,
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
        tex_allocator: &mut TextureAllocator,
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
        tex_allocator: &mut TextureAllocator,
    ) {
        self._color_box(color, size, ui, tex_allocator, false);
    }

    fn _color_box(
        &mut self,
        color: &Color,
        size: Vec2,
        ui: &mut Ui,
        tex_allocator: &mut TextureAllocator,
        with_label: bool,
    ) {
        let display_str = self.display_color(color);
        let format = self.display_format();
        let on_hover = color_tooltip(
            color,
            format,
            self.settings_window.settings.rgb_working_space,
            self.settings_window.settings.illuminant,
        );
        let color_box = tex_color(
            ui,
            tex_allocator,
            &mut self.texture_manager,
            color.color32(),
            size,
            Some(&on_hover),
        );
        if let Some(color_box) = color_box {
            if with_label {
                ui.monospace(&display_str);
            }

            if color_box.clicked() {
                self.picker.set_cur_color(*color);
            }

            if color_box.middle_clicked() {
                self.add_color(*color);
            }

            if color_box.secondary_clicked() {
                let _ = save_to_clipboard(self.clipboard_color(color));
            }
        }
    }

    fn gradient_box(
        &mut self,
        gradient: &Gradient,
        size: Vec2,
        ui: &mut Ui,
        tex_allocator: &mut TextureAllocator,
    ) {
        let _ = tex_gradient(
            ui,
            tex_allocator,
            &mut self.texture_manager,
            gradient,
            size,
            None,
        );
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        let frame = egui::Frame {
            fill: if ctx.style().visuals.dark_mode {
                *D_BG_00
            } else {
                *L_BG_0
            },
            inner_margin: Margin::symmetric(15., 10.),
            ..Default::default()
        };
        egui::TopBottomPanel::top("top panel")
            .frame(frame)
            .show(ctx, |ui| {
                self.top_ui(ui);
            });
    }

    fn side_panel(&mut self, ctx: &egui::Context, tex_allocator: &mut TextureAllocator) {
        let frame = egui::Frame {
            fill: if ctx.style().visuals.dark_mode {
                *D_BG_00
            } else {
                *L_BG_0
            },
            inner_margin: Margin::symmetric(15., 10.),
            ..Default::default()
        };

        egui::SidePanel::right("colors")
            .frame(frame)
            .resizable(false)
            .max_width(self.side_panel_box_width * 1.2)
            .default_width(self.side_panel_box_width)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    self.side_ui(ui, tex_allocator);
                })
            });
    }

    fn central_panel(&mut self, ctx: &egui::Context, tex_allocator: &mut TextureAllocator) {
        let _frame = egui::Frame {
            fill: if ctx.style().visuals.dark_mode {
                *D_BG_0
            } else {
                *L_BG_2
            },
            inner_margin: Margin::symmetric(10., 5.),
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
            self.set_theme(ui.ctx());
        }
    }

    fn side_panel_palette_picker(&mut self, ui: &mut Ui) -> egui::InnerResponse<()> {
        let current_palette = self.palettes.current().clone();
        let mut selected_palette_name = current_palette.name.clone();
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source("side-panel-palette-chooser")
                .selected_text(&current_palette.name)
                .width(self.side_panel_box_width * 0.69)
                .show_ui(ui, |ui| {
                    for palette in self.palettes.iter() {
                        let _ = ui.selectable_value(
                            &mut selected_palette_name,
                            palette.name.clone(),
                            &palette.name,
                        );
                    }
                });

            if selected_palette_name != current_palette.name {
                self.palettes.move_to_name(&selected_palette_name);
            }
        })
    }

    fn side_panel_button_toolbar(&mut self, ui: &mut Ui) -> egui::InnerResponse<()> {
        ui.horizontal(|ui| {
            if ui
                .button(ADD_ICON)
                .on_hover_text("Add a new palette")
                .clicked()
            {
                self.palettes.append_empty();
                self.palettes.move_to_last();
            }
            if ui
                .button(CLEAR_ICON)
                .on_hover_text("Clear colors")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                self.palettes.current_mut().palette.clear();
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
                let _ = save_to_clipboard(self.palettes.current().palette.as_hex_list());
            }
            #[allow(clippy::collapsible_if)]
            if !self.edit_side_palette_name {
                if ui
                    .button(EDIT_ICON)
                    .on_hover_text("Change palette name")
                    .on_hover_cursor(CursorIcon::Text)
                    .clicked()
                {
                    self.edit_side_palette_name = true;
                }
            }
            if ui
                .button(DELETE_ICON)
                .on_hover_text("Delete current palette")
                .clicked()
            {
                self.palettes.remove_current();
            }
        })
    }

    fn side_panel_palette_name(&mut self, ui: &mut Ui) -> egui::InnerResponse<()> {
        let current_palette = self.palettes.current_mut();
        let mut name_text = if current_palette.name.is_empty() {
            "Current palette".to_string()
        } else {
            current_palette.name.clone()
        };
        const MAX_NAME_LEN: usize = 15;
        const NAME_MULTIPLIER: usize = 10;
        const NAME_MAX_WIDTH: usize = MAX_NAME_LEN * NAME_MULTIPLIER;
        const NAME_MIN_WIDTH: usize = 50;
        if name_text.len() > MAX_NAME_LEN {
            name_text.truncate(MAX_NAME_LEN);
            name_text.push_str("...");
        }
        ui.scope(|ui| {
            if self.edit_side_palette_name {
                let mut edit_name = current_palette.name.clone();
                let width = (edit_name.len() * NAME_MULTIPLIER)
                    .max(NAME_MIN_WIDTH)
                    .min(NAME_MAX_WIDTH) as f32;
                egui::TextEdit::singleline(&mut edit_name)
                    .desired_width(width)
                    .show(ui);
                current_palette.name = edit_name;
                if ui
                    .button(APPLY_ICON)
                    .on_hover_text("Finish editing")
                    .clicked()
                {
                    self.edit_side_palette_name = false;
                }
            } else {
                let heading = Label::new(RichText::new(name_text).heading());
                ui.add(heading);
            }
        })
    }

    fn side_panel_colors_column(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut TextureAllocator,
    ) -> egui::InnerResponse<()> {
        let current_palette = self.palettes.current().clone();
        let mut src_row = None;
        let mut dst_row = None;

        let display_strings: Vec<_> = current_palette
            .palette
            .iter()
            .map(|c| self.display_color(c))
            .collect();
        let max_len = display_strings
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or_default();
        let box_width = (max_len * 11).max((self.side_panel_box_width * 0.8) as usize) as f32;

        let resp = ui.scope(|ui| {
            for (idx, color) in current_palette.palette.iter().enumerate() {
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
                                    self.palettes.current_mut().palette.remove(color);
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
        });

        if let Some(src_row) = src_row {
            if let Some(dst_row) = dst_row {
                if ui.input().pointer.any_released() {
                    let palette = &mut self.palettes.current_mut().palette;
                    if let Some(it) = palette.remove_pos(src_row) {
                        palette.insert(dst_row, it);
                    }
                }
            }
        }

        resp
    }

    fn side_ui(&mut self, ui: &mut Ui, tex_allocator: &mut TextureAllocator) {
        ui.vertical(|ui| {
            self.side_panel_palette_picker(ui);
            ui.add_space(HALF_SPACE);

            let resp = self.side_panel_button_toolbar(ui);
            self.side_panel_box_width = resp.response.rect.width() * 1.3;

            self.side_panel_palette_name(ui);
            ui.add_space(SPACE);
            self.side_panel_colors_column(ui, tex_allocator);
        });
    }

    fn display_windows(&mut self, ctx: &egui::Context, tex_allocator: &mut TextureAllocator) {
        self.settings_window.display(ctx);
        self.settings_window.custom_formats_window.display(
            &mut self.settings_window.settings,
            ctx,
            self.picker.current_color,
        );
        if let Err(e) = self.export_window.display(ctx, &self.palettes) {
            append_global_error(e);
        }

        self.shades_window(ctx, tex_allocator);
        self.tints_window(ctx, tex_allocator);
        self.hues_window(ctx, tex_allocator);
        self.help_window.display(ctx);
    }

    fn ui(&mut self, ui: &mut Ui, tex_allocator: &mut TextureAllocator) {
        let mut top_padding = 0.;
        let mut err_idx = 0;
        self.display_errors.retain(|e| {
            if let Ok(elapsed) = e.timestamp().elapsed() {
                if elapsed >= ERROR_DISPLAY_DURATION {
                    false
                } else {
                    if let Some(rsp) = egui::Window::new("Error")
                        .collapsible(false)
                        .id(Id::new(format!("err_ntf_{err_idx}")))
                        .anchor(
                            egui::Align2::RIGHT_TOP,
                            (-self.side_panel_box_width - 25., top_padding),
                        )
                        .hscroll(true)
                        .fixed_size((self.side_panel_box_width, 50.))
                        .show(ui.ctx(), |ui| {
                            let label = Label::new(RichText::new(e.message()).color(Color32::RED))
                                .wrap(true);
                            ui.add(label);
                        })
                    {
                        top_padding += rsp.response.rect.height() + 6.;
                        err_idx += 1;
                    };
                    true
                }
            } else {
                false
            }
        });
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
                    append_global_error(format!("Failed to save color to clipboard - {}", e));
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
        });
        let c = self.picker.current_color;
        self.color_box_label_side(&c, vec2(25., 25.), ui, tex_allocator);

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

    fn handle_display_picker(&mut self, ui: &mut Ui, tex_allocator: &mut TextureAllocator) {
        if let Some(picker) = self.display_picker.clone() {
            if let Ok(color) = picker.get_color_under_cursor() {
                if ui.ctx().input().key_pressed(egui::Key::P) {
                    self.picker.set_cur_color(color);
                    if self.settings_window.settings.auto_copy_picked_color {
                        let _ = save_to_clipboard(self.clipboard_color(&color));
                    }
                }
                if ui.ctx().input().key_pressed(egui::Key::S) {
                    self.palettes.current_mut().palette.add(color);
                }
                ui.horizontal(|ui| {
                    ui.label("Color at cursor: ");
                    #[cfg(any(windows, target_os = "linux"))]
                    self.zoom_picker_impl(ui, picker);
                });
                self.color_box_label_side(&color, vec2(25., 25.), ui, tex_allocator);
            }
        };
    }

    fn toggle_mouse(&mut self, icon: CursorIcon) {
        self.cursor_icon = if icon == self.cursor_icon {
            CursorIcon::default()
        } else {
            icon
        }
    }

    #[cfg(any(target_os = "linux", windows))]
    fn display_zoom_window(&mut self, picker: &Rc<dyn DisplayPickerExt>) {
        if self.picker_window.is_none() {
            self.toggle_mouse(CursorIcon::Crosshair);
            let cursor_pos = picker.get_cursor_pos().unwrap_or_default();

            #[cfg(target_os = "linux")]
            if let Ok(window) = picker.spawn_window(
                CURSOR_PICKER_WINDOW_NAME,
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i16,
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i16,
                ZOOM_WIN_WIDTH + (ZOOM_WIN_BORDER_WIDTH * 2) as u16,
                ZOOM_WIN_HEIGHT + (ZOOM_WIN_BORDER_WIDTH * 2) as u16,
                picker.screen_num(),
                display_picker::x11::WindowType::Notification,
            ) {
                self.picker_window = Some(window);
            }

            #[cfg(windows)]
            if let Ok(window) = picker.spawn_window(
                "EPICK_DIALOG",
                CURSOR_PICKER_WINDOW_NAME,
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET),
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET),
                ZOOM_WIN_WIDTH as i32,
                ZOOM_WIN_HEIGHT as i32,
                WS_POPUP | WS_BORDER,
            ) {
                self.picker_window = Some(window);
                if let Err(e) = picker.show_window(window, SW_SHOWDEFAULT) {
                    append_global_error(e);
                }
            }
        }
    }

    #[cfg(any(target_os = "linux", windows))]
    fn hide_zoom_window(&mut self, picker: &Rc<dyn DisplayPickerExt>) {
        if let Some(picker_window) = self.picker_window {
            #[cfg(target_os = "linux")]
            let _ = picker.destroy_window(picker_window.0);

            #[cfg(windows)]
            if let Err(e) = picker.destroy_window(picker_window) {
                append_global_error(e);
            }

            self.picker_window = None;
        }
    }

    #[cfg(target_os = "linux")]
    fn handle_zoom_picker(&mut self, _ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        if let Some((window, gc)) = self.picker_window {
            let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
            if let Ok(img) = picker.get_image(
                picker.screen().root,
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i16,
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i16,
                ZOOM_IMAGE_WIDTH,
                ZOOM_IMAGE_HEIGHT,
            ) {
                use image::Pixel;
                let white = image::Rgba::from_slice(&[255, 255, 255, 255]);
                let black = image::Rgba::from_slice(&[0, 0, 0, 255]);
                let img = display_picker::x11::resize_image(&img, ZOOM_SCALE);
                let img = display_picker::x11::add_border(&img, white, ZOOM_WIN_BORDER_WIDTH / 2)
                    .unwrap();
                let img = display_picker::x11::add_border(&img, black, ZOOM_WIN_BORDER_WIDTH / 2)
                    .unwrap();

                if let Err(e) = img.put(picker.conn(), window, gc, 0, 0) {
                    append_global_error(e);
                    return;
                };

                if let Err(e) = picker.draw_circle(
                    window,
                    gc,
                    (ZOOM_WIN_WIDTH / 2) as i16,
                    (ZOOM_WIN_HEIGHT / 2) as i16,
                    ZOOM_WIN_POINTER_DIAMETER,
                ) {
                    append_global_error(e);
                };
            }
            if let Err(e) = picker.update_window_pos(
                window,
                cursor_pos.0 + ZOOM_WIN_OFFSET,
                cursor_pos.1 + ZOOM_WIN_OFFSET,
            ) {
                append_global_error(e);
                return;
            }
            if let Err(e) = picker.flush() {
                append_global_error(e);
            }
        }
    }

    #[cfg(windows)]
    fn handle_zoom_picker(&mut self, _ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        if let Some(window) = self.picker_window {
            let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
            match picker.get_screenshot(
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i32,
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i32,
                (ZOOM_WIN_WIDTH as f32 / ZOOM_SCALE) as i32,
                (ZOOM_WIN_HEIGHT as f32 / ZOOM_SCALE) as i32,
            ) {
                Ok(bitmap) => {
                    if let Err(e) = picker.render_bitmap(&bitmap, window, 0, 0, ZOOM_SCALE) {
                        append_global_error(e);
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
                        append_global_error(e);
                    }
                }
                Err(e) => {
                    append_global_error(e);
                }
            }
            if let Err(e) = picker.move_window(
                window,
                (cursor_pos.0 + ZOOM_WIN_OFFSET) as i32,
                (cursor_pos.1 + ZOOM_WIN_OFFSET) as i32,
                ZOOM_WIN_WIDTH as i32,
                ZOOM_WIN_HEIGHT as i32,
            ) {
                append_global_error(e);
            }
        }
    }

    #[cfg(any(target_os = "linux", windows))]
    fn zoom_picker_impl(&mut self, ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        let btn = Button::new(ZOOM_PICKER_ICON).sense(egui::Sense::drag());
        let btn = ui
            .add(btn)
            .on_hover_cursor(CursorIcon::ZoomIn)
            .on_hover_text("Drag to enable zoomed window");

        if btn.dragged() {
            self.zoom_window_dragged = true;
            self.display_zoom_window(&picker);
        }
        if !btn.dragged() && !btn.has_focus() {
            self.hide_zoom_window(&picker);
            self.zoom_window_dragged = false;
        }

        self.handle_zoom_picker(ui, picker);
    }

    #[cfg(not(any(target_os = "linux", windows)))]
    fn zoom_picker_impl(&mut self, _: &mut Ui, _: Rc<dyn DisplayPickerExt>) {}
}
