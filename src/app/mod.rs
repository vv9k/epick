#![allow(dead_code)]
mod context;
mod keybinding;
mod scheme;
mod settings;
mod sidepanel;
pub mod windows;

use crate::color::Palettes;
use crate::color::{Color, ColorHarmony, DisplayFormat, Gradient};
use crate::color_picker::ColorPicker;
use crate::display_picker::{self, DisplayPickerExt};
use crate::error::{append_global_error, DisplayError, ERROR_STACK};
use crate::render::{render_color, render_gradient, TextureManager};
use crate::save_to_clipboard;
use crate::screen_size::ScreenSize;
use crate::ui::{
    color_tooltip,
    colorbox::{ColorBox, COLORBOX_DRAG_TOOLTIP, COLORBOX_PICK_TOOLTIP},
    colors::*,
    dark_visuals, drag_source, drop_target, icon, light_visuals, DOUBLE_SPACE, SPACE,
};
use context::{AppCtx, FrameCtx};
use keybinding::{default_keybindings, KeyBindings};
use settings::{DisplayFmtEnum, Settings};
use windows::{ExportWindow, HelpWindow, HuesWindow, SettingsWindow, ShadesWindow, TintsWindow};

use eframe::{CreationContext, Storage};
use egui::{
    color::Color32, style::Margin, vec2, Button, CollapsingHeader, CursorIcon, Id, Label, Layout,
    Rgba, RichText, ScrollArea, Ui, Vec2, Visuals,
};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::RwLock;
use std::time::Duration;

#[cfg(target_os = "linux")]
use x11rb::protocol::xproto;

#[cfg(windows)]
use crate::display_picker::windows::{HWND, SW_SHOWDEFAULT, WS_BORDER, WS_POPUP};

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

lazy_static::lazy_static! {
    pub static ref KEYBINDINGS: KeyBindings = default_keybindings();
    pub static ref LIGHT_VISUALS: Visuals = light_visuals();
    pub static ref DARK_VISUALS: Visuals = dark_visuals();
}

static CONTEXT: OnceCell<RwLock<AppCtx>> = OnceCell::new();

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum CentralPanelTab {
    Picker,
    Palettes,
}

pub fn load_settings(_storage: Option<&dyn eframe::Storage>) -> Option<Settings> {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = _storage {
        if let Some(yaml) = storage.get_string(Settings::STORAGE_KEY) {
            if let Ok(settings) = Settings::from_yaml_str(&yaml) {
                return Some(settings);
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(config_dir) = Settings::dir("epick") {
        let path = config_dir.join(Settings::FILE_NAME);

        if let Ok(settings) = Settings::load(&path) {
            return Some(settings);
        }
    }

    None
}

pub fn save_settings(settings: &Settings, _storage: &mut dyn Storage) {
    #[cfg(target_arch = "wasm32")]
    if let Ok(yaml) = settings.as_yaml_str() {
        _storage.set_string(Settings::STORAGE_KEY, yaml);
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(dir) = Settings::dir("epick") {
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        let _ = settings.save(dir.join(Settings::FILE_NAME));
    }
}

pub struct App {
    pub picker: ColorPicker,
    pub texture_manager: TextureManager,
    pub display_picker: Option<Rc<dyn DisplayPickerExt>>,
    pub error_message: Option<String>,

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

    pub display_errors: Vec<DisplayError>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if let Some(mut app_ctx) = CONTEXT.get().and_then(|ctx| ctx.write().ok()) {
            let mut ctx = FrameCtx::new(&mut app_ctx, ctx);
            ctx.egui.output().cursor_icon = ctx.app.cursor_icon;

            let screen_size = ScreenSize::from(ctx.egui.available_rect());
            if ctx.app.screen_size != screen_size {
                self.set_styles(&mut ctx, screen_size);
            }

            self.check_settings_change(&mut ctx);

            self.top_panel(&mut ctx);

            self.central_panel(&mut ctx);

            if ctx.app.sidepanel.show {
                self.side_panel(&mut ctx);
            }

            self.display_windows(&mut ctx);

            frame.set_window_size(ctx.egui.used_size());

            self.picker.check_for_change();

            // populate display errors from the global error stack
            if let Ok(mut stack) = ERROR_STACK.try_lock() {
                while let Some(error) = stack.errors.pop_front() {
                    self.display_errors.push(error);
                }
            }

            if ctx.egui.memory().focus().is_none() {
                self.check_keys_pressed(&mut ctx);
            }

            // No need to repaint in wasm, there is no way to pick color from under the cursor anyway
            #[cfg(not(target_arch = "wasm32"))]
            if !ctx.egui.is_pointer_over_area() {
                // This paint request makes sure that the color displayed as color under cursor
                // gets updated even when the pointer is not in the egui window area.
                ctx.egui.request_repaint();

                if ctx.app.zoom_window_dragged {
                    // When zooming we want to continually repaint for smooth experience
                    // even if the pointer is not over main window area
                    return;
                }

                // Otherwise sleep to save some cycles
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            ctx.app.current_selected_color = self.picker.current_color;
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if let Some(ctx) = CONTEXT.get().and_then(|ctx| ctx.read().ok()) {
            self.save_colors(&ctx, storage);
            save_settings(&ctx.settings, storage);
        }
        storage.flush();
    }

    fn max_size_points(&self) -> egui::Vec2 {
        vec2(4096., 8192.)
    }
}

impl App {
    pub fn init(context: &CreationContext) -> Box<dyn eframe::App + 'static> {
        let mut app_ctx = AppCtx::new(context);

        let app = Box::new(Self {
            picker: ColorPicker::default(),
            texture_manager: TextureManager::default(),
            display_picker: crate::display_picker::init_display_picker(),
            error_message: None,

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
        });

        let prefer_dark = context.integration_info.prefer_dark_mode.unwrap_or(true);

        let mut ctx = FrameCtx::new(&mut app_ctx, &context.egui_ctx);

        app.load_colors(&mut ctx, context.storage);

        if prefer_dark {
            app.set_dark_theme(&mut ctx);
        } else {
            app.set_light_theme(&mut ctx);
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

        CONTEXT.try_insert(RwLock::new(app_ctx)).unwrap();

        app
    }

    fn set_dark_theme(&self, ctx: &mut FrameCtx<'_>) {
        ctx.app.settings.is_dark_mode = true;
        ctx.egui.set_visuals(DARK_VISUALS.clone());
    }

    fn set_light_theme(&self, ctx: &mut FrameCtx<'_>) {
        ctx.app.settings.is_dark_mode = false;
        ctx.egui.set_visuals(LIGHT_VISUALS.clone());
    }

    fn set_theme(&self, ctx: &mut FrameCtx<'_>) {
        if ctx.is_dark_mode() {
            self.set_light_theme(ctx);
        } else {
            self.set_dark_theme(ctx);
        }
    }

    fn set_styles(&self, ctx: &mut FrameCtx<'_>, screen_size: ScreenSize) {
        ctx.app.screen_size = screen_size;

        let slider_size = match screen_size {
            ScreenSize::Phone(w, _) => w * 0.5,
            ScreenSize::Desktop(w, _) if w > 1500. => w * 0.2,
            ScreenSize::Tablet(w, _) | ScreenSize::Laptop(w, _) | ScreenSize::Desktop(w, _) => {
                w * 0.35
            }
        };

        let mut style = (*ctx.egui.style()).clone();
        style.spacing.slider_width = slider_size / 2.;
        ctx.egui.set_style(style);
    }

    fn load_colors(&self, ctx: &mut FrameCtx, _storage: Option<&dyn Storage>) {
        if ctx.app.settings.cache_colors {
            #[cfg(target_arch = "wasm32")]
            if let Some(storage) = _storage {
                if let Some(yaml) = storage.get_string(Palettes::STORAGE_KEY) {
                    if let Ok(palettes) = Palettes::from_yaml_str(&yaml) {
                        ctx.app.palettes = palettes;
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(path) = Palettes::dir("epick") {
                if let Ok(palettes) = Palettes::load(path.join(Palettes::FILE_NAME)) {
                    ctx.app.palettes = palettes;
                }
            }
        }
    }

    fn save_colors(&self, ctx: &AppCtx, _storage: &mut dyn Storage) {
        #[cfg(target_arch = "wasm32")]
        if ctx.settings.cache_colors {
            if let Ok(yaml) = ctx.palettes.as_yaml_str() {
                _storage.set_string(Palettes::STORAGE_KEY, yaml);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(dir) = Palettes::dir("epick") {
            if !dir.exists() {
                let _ = std::fs::create_dir_all(&dir);
            }
            let _ = ctx.palettes.save(dir.join(Palettes::FILE_NAME));
        }
    }

    fn check_settings_change(&mut self, ctx: &mut FrameCtx<'_>) {
        if ctx.app.settings.chromatic_adaptation_method
            != self.picker.sliders.chromatic_adaptation_method
        {
            self.picker.sliders.chromatic_adaptation_method =
                ctx.app.settings.chromatic_adaptation_method;
        }
        if ctx.app.settings.rgb_working_space != self.picker.sliders.rgb_working_space {
            self.picker.new_workspace = Some(ctx.app.settings.rgb_working_space);
        }
        if ctx.app.settings.illuminant != self.picker.sliders.illuminant {
            self.picker.new_illuminant = Some(ctx.app.settings.illuminant);
        }
    }

    fn check_keys_pressed(&mut self, ctx: &mut FrameCtx) {
        for kb in KEYBINDINGS.iter() {
            if ctx.egui.input().key_pressed(kb.key()) {
                let f = kb.binding();
                f(self, ctx)
            }
        }
    }

    fn add_color(&self, ctx: &mut FrameCtx<'_>, color: Color) {
        if !ctx.app.palettes.current_mut().palette.add(color) {
            let color_str = self.display_color(ctx, &color);
            append_global_error(format!("Color {} already saved!", color_str));
        } else {
            ctx.app.sidepanel.show = true;
        }
    }

    fn add_cur_color(&self, ctx: &mut FrameCtx<'_>) {
        self.add_color(ctx, self.picker.current_color)
    }

    fn hex_input(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        CollapsingHeader::new("Text input").show(ui, |ui| {
            ui.label("Enter a hex color: ");
            ui.horizontal(|ui| {
                let resp = ui.text_edit_singleline(&mut self.picker.hex_color);
                if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                    || ui
                        .button(icon::PLAY)
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
                    .button(icon::ADD)
                    .on_hover_text(ADD_DESCR)
                    .on_hover_cursor(CursorIcon::Copy)
                    .clicked()
                {
                    self.add_cur_color(ctx)
                }
            });
        });
    }

    fn display_format<'fmt>(&self, ctx: &'fmt FrameCtx<'_>) -> DisplayFormat<'fmt> {
        match ctx.app.settings.color_display_format {
            DisplayFmtEnum::Hex => DisplayFormat::Hex,
            DisplayFmtEnum::HexUppercase => DisplayFormat::HexUpercase,
            DisplayFmtEnum::CssRgb => DisplayFormat::CssRgb,
            DisplayFmtEnum::CssHsl => DisplayFormat::CssHsl {
                degree_symbol: true,
            },
            DisplayFmtEnum::Custom(ref name) => {
                if ctx.app.settings.saved_color_formats.get(name).is_some() {
                    DisplayFormat::Custom(&ctx.app.settings.saved_color_formats[name])
                } else {
                    append_global_error(format!("Custom color format `{name}` not found"));
                    DisplayFmtEnum::default_display_format()
                }
            }
        }
    }

    fn display_color(&self, ctx: &mut FrameCtx<'_>, color: &Color) -> String {
        color.display(
            self.display_format(ctx),
            ctx.app.settings.rgb_working_space,
            ctx.app.settings.illuminant,
        )
    }

    fn clipboard_color(&self, ctx: &mut FrameCtx<'_>, color: &Color) -> String {
        let format = match ctx
            .app
            .settings
            .color_clipboard_format
            .as_ref()
            .unwrap_or(&ctx.app.settings.color_display_format)
        {
            DisplayFmtEnum::Hex => DisplayFormat::Hex,
            DisplayFmtEnum::HexUppercase => DisplayFormat::HexUpercase,
            DisplayFmtEnum::CssRgb => DisplayFormat::CssRgb,
            DisplayFmtEnum::CssHsl => DisplayFormat::CssHsl {
                degree_symbol: false,
            },
            DisplayFmtEnum::Custom(name) => {
                if ctx.app.settings.saved_color_formats.get(name).is_some() {
                    DisplayFormat::Custom(&ctx.app.settings.saved_color_formats[name])
                } else {
                    append_global_error(format!("Custom color format `{name}` not found"));
                    DisplayFmtEnum::default_display_format()
                }
            }
        };
        color.display(
            format,
            ctx.app.settings.rgb_working_space,
            ctx.app.settings.illuminant,
        )
    }

    fn display_color_box(&mut self, color_box: ColorBox, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        let color = color_box.color();
        let display_str = self.display_color(ctx, &color);
        let format = self.display_format(ctx);
        let on_hover = color_tooltip(
            &color,
            format,
            ctx.app.settings.rgb_working_space,
            ctx.app.settings.illuminant,
            color_box.hover_help(),
        );
        let tex_allocator = &mut ctx.tex_allocator();
        let resp = render_color(
            ui,
            tex_allocator,
            &mut self.texture_manager,
            color_box.color().color32(),
            color_box.size(),
            Some(&on_hover),
            color_box.border(),
        );
        if let Some(resp) = resp {
            if color_box.label() {
                ui.monospace(&display_str);
            }

            if resp.clicked() {
                self.picker.set_cur_color(color);
            }

            if resp.middle_clicked() {
                self.add_color(ctx, color);
            }

            if resp.secondary_clicked() {
                let _ = save_to_clipboard(self.clipboard_color(ctx, &color));
            }
        }
    }

    fn gradient_box(
        &mut self,
        ctx: &mut FrameCtx,
        gradient: &Gradient,
        size: Vec2,
        ui: &mut Ui,
        border: bool,
    ) {
        let tex_allocator = &mut ctx.tex_allocator();
        let _ = render_gradient(
            ui,
            tex_allocator,
            &mut self.texture_manager,
            gradient,
            size,
            None,
            border,
        );
    }

    fn top_panel(&mut self, ctx: &mut FrameCtx<'_>) {
        let frame = egui::Frame {
            fill: if ctx.egui.style().visuals.dark_mode {
                *D_BG_00
            } else {
                *L_BG_0
            },
            inner_margin: Margin::symmetric(15., 10.),
            ..Default::default()
        };
        egui::TopBottomPanel::top("top panel")
            .frame(frame)
            .show(ctx.egui, |ui| {
                self.top_ui(ctx, ui);
            });
    }

    fn top_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            macro_rules! add_button_if {
                ($text:expr, $condition:expr, $block:tt) => {
                    add_button_if!($text, $condition, $block, $block);
                };
                ($text:expr, $condition:expr, $block_a:tt, $block_b:tt) => {
                    if $condition {
                        if ui
                            .button($text)
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        $block_a;
                    } else {
                        let btn = Button::new($text).fill(Rgba::from_black_alpha(0.));
                        if ui
                            .add(btn)
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        $block_b;
                    }
                };
            }
            add_button_if!(
                "picker",
                matches!(ctx.app.central_panel_tab, CentralPanelTab::Picker),
                {
                    ctx.app.central_panel_tab = CentralPanelTab::Picker;
                }
            );
            add_button_if!(
                "palettes",
                matches!(ctx.app.central_panel_tab, CentralPanelTab::Palettes),
                {
                    ctx.app.central_panel_tab = CentralPanelTab::Palettes;
                    ctx.app.sidepanel.show = false;
                }
            );

            ui.add_space(DOUBLE_SPACE);

            add_button_if!(
                "hues",
                self.hues_window.is_open,
                { self.hues_window.is_open = false },
                { self.hues_window.is_open = true }
            );
            add_button_if!(
                "shades",
                self.shades_window.is_open,
                { self.shades_window.is_open = false },
                { self.shades_window.is_open = true }
            );
            add_button_if!(
                "tints",
                self.tints_window.is_open,
                { self.tints_window.is_open = false },
                { self.tints_window.is_open = true }
            );

            ui.with_layout(Layout::right_to_left(), |ui| {
                if ui
                    .button(icon::HELP)
                    .on_hover_text("Show help")
                    .on_hover_cursor(CursorIcon::Help)
                    .clicked()
                {
                    self.help_window.toggle_window();
                }
                if ui
                    .button(icon::EXPAND)
                    .on_hover_text("Show/hide side panel")
                    .on_hover_cursor(CursorIcon::ResizeHorizontal)
                    .clicked()
                {
                    ctx.app.sidepanel.show = !ctx.app.sidepanel.show;
                }
                if ui
                    .button(icon::SETTINGS)
                    .on_hover_text("Settings")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    self.settings_window.show = true;
                }
                self.dark_light_switch(ctx, ui);
            });
        });
    }

    fn dark_light_switch(&mut self, ctx: &mut FrameCtx, ui: &mut Ui) {
        let btn = if ctx.is_dark_mode() {
            icon::LIGHT_MODE
        } else {
            icon::DARK_MODE
        };
        if ui
            .button(btn)
            .on_hover_text("Switch ui color theme")
            .on_hover_cursor(CursorIcon::PointingHand)
            .clicked()
        {
            self.set_theme(ctx);
        }
    }

    fn display_windows(&mut self, ctx: &mut FrameCtx<'_>) {
        self.settings_window.display(ctx.app, ctx.egui);
        self.settings_window.custom_formats_window.display(
            &mut ctx.app.settings,
            ctx.egui,
            self.picker.current_color,
        );
        if let Err(e) = self.export_window.display(ctx.egui) {
            append_global_error(e);
        }

        self.shades_window(ctx);
        self.tints_window(ctx);
        self.hues_window(ctx);
        self.help_window.display(ctx.egui);
    }

    fn central_panel(&mut self, ctx: &mut FrameCtx<'_>) {
        let _frame = egui::Frame {
            fill: if ctx.egui.style().visuals.dark_mode {
                *D_BG_0
            } else {
                *L_BG_2
            },
            inner_margin: Margin::symmetric(10., 5.),
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(_frame)
            .show(ctx.egui, |ui| match ctx.app.central_panel_tab {
                CentralPanelTab::Picker => self.picker_ui(ctx, ui),
                CentralPanelTab::Palettes => self.palettes_ui(ctx, ui),
            });
    }

    fn palettes_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ScrollArea::new([true, true]).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut ctx.app.palettes_tab_display_label,
                    "Display color labels",
                );
            });

            for palette in ctx
                .app
                .palettes
                .clone()
                .iter()
                .filter(|p| !p.palette.is_empty())
            {
                if palette.palette.is_empty() {
                    continue;
                }
                let label = RichText::new(&palette.name).heading();
                ui.horizontal(|ui| {
                    ui.add(Label::new(label));
                    if ui
                        .button(icon::EXPORT)
                        .on_hover_text("Export")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.export_window.show = true;
                        self.export_window.export_palette = Some(palette.clone());
                    }
                    if ui
                        .button(icon::COPY)
                        .on_hover_text("Copy all colors to clipboard")
                        .on_hover_cursor(CursorIcon::Alias)
                        .clicked()
                    {
                        let _ = save_to_clipboard(palette.palette.as_hex_list());
                    }
                    if ui
                        .button(icon::DELETE)
                        .on_hover_text("Delete this palette")
                        .clicked()
                    {
                        ctx.app.palettes.remove(palette);
                    }
                });
                egui::Grid::new(&palette.name)
                    .spacing((2.5, 0.))
                    .show(ui, |ui| {
                        let mut src_row = None;
                        let mut dst_row = None;
                        for (i, color) in palette.palette.iter().enumerate() {
                            let resp = drop_target(ui, true, |ui| {
                                let color_id = Id::new(&palette.name).with(i);
                                drag_source(ui, color_id, |ui| {
                                    let cb = ColorBox::builder()
                                        .size((50., 50.))
                                        .color(*color)
                                        .label(ctx.app.palettes_tab_display_label)
                                        .hover_help(COLORBOX_DRAG_TOOLTIP)
                                        .build();
                                    ui.vertical(|ui| {
                                        self.display_color_box(cb, ctx, ui);
                                    });
                                });
                                if ui.memory().is_being_dragged(color_id) {
                                    src_row = Some(i);
                                }
                            });
                            let is_being_dragged = ui.memory().is_anything_being_dragged();
                            if is_being_dragged && resp.response.hovered() {
                                dst_row = Some(i);
                            }
                        }
                        if let Some(src_row) = src_row {
                            if let Some(dst_row) = dst_row {
                                if ui.input().pointer.any_released() {
                                    ctx.app.palettes.move_to_name(&palette.name);
                                    let palette = &mut ctx.app.palettes.current_mut().palette;
                                    if let Some(it) = palette.remove_pos(src_row) {
                                        palette.insert(dst_row, it);
                                    }
                                }
                            }
                        }
                    });
                ui.add_space(SPACE);
            }
        });
    }

    fn picker_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
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
                            (-ctx.app.sidepanel.box_width - 25., top_padding),
                        )
                        .hscroll(true)
                        .fixed_size((ctx.app.sidepanel.box_width, 50.))
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
                .button(icon::COPY)
                .on_hover_text("Copy color to clipboard")
                .on_hover_cursor(CursorIcon::Alias)
                .clicked()
            {
                if let Err(e) =
                    save_to_clipboard(self.clipboard_color(ctx, &self.picker.current_color))
                {
                    append_global_error(format!("Failed to save color to clipboard - {}", e));
                }
            }
            if ui
                .button(icon::ADD)
                .on_hover_text(ADD_DESCR)
                .on_hover_cursor(CursorIcon::Copy)
                .clicked()
            {
                self.add_cur_color(ctx);
            }
        });
        let cb = ColorBox::builder()
            .size((25., 25.))
            .color(self.picker.current_color)
            .label(true)
            .hover_help(COLORBOX_PICK_TOOLTIP)
            .border(true)
            .build();
        ui.horizontal(|ui| {
            self.display_color_box(cb, ctx, ui);
        });

        self.handle_display_picker(ctx, ui);

        ui.add_space(SPACE);
        ScrollArea::vertical()
            .id_source("picker scroll")
            .show(ui, |ui| {
                self.harmonies(ctx, ui);
                self.sliders(ctx, ui);
                self.hex_input(ctx, ui);
            });
    }

    fn sliders(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            if ctx.app.settings.color_spaces.rgb {
                self.picker.rgb_sliders(ui);
            }
            if ctx.app.settings.color_spaces.cmyk {
                self.picker.cmyk_sliders(ui);
            }
            if ctx.app.settings.color_spaces.hsv {
                self.picker.hsv_sliders(ui);
            }
            if ctx.app.settings.color_spaces.hsl {
                self.picker.hsl_sliders(ui);
            }
            if ctx.app.settings.color_spaces.luv {
                self.picker.luv_sliders(ui);
            }
            if ctx.app.settings.color_spaces.lch_uv {
                self.picker.lch_uv_sliders(ui);
            }
            if ctx.app.settings.color_spaces.lab {
                self.picker.lab_sliders(ui);
            }
            if ctx.app.settings.color_spaces.lch_ab {
                self.picker.lch_ab_sliders(ui);
            }
        });
    }

    fn handle_display_picker(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        if let Some(picker) = self.display_picker.clone() {
            if let Ok(color) = picker.get_color_under_cursor() {
                ctx.app.cursor_pick_color = color;
                ui.horizontal(|ui| {
                    ui.label("Color at cursor: ");
                    #[cfg(any(windows, target_os = "linux"))]
                    self.zoom_picker_impl(ctx, ui, picker);
                });
                let cb = ColorBox::builder()
                    .size((25., 25.))
                    .color(color)
                    .label(true)
                    .hover_help(COLORBOX_PICK_TOOLTIP)
                    .border(true)
                    .build();
                ui.horizontal(|ui| {
                    self.display_color_box(cb, ctx, ui);
                });
            }
        };
    }

    fn toggle_mouse(&mut self, ctx: &mut FrameCtx<'_>, icon: CursorIcon) {
        ctx.app.cursor_icon = if icon == ctx.app.cursor_icon {
            CursorIcon::default()
        } else {
            icon
        }
    }

    #[cfg(any(target_os = "linux", windows))]
    fn display_zoom_window(&mut self, ctx: &mut FrameCtx<'_>, picker: &Rc<dyn DisplayPickerExt>) {
        if self.picker_window.is_none() {
            self.toggle_mouse(ctx, CursorIcon::Crosshair);
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
    fn zoom_picker_impl(
        &mut self,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
        picker: Rc<dyn DisplayPickerExt>,
    ) {
        let btn = Button::new(icon::ZOOM_PICKER).sense(egui::Sense::drag());
        let btn = ui
            .add(btn)
            .on_hover_cursor(CursorIcon::ZoomIn)
            .on_hover_text("Drag to enable zoomed window");

        if btn.dragged() {
            ctx.app.zoom_window_dragged = true;
            self.display_zoom_window(ctx, &picker);
        }
        if !btn.dragged() && !btn.has_focus() {
            self.hide_zoom_window(&picker);
            ctx.app.zoom_window_dragged = false;
        }

        self.handle_zoom_picker(ui, picker);
    }

    #[cfg(not(any(target_os = "linux", windows)))]
    fn zoom_picker_impl(&mut self, _: &mut Ui, _: Rc<dyn DisplayPickerExt>) {}
}
