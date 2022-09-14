use egui::{ComboBox, Window};

use crate::{
    color::{CustomPaletteFormat, PaletteDisplayFormat},
    context::FrameCtx,
    ui::{icon, SPACE},
};

#[derive(Default, Debug)]
pub struct PaletteFormatsWindow {
    pub show: bool,
    pub current_format: Option<(String, CustomPaletteFormat)>,
}
impl PaletteFormatsWindow {
    pub(crate) fn display(&mut self, ctx: &mut FrameCtx<'_>) {
        Window::new("Custom palette formats")
            .open(&mut self.show)
            .show(ctx.egui, |ui| {
                let mut current = if let Some(current) = &self.current_format {
                    current.clone()
                } else {
                    ctx.app
                        .settings
                        .saved_palette_formats
                        .iter()
                        .next()
                        .map(|(name, fmt)| (name.clone(), fmt.clone()))
                        .unwrap_or_default()
                };

                ui.horizontal(|ui| {
                    let name_before_selection = current.0.clone();
                    ComboBox::new("palette_format_combobox", "")
                        .selected_text(&current.0)
                        .show_ui(ui, |ui| {
                            for (name, _) in &ctx.app.settings.saved_palette_formats {
                                ui.selectable_value(&mut current.0, name.clone(), name);
                            }
                        });
                    if name_before_selection != current.0 {
                        self.current_format = ctx
                            .app
                            .settings
                            .saved_palette_formats
                            .get(&current.0)
                            .map(|fmt| (current.0.clone(), fmt.clone()));
                        current = self.current_format.clone().unwrap_or_default();
                    }

                    if ui
                        .button(icon::DELETE)
                        .on_hover_text("Delete this format")
                        .clicked()
                    {
                        ctx.app.settings.saved_palette_formats.remove(&current.0);
                        return;
                    }
                    if ui
                        .button(icon::ADD)
                        .on_hover_text("Add a new format")
                        .clicked()
                    {
                        let new = CustomPaletteFormat::default();
                        let len = ctx.app.settings.saved_palette_formats.len();
                        let name = format!("palette format {len}");
                        ctx.app
                            .settings
                            .saved_palette_formats
                            .insert(name.clone(), new);
                        self.current_format = ctx
                            .app
                            .settings
                            .saved_palette_formats
                            .get(&name)
                            .map(|fmt| (name, fmt.clone()));
                        return;
                    }
                    if ui
                        .button(icon::PLAY)
                        .on_hover_text("Use this format")
                        .clicked()
                    {
                        ctx.app.settings.palette_clipboard_format =
                            PaletteDisplayFormat::Custom(current.0.clone(), current.1.clone());
                        return;
                    }
                });
                let (name_before_edit, format_before_edit) = current.clone();

                ui.add_space(SPACE);
                egui::Grid::new("palette_format_edit_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut current.0);
                        ui.end_row();

                        ui.label("Prefix: ");
                        egui::TextEdit::multiline(&mut current.1.prefix)
                            .desired_rows(1)
                            .show(ui);
                        ui.end_row();

                        ui.label("Color format: ");
                        egui::TextEdit::multiline(&mut current.1.entry_format)
                            .desired_rows(1)
                            .show(ui);
                        ui.end_row();

                        ui.label("Suffix: ");
                        egui::TextEdit::multiline(&mut current.1.suffix)
                            .desired_rows(1)
                            .show(ui);
                        ui.end_row();
                    });

                let mut preview = current
                    .1
                    .format_palette(
                        &ctx.app.palettes.current().palette,
                        ctx.app.settings.rgb_working_space,
                        ctx.app.settings.illuminant,
                    )
                    .unwrap_or_default();

                ui.add_space(SPACE);
                ui.label("Preview");
                egui::TextEdit::multiline(&mut preview)
                    .interactive(false)
                    .font(egui::TextStyle::Monospace)
                    .frame(false)
                    .show(ui);

                if ui.button("copy").clicked() {
                    let _ = crate::save_to_clipboard(preview);
                }

                if name_before_edit != current.0 || format_before_edit != current.1 {
                    ctx.app
                        .settings
                        .saved_palette_formats
                        .remove(&name_before_edit);
                    ctx.app
                        .settings
                        .saved_palette_formats
                        .insert(current.0.clone(), current.1);
                    self.current_format = ctx
                        .app
                        .settings
                        .saved_palette_formats
                        .get(&current.0)
                        .map(|fmt| (current.0, fmt.clone()));
                }
            });
    }
}
