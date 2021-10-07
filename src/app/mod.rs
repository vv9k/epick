#![allow(dead_code)]
mod color_picker;
mod render;
mod saved_colors;
mod scheme;
mod sliders;
mod ui;
mod windows;

use self::windows::{ExportWindow, HuesWindow, SettingsWindow, ShadesWindow, TintsWindow};
use crate::color::{Color, SchemeType};
use crate::picker::{self, DisplayPickerExt};
use crate::save_to_clipboard;
use color_picker::ColorPicker;
use render::{tex_color, TextureManager};
use saved_colors::SavedColors;
use ui::{color_tooltip, colors::*, dark_visuals, drag_source, drop_target, light_visuals};

use egui::{color::Color32, vec2, Ui};
use egui::{Id, ScrollArea, Vec2, Visuals};
use std::borrow::Cow;
use std::rc::Rc;

#[cfg(unix)]
use x11rb::protocol::xproto;

#[cfg(windows)]
use crate::picker::windows::{HWND, SW_SHOWDEFAULT, WS_BORDER, WS_POPUP};

static ADD_ICON: &str = "➕";
static COPY_ICON: &str = "📋";
static ZOOM_PICKER_ICON: &str = "💉";
static SETTINGS_ICON: &str = "⚙";
static EXPAND_ICON: &str = "↔";
static EXPORT_ICON: &str = "🖹";
static CLEAR_ICON: &str = "🗑";
static DELETE_ICON: &str = "❌";
static PLAY_ICON: &str = "▶";
static DARK_MODE_ICON: &str = "🌙";
static LIGHT_MODE_ICON: &str = "☀";

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TopMenuTab {
    Hues,
    Shades,
    Tints,
    NoTab,
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
    #[cfg(windows)]
    pub picker_window: Option<HWND>,
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
            #[cfg(any(unix, windows))]
            let sleep_duration = if self.picker_window.is_some() {
                // Quicker repaints so that the zoomed window doesn't lag behind
                SLEEP_DURATION / 4
            } else {
                SLEEP_DURATION
            };
            #[cfg(not(any(unix, windows)))]
            let sleep_duration = SLEEP_DURATION;

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
            #[cfg(windows)]
            picker_window: None,
        }
    }
}

impl App {
    fn add_color(&mut self, color: Color) {
        if !self.saved_colors.add(color) {
            let color_str = self.display_color(&color);
            self.error_message = Some(format!("Color {} already saved!", color_str));
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
                    || ui
                        .button(PLAY_ICON)
                        .on_hover_text("Use this color")
                        .clicked()
                {
                    if self.picker.hex_color.len() < 6 {
                        self.error_message =
                            Some("Enter a color first (ex. ab12ff #1200ff)".to_owned());
                    } else if let Some(color) =
                        Color::from_hex(self.picker.hex_color.trim_start_matches('#'))
                    {
                        self.picker.set_cur_color(color);
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

    fn display_color(&self, color: &Color) -> String {
        color.display(self.settings_window.color_display_format)
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
        let color_box = tex_color(
            ui,
            texture_allocator,
            &mut self.texture_manager,
            color.color32(),
            size,
            Some(&color_tooltip(
                color,
                self.settings_window.color_display_format,
            )),
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
            if ui.button(SETTINGS_ICON).on_hover_text("Settings").clicked() {
                self.settings_window.show = true;
            }
            if ui
                .button(EXPAND_ICON)
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

    fn dark_light_switch(&mut self, ui: &mut Ui) {
        let is_dark = ui.style().visuals.dark_mode;
        let btn = if is_dark {
            LIGHT_MODE_ICON
        } else {
            DARK_MODE_ICON
        };

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
                if ui
                    .button(CLEAR_ICON)
                    .on_hover_text("Clear colors")
                    .clicked()
                {
                    self.saved_colors.clear();
                }
                if ui.button(EXPORT_ICON).on_hover_text("Export").clicked() {
                    self.export_window.show = true;
                }
                if ui
                    .button(COPY_ICON)
                    .on_hover_text("Copy all colors to clipboard")
                    .clicked()
                {
                    let _ = save_to_clipboard(self.saved_colors.as_hex_list());
                }
            });

            let mut src_row = None;
            let mut dst_row = None;

            for (idx, (_, color)) in self.saved_colors.as_ref().to_vec().iter().enumerate() {
                let resp = drop_target(ui, true, |ui| {
                    let color_id = Id::new("side-color").with(idx);
                    let color_str = self.display_color(color);
                    ui.vertical(|mut ui| {
                        let fst = ui.horizontal(|ui| {
                            ui.monospace(&color_str);
                            if ui
                                .button(DELETE_ICON)
                                .on_hover_text("Delete this color")
                                .clicked()
                            {
                                self.saved_colors.remove(color);
                            }
                            if ui.button(COPY_ICON).on_hover_text("Copy color").clicked() {
                                let _ = save_to_clipboard(color_str.clone());
                            }
                            if ui
                                .button(PLAY_ICON)
                                .on_hover_text("Use this color")
                                .clicked()
                            {
                                self.picker.set_cur_color(*color);
                            }
                        });
                        let help = format!(
                            "{}\n\nDrag and drop to change the order of colors",
                            color_str
                        );

                        let w = fst.response.rect.width();
                        let size = vec2(w, w / 2.);
                        drag_source(&mut ui, color_id, |ui| {
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
        self.settings_window.display(ctx);
        if let Err(e) = self.export_window.display(ctx, &self.saved_colors) {
            self.error_message = Some(e.to_string());
        }

        let color_str = self.display_color(&self.picker.current_color);

        ui.horizontal(|ui| {
            ui.label("Current color: ");
            ui.monospace(&color_str);
            if ui
                .button(COPY_ICON)
                .on_hover_text("Copy color to clipboard")
                .clicked()
            {
                if let Err(e) = save_to_clipboard(color_str.clone()) {
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

        self.picker.check_color_change();
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

    fn sliders(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            if self.settings_window.colorspaces.rgb {
                self.picker.rgb_sliders(ui);
            }
            if self.settings_window.colorspaces.cmyk {
                self.picker.cmyk_sliders(ui);
            }
            if self.settings_window.colorspaces.hsv {
                self.picker.hsv_sliders(ui);
            }
            if self.settings_window.colorspaces.hsl {
                self.picker.hsl_sliders(ui);
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
                ui.horizontal(|mut ui| {
                    ui.label("Color at cursor: ");
                    self.color_box_label_side(&color, vec2(25., 25.), &mut ui, tex_allocator);
                    #[cfg(unix)]
                    self.handle_zoom_picker(ui, picker);
                    #[cfg(windows)]
                    self.handle_zoom_picker(ui, picker);
                });
            }
        };
    }

    #[cfg(unix)]
    fn handle_zoom_picker(&mut self, ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
        if ui.button(ZOOM_PICKER_ICON).clicked() {
            if self.picker_window.is_none() {
                if let Ok(window) = picker.spawn_window(
                    CURSOR_PICKER_WINDOW_NAME,
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
    }

    #[cfg(windows)]
    fn handle_zoom_picker(&mut self, ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
        if ui.button(ZOOM_PICKER_ICON).clicked() {
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
                        self.error_message = Some(e.to_string());
                    }
                }
            } else {
                // Close the window on second click
                if let Err(e) = picker.destroy_window(self.picker_window.unwrap()) {
                    self.error_message = Some(e.to_string());
                }
                self.picker_window = None;
            }
        } else if let Some(window) = self.picker_window {
            match picker.get_screenshot(
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i32,
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i32,
                (ZOOM_WIN_WIDTH as f32 / ZOOM_SCALE) as i32,
                (ZOOM_WIN_HEIGHT as f32 / ZOOM_SCALE) as i32,
            ) {
                Ok(bitmap) => {
                    if let Err(e) = picker.render_bitmap(&bitmap, window, 0, 0, ZOOM_SCALE) {
                        self.error_message = Some(e.to_string());
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
                        self.error_message = Some(e.to_string());
                    }
                }
                Err(e) => {
                    self.error_message = Some(e.to_string());
                }
            }
            if let Err(e) = picker.move_window(
                window,
                (cursor_pos.0 + ZOOM_WIN_OFFSET) as i32,
                (cursor_pos.1 + ZOOM_WIN_OFFSET) as i32,
                ZOOM_WIN_WIDTH as i32,
                ZOOM_WIN_HEIGHT as i32,
            ) {
                self.error_message = Some(e.to_string());
            }
        }
    }
}
