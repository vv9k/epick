use eframe::egui::TextStyle;
use egui::{Label, Window};

#[derive(Debug, Default)]
pub struct HelpWindow {
    pub is_open: bool,
}

impl HelpWindow {
    pub fn toggle_window(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn display(&mut self, ctx: &egui::CtxRef) {
        macro_rules! show_keybinding {
            ($ui:ident, $key:literal, $description:literal) => {
                $ui.horizontal(|ui| {
                    let key = Label::new($key).strong();
                    ui.add(key);
                    ui.label($description);
                });
            };
        }
        if self.is_open {
            let mut is_open = true;
            Window::new("Help")
                .collapsible(false)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        let label = Label::new("Keybindings").text_style(TextStyle::Heading);
                        ui.add(label);

                        show_keybinding!(ui, "z", "display zoomed window");
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
