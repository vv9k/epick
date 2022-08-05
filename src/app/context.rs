use crate::{
    app::{load_settings, CentralPanelTab, Color, CursorIcon, Palettes, ScreenSize, Settings},
    TextureAllocator,
};
use serde::{Deserialize, Serialize};

use eframe::CreationContext;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppCtx {
    pub settings: Settings,

    pub palettes: Palettes,
    pub palettes_tab_display_label: bool,

    pub screen_size: ScreenSize,
    pub cursor_icon: CursorIcon,
    pub cursor_pick_color: Color,
    pub current_selected_color: Color,
    pub central_panel_tab: CentralPanelTab,

    pub sidepanel: SidePanelData,

    pub zoom_window_dragged: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SidePanelData {
    pub show: bool,
    pub edit_palette_name: bool,
    pub trigger_edit_focus: bool,
    pub box_width: f32,
    pub response_size: egui::Vec2,
}

impl Default for AppCtx {
    fn default() -> Self {
        Self {
            settings: Settings::default(),

            palettes: Palettes::default(),
            palettes_tab_display_label: false,

            screen_size: ScreenSize::Desktop(0., 0.),
            cursor_icon: CursorIcon::default(),
            cursor_pick_color: Color::black(),
            current_selected_color: Color::black(),
            central_panel_tab: CentralPanelTab::Picker,
            sidepanel: SidePanelData {
                show: false,
                edit_palette_name: false,
                trigger_edit_focus: false,
                box_width: 0.,
                response_size: (0., 0.).into(),
            },

            zoom_window_dragged: false,
        }
    }
}
impl AppCtx {
    pub const KEY: &'static str = "app-global-ctx";
    pub fn new(context: &CreationContext) -> Self {
        Self {
            settings: load_settings(context.storage).unwrap_or_default(),

            palettes: Palettes::default(),
            palettes_tab_display_label: false,
            screen_size: ScreenSize::Desktop(0., 0.),
            cursor_icon: CursorIcon::default(),
            cursor_pick_color: Color::black(),
            current_selected_color: Color::black(),
            central_panel_tab: CentralPanelTab::Picker,
            sidepanel: SidePanelData {
                show: false,
                edit_palette_name: false,
                trigger_edit_focus: false,
                box_width: 0.,
                response_size: (0., 0.).into(),
            },

            zoom_window_dragged: false,
        }
    }
}

pub struct FrameCtx<'frame> {
    pub app: &'frame mut AppCtx,
    pub egui: &'frame egui::Context,
}

impl<'frame> FrameCtx<'frame> {
    pub fn new(app: &'frame mut AppCtx, egui: &'frame egui::Context) -> Self {
        Self { app, egui }
    }

    pub fn tex_allocator(&self) -> TextureAllocator {
        Some(self.egui.tex_manager())
    }

    pub fn is_dark_mode(&self) -> bool {
        self.app.settings.is_dark_mode
    }
}
