#![allow(dead_code)]
mod scheme;
mod sidepanel;
pub mod window;

use crate::color::{Color, ColorHarmony, Gradient};
use crate::context::{AppCtx, FrameCtx};
use crate::error::{append_global_error, DisplayError, ERROR_STACK};
use crate::keybinding::{default_keybindings, KeyBindings};
use crate::render::{render_gradient, TextureManager};
use crate::save_to_clipboard;
use crate::screen_size::ScreenSize;
use crate::settings;
use crate::ui::{
    colorbox::{ColorBox, COLORBOX_DRAG_TOOLTIP, COLORBOX_PICK_TOOLTIP},
    colors::*,
    dark_visuals, drag_source, drop_target, icon, light_visuals, DOUBLE_SPACE, SPACE,
};
use crate::zoom_picker::ZoomPicker;
use window::{ExportWindow, HelpWindow, HuesWindow, SettingsWindow, ShadesWindow, TintsWindow};

use eframe::{CreationContext, Storage};
use egui::{
    color::Color32, style::Margin, vec2, Button, CollapsingHeader, CursorIcon, Id, Label, Layout,
    Rgba, RichText, ScrollArea, Ui, Vec2, Visuals,
};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use std::time::Duration;

static ADD_DESCR: &str = "Add this color to saved colors";

const ERROR_DISPLAY_DURATION: Duration = Duration::new(20, 0);

//####################################################################################################

lazy_static::lazy_static! {
    pub static ref KEYBINDINGS: KeyBindings = default_keybindings();
    pub static ref LIGHT_VISUALS: Visuals = light_visuals();
    pub static ref DARK_VISUALS: Visuals = dark_visuals();
}

static CONTEXT: OnceCell<RwLock<AppCtx>> = OnceCell::new();
static TEXTURE_MANAGER: Lazy<RwLock<TextureManager>> =
    Lazy::new(|| RwLock::new(TextureManager::default()));

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum CentralPanelTab {
    Picker,
    Palettes,
}

pub struct App {
    pub display_errors: Vec<DisplayError>,

    pub settings_window: SettingsWindow,
    pub export_window: ExportWindow,
    pub help_window: HelpWindow,
    pub hues_window: HuesWindow,
    pub tints_window: TintsWindow,
    pub shades_window: ShadesWindow,

    pub zoom_picker: ZoomPicker,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if let Some(mut app_ctx) = CONTEXT.get().and_then(|ctx| ctx.write().ok()) {
            let res = TEXTURE_MANAGER.try_write();
            if res.is_err() {
                append_global_error(res.unwrap_err());
                return;
            }
            let mut tex_manager = res.unwrap();
            let mut ctx = FrameCtx {
                app: &mut app_ctx,
                egui: ctx,
                tex_manager: &mut tex_manager,
            };
            ctx.egui.output().cursor_icon = ctx.app.cursor_icon;

            let screen_size = ScreenSize::from(ctx.egui.available_rect());
            if ctx.app.screen_size != screen_size {
                ctx.set_styles(screen_size);
            }

            ctx.app.check_settings_change();

            self.top_panel(&mut ctx);

            self.central_panel(&mut ctx);

            if ctx.app.sidepanel.show {
                self.side_panel(&mut ctx);
            }

            self.display_windows(&mut ctx);

            frame.set_window_size(ctx.egui.used_size());

            ctx.app.picker.check_for_change();

            #[cfg(not(target_arch = "wasm32"))]
            // populate display errors from the global error stack
            if let Ok(mut stack) = ERROR_STACK.try_lock() {
                while let Some(error) = stack.errors.pop_front() {
                    self.display_errors.push(error);
                }
            }
            #[cfg(target_arch = "wasm32")]
            unsafe {
                while let Some(error) = ERROR_STACK.errors.pop_front() {
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

            ctx.app.current_selected_color = ctx.app.picker.current_color;
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if let Some(ctx) = CONTEXT.get().and_then(|ctx| ctx.read().ok()) {
            ctx.save_palettes(storage);
            settings::save_global(&ctx.settings, storage);
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
            display_errors: Default::default(),

            settings_window: SettingsWindow::default(),
            export_window: ExportWindow::default(),
            help_window: HelpWindow::default(),
            hues_window: HuesWindow::default(),
            tints_window: TintsWindow::default(),
            shades_window: ShadesWindow::default(),

            zoom_picker: ZoomPicker::default(),
        });

        let prefer_dark = context.integration_info.prefer_dark_mode.unwrap_or(true);

        if let Ok(mut tex_manager) = TEXTURE_MANAGER.write() {
            let mut ctx = FrameCtx {
                app: &mut app_ctx,
                egui: &context.egui_ctx,
                tex_manager: &mut tex_manager,
            };

            ctx.app.load_palettes(context.storage);

            if prefer_dark {
                ctx.set_dark_theme();
            } else {
                ctx.set_light_theme();
            }
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

    fn check_keys_pressed(&mut self, ctx: &mut FrameCtx) {
        for kb in KEYBINDINGS.iter() {
            if ctx.egui.input().key_pressed(kb.key()) {
                let f = kb.binding();
                f(ctx)
            }
        }
    }

    fn hex_input(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        CollapsingHeader::new("Text input").show(ui, |ui| {
            ui.label("Enter a hex color: ");
            ui.horizontal(|ui| {
                let resp = ui.text_edit_singleline(&mut ctx.app.picker.hex_color);
                if (resp.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                    || ui
                        .button(icon::PLAY)
                        .on_hover_text("Use this color")
                        .on_hover_cursor(CursorIcon::PointingHand)
                        .clicked()
                {
                    if ctx.app.picker.hex_color.len() < 6 {
                        append_global_error("Enter a color first (ex. ab12ff #1200ff)".to_owned());
                    } else if let Some(color) =
                        Color::from_hex(ctx.app.picker.hex_color.trim_start_matches('#'))
                    {
                        ctx.app.picker.set_cur_color(color);
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
                    ctx.app.add_cur_color()
                }
            });
        });
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
            ctx.tex_manager,
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
            ctx.set_theme();
        }
    }

    fn display_windows(&mut self, ctx: &mut FrameCtx<'_>) {
        self.settings_window.display(ctx.app, ctx.egui);
        self.settings_window.custom_formats_window.display(
            &mut ctx.app.settings,
            ctx.egui,
            ctx.app.picker.current_color,
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

            inner_margin: Margin {
                left: 10.,
                top: 5.,
                right: 0.,
                bottom: 0.,
            },
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
                                        cb.display(ctx, ui);
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
                    save_to_clipboard(ctx.app.clipboard_color(&ctx.app.picker.current_color))
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
                ctx.app.add_cur_color();
            }
        });
        let cb = ColorBox::builder()
            .size((25., 25.))
            .color(ctx.app.picker.current_color)
            .label(true)
            .hover_help(COLORBOX_PICK_TOOLTIP)
            .border(true)
            .build();
        ui.horizontal(|ui| {
            cb.display(ctx, ui);
        });

        self.zoom_picker.display(ctx, ui);

        ui.add_space(SPACE);
        ScrollArea::vertical()
            .id_source("picker scroll")
            .show(ui, |ui| {
                self.harmonies(ctx, ui);
                self.sliders(ctx, ui);
                self.hex_input(ctx, ui);
                let mut available_space = ui.available_size_before_wrap();
                if ctx.app.sidepanel.show {
                    available_space.x -= ctx.app.sidepanel.response_size.x;
                }
                ui.allocate_space(available_space);
            });
    }

    fn sliders(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            if ctx.app.settings.color_spaces.rgb {
                ctx.app.picker.rgb_sliders(ui);
            }
            if ctx.app.settings.color_spaces.cmyk {
                ctx.app.picker.cmyk_sliders(ui);
            }
            if ctx.app.settings.color_spaces.hsv {
                ctx.app.picker.hsv_sliders(ui);
            }
            if ctx.app.settings.color_spaces.hsl {
                ctx.app.picker.hsl_sliders(ui);
            }
            if ctx.app.settings.color_spaces.luv {
                ctx.app.picker.luv_sliders(ui);
            }
            if ctx.app.settings.color_spaces.lch_uv {
                ctx.app.picker.lch_uv_sliders(ui);
            }
            if ctx.app.settings.color_spaces.lab {
                ctx.app.picker.lab_sliders(ui);
            }
            if ctx.app.settings.color_spaces.lch_ab {
                ctx.app.picker.lch_ab_sliders(ui);
            }
        });
    }
}
