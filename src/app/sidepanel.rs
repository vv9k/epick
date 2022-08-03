use crate::app::{
    render::tex_color,
    ui::{colors::*, drag_source, drop_target, icon, HALF_SPACE, SPACE},
    App,
};
use crate::{save_to_clipboard, TextureAllocator};

use egui::{style::Margin, vec2, CursorIcon, Id, Label, RichText, ScrollArea, Ui};

impl App {
    const MAX_NAME_LEN: usize = 15;
    const NAME_MULTIPLIER: usize = 10;
    const NAME_MAX_WIDTH: usize = Self::MAX_NAME_LEN * Self::NAME_MULTIPLIER;
    const NAME_MIN_WIDTH: usize = 50;

    pub fn side_panel(&mut self, ctx: &egui::Context, tex_allocator: &mut TextureAllocator) {
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
            .max_width(self.sp_box_width * 1.2)
            .default_width(self.sp_box_width)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    self.side_ui(ui, tex_allocator);
                })
            });
    }

    fn side_ui(&mut self, ui: &mut Ui, tex_allocator: &mut TextureAllocator) {
        ui.vertical(|ui| {
            self.side_panel_palette_picker(ui);
            ui.add_space(HALF_SPACE);

            let resp = self.side_panel_button_toolbar(ui);
            self.sp_box_width = resp.response.rect.width() * 1.3;

            self.side_panel_palette_name(ui);
            ui.add_space(SPACE);
            self.side_panel_colors_column(ui, tex_allocator);
        });
    }

    fn side_panel_palette_picker(&mut self, ui: &mut Ui) -> egui::InnerResponse<()> {
        let original_name = self.palettes.current().name.clone();
        let mut selected_palette_name = original_name;
        Self::format_palette_name(&mut selected_palette_name);
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source("side-panel-palette-chooser")
                .selected_text(&selected_palette_name)
                .width(self.sp_box_width * 0.69)
                .show_ui(ui, |ui| {
                    for palette in self.palettes.iter() {
                        let mut display_name = palette.name.clone();
                        Self::format_palette_name(&mut display_name);
                        let _ = ui.selectable_value(
                            &mut selected_palette_name,
                            palette.name.clone(),
                            display_name,
                        );
                    }
                });

            if !&self
                .palettes
                .current()
                .name
                .starts_with(selected_palette_name.trim_end_matches("..."))
            {
                self.palettes.move_to_name(&selected_palette_name);
            }
        })
    }

    fn format_palette_name(name: &mut String) {
        if name.len() > Self::MAX_NAME_LEN {
            name.truncate(Self::MAX_NAME_LEN);
            name.push_str("...");
        }
    }

    fn side_panel_button_toolbar(&mut self, ui: &mut Ui) -> egui::InnerResponse<()> {
        ui.horizontal(|ui| {
            if ui
                .button(icon::ADD)
                .on_hover_text("Add a new palette")
                .clicked()
            {
                self.palettes.append_empty();
                self.palettes.move_to_last();
            }
            if ui
                .button(icon::CLEAR)
                .on_hover_text("Clear colors")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                self.palettes.current_mut().palette.clear();
            }
            if ui
                .button(icon::EXPORT)
                .on_hover_text("Export")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                self.export_window.show = true;
            }
            if ui
                .button(icon::COPY)
                .on_hover_text("Copy all colors to clipboard")
                .on_hover_cursor(CursorIcon::Alias)
                .clicked()
            {
                let _ = save_to_clipboard(self.palettes.current().palette.as_hex_list());
            }
            #[allow(clippy::collapsible_if)]
            if ui
                .button(icon::EDIT)
                .on_hover_text("Change palette name")
                .on_hover_cursor(CursorIcon::Text)
                .clicked()
            {
                self.sp_edit_palette_name = !self.sp_edit_palette_name;
                self.sp_trigger_edit_focus = self.sp_edit_palette_name;
            }
            if ui
                .button(icon::DELETE)
                .on_hover_text("Delete current palette")
                .clicked()
            {
                self.palettes.remove_current();
            }
        })
    }

    fn side_panel_palette_name(&mut self, ui: &mut Ui) -> egui::InnerResponse<()> {
        let current_palette = self.palettes.current_mut();
        let name_text = if current_palette.name.is_empty() {
            "Current palette".to_string()
        } else {
            current_palette.name.clone()
        };
        ui.scope(|ui| {
            if self.sp_edit_palette_name {
                let mut edit_name = current_palette.name.clone();
                let width = (edit_name.len() * Self::NAME_MULTIPLIER)
                    .max(Self::NAME_MIN_WIDTH)
                    .min(Self::NAME_MAX_WIDTH) as f32;
                let resp = egui::TextEdit::singleline(&mut edit_name)
                    .desired_width(width)
                    .show(ui);
                if self.sp_trigger_edit_focus {
                    resp.response.request_focus();
                    self.sp_trigger_edit_focus = false;
                }
                current_palette.name = edit_name;
                if ui
                    .button(icon::APPLY)
                    .on_hover_text("Finish editing")
                    .clicked()
                {
                    self.sp_edit_palette_name = false;
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
        let box_width = (max_len * 11).max((self.sp_box_width * 0.64) as usize) as f32;

        let resp = ui.scope(|ui| {
            for (idx, color) in current_palette.palette.iter().enumerate() {
                let resp = drop_target(ui, true, |ui| {
                    let color_id = Id::new("side-color").with(idx);
                    let color_str = &display_strings[idx];
                    ui.vertical(|ui| {
                        let box_response = ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                if ui
                                    .button(icon::PLAY)
                                    .on_hover_text("Use this color")
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    self.picker.set_cur_color(*color);
                                }
                                if ui
                                    .button(icon::COPY)
                                    .on_hover_text("Copy color")
                                    .on_hover_cursor(CursorIcon::Alias)
                                    .clicked()
                                {
                                    let _ = save_to_clipboard(
                                        self.clipboard_color(&self.picker.current_color),
                                    );
                                }
                                if ui
                                    .button(icon::DELETE)
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
                        self.sp_box_width = box_response.response.rect.width();
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
}
