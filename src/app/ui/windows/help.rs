use crate::app::ui::windows::{self, WINDOW_X_OFFSET, WINDOW_Y_OFFSET};
use egui::RichText;
use egui::{Label, Window};

#[derive(Debug, Default)]
pub struct HelpWindow {
    pub is_open: bool,
}

impl HelpWindow {
    pub fn toggle_window(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn display(&mut self, ctx: &egui::Context) {
        macro_rules! show_keybinding {
            ($ui:ident, $key:literal, $description:literal) => {
                $ui.horizontal(|ui| {
                    let key = Label::new(RichText::new($key).strong());
                    ui.add(key);
                    ui.label($description);
                });
            };
        }
        if self.is_open {
            let offset = ctx.style().spacing.slider_width * WINDOW_X_OFFSET;
            let mut is_open = true;
            let is_dark_mode = ctx.style().visuals.dark_mode;

            Window::new("Help")
                .collapsible(false)
                .frame(windows::default_frame(is_dark_mode))
                .default_pos((offset, WINDOW_Y_OFFSET))
                .open(&mut is_open)
                .show(ctx, |ui| {
                    windows::apply_default_style(ui, is_dark_mode);
                    ui.vertical(|ui| {
                        let label = Label::new(RichText::new("Keybindings").strong());
                        ui.add(label);

                        show_keybinding!(ui, "p", "pick a color from under the cursor");
                        show_keybinding!(ui, "s", "save a color from under the cursor");
                        show_keybinding!(ui, "h", "toggle side panel");
                    });
                });

            if !is_open {
                self.is_open = false;
            }
        }
    }
}
