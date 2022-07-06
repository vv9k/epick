use egui::{Button, Key, TextBuffer, TextEdit, Window};

use crate::{
    app::{
        settings::{DisplayFmtEnum, Settings},
        ADD_ICON, APPLY_ICON, DELETE_ICON, EDIT_ICON,
    },
    color::{Color, DisplayFormat},
};

#[derive(Default, Debug)]
pub struct CustomFormatsWindow {
    pub show: bool,
    pub new_name: String,
    pub edit_key: String,
    pub new_key: String,
    pub adding_new: bool,
    pub highlighted_key: String,
}
impl CustomFormatsWindow {
    pub(crate) fn display(
        &mut self,
        settings: &mut Settings,
        ctx: &egui::Context,
        preview_color: Color,
    ) {
        Window::new("Custom color formats")
            .open(&mut self.show)
            .show(ctx, |ui| {
                let mut replace = false;
                let keys: Vec<String> = settings.saved_color_formats.keys().cloned().collect();
                let enter_pressed = ui.input().key_pressed(Key::Enter);
                egui::Grid::new("custom_formats_grid")
                    .num_columns(4)
                    .show(ui, |ui| {
                        settings.saved_color_formats.retain(|k, v| {
                            let mut retain = true;
                            if ui.button(DELETE_ICON).clicked() {
                                retain = false;
                            }
                            let text_edit_id = ui.make_persistent_id(format!("ke_{}", k));
                            if self.edit_key == *k {
                                let valid = name_valid(&self.new_key, keys.iter());
                                let edit_re = ui
                                    .add(TextEdit::singleline(&mut self.new_key).id(text_edit_id));
                                let btn_re = ui.add_enabled(valid, Button::new(APPLY_ICON));
                                if valid
                                    && (btn_re.clicked() || (edit_re.lost_focus() && enter_pressed))
                                {
                                    replace = true;
                                } else if edit_re.lost_focus() {
                                    self.edit_key.clear();
                                }
                            } else {
                                ui.label(k);
                                if ui.button(EDIT_ICON).clicked() {
                                    ui.memory().request_focus(text_edit_id);
                                    self.edit_key = k.clone();
                                    self.new_key = k.clone();
                                }
                            }
                            let edit_re = ui.text_edit_singleline(v);
                            if edit_re.gained_focus() {
                                self.highlighted_key = k.clone();
                            } else if *k == self.highlighted_key && edit_re.lost_focus() {
                                self.highlighted_key.clear();
                            }

                            if !retain {
                                // Check if format is used by current display format
                                let is_in_use = matches!(&settings.color_display_format, DisplayFmtEnum::Custom(fmt) if fmt == k);
                                if is_in_use {
                                    settings.color_display_format = DisplayFmtEnum::default();
                                }

                                // Check if format is used by current clipboard format
                                let is_in_use = matches!(&settings.color_clipboard_format, Some(DisplayFmtEnum::Custom(fmt)) if fmt == k);
                                if is_in_use {
                                    settings.color_clipboard_format =
                                        Some(DisplayFmtEnum::default());
                                }
                            }

                            ui.end_row();
                            retain
                        });
                        ui.end_row();
                        let mut adding_new_opened = false;
                        if ui.button(ADD_ICON).clicked() {
                            self.adding_new ^= true;
                            adding_new_opened = true;
                        }
                        if self.adding_new {
                            let edit_re = ui.text_edit_singleline(&mut self.new_name);
                            if adding_new_opened {
                                edit_re.request_focus();
                            }
                            let valid =
                                name_valid(&self.new_name, settings.saved_color_formats.keys());
                            let btn_re = ui.add_enabled(valid, Button::new(APPLY_ICON));
                            if valid
                                && (btn_re.clicked() || (edit_re.lost_focus() && enter_pressed))
                            {
                                settings
                                    .saved_color_formats
                                    .insert(self.new_name.take(), String::default());
                                self.adding_new = false;
                            }
                            if edit_re.lost_focus() {
                                self.adding_new = false;
                            }
                        }
                    });
                if replace {
                    let value = settings.saved_color_formats.remove(&self.edit_key).unwrap();
                    settings
                        .saved_color_formats
                        .insert(self.new_key.take(), value);
                    self.edit_key.clear();
                }
                if !self.highlighted_key.is_empty() {
                    ui.heading("Preview");
                    let preview_string = preview_color.display(
                        DisplayFormat::Custom(&settings.saved_color_formats[&self.highlighted_key]),
                        settings.rgb_working_space,
                        settings.illuminant,
                    );
                    ui.label(preview_string);
                }
            });
    }
}

fn name_valid<'a>(name: &str, mut existing_names: impl Iterator<Item = &'a String>) -> bool {
    !name.is_empty() && !existing_names.any(|ex_name| ex_name == name)
}
