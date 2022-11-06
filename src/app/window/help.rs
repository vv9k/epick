use crate::app::{
    window::{self, WINDOW_X_OFFSET, WINDOW_Y_OFFSET},
    KEYBINDINGS,
};

use egui::{Label, RichText, Window};

#[derive(Debug, Default)]
pub struct HelpWindow {
    pub is_open: bool,
}

impl HelpWindow {
    pub fn toggle_window(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn display(&mut self, ctx: &egui::Context) {
        if self.is_open {
            let offset = ctx.style().spacing.slider_width * WINDOW_X_OFFSET;
            let mut is_open = true;
            let is_dark_mode = ctx.style().visuals.dark_mode;

            Window::new("Help")
                .collapsible(false)
                .frame(window::default_frame(is_dark_mode))
                .default_pos((offset, WINDOW_Y_OFFSET))
                .open(&mut is_open)
                .show(ctx, |ui| {
                    window::apply_default_style(ui, is_dark_mode);
                    ui.vertical(|ui| {
                        let label = Label::new(RichText::new("Keybindings").heading());
                        ui.add(label);
                        for kb in KEYBINDINGS.iter() {
                            ui.horizontal(|ui| {
                                let key = Label::new(RichText::new(kb.str_key()).strong());
                                ui.add(key);
                                ui.label(kb.description());
                            });
                        }
                    });
                });

            if !is_open {
                self.is_open = false;
            }
        }
    }
}
