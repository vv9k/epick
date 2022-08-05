use crate::{
    app::{App, FrameCtx},
    render::render_color,
    save_to_clipboard,
    ui::{colors::*, drag_source, drop_target, icon, HALF_SPACE, SPACE},
};

use egui::{style::Margin, vec2, CursorIcon, Id, Label, RichText, ScrollArea, Ui};

impl App {
    const MAX_NAME_LEN: usize = 15;
    const NAME_MULTIPLIER: usize = 10;
    const NAME_MAX_WIDTH: usize = Self::MAX_NAME_LEN * Self::NAME_MULTIPLIER;
    const NAME_MIN_WIDTH: usize = 50;

    pub fn side_panel(&mut self, ctx: &mut FrameCtx<'_>) {
        let frame = egui::Frame {
            fill: if ctx.egui.style().visuals.dark_mode {
                *D_BG_00
            } else {
                *L_BG_0
            },
            inner_margin: Margin::symmetric(15., 10.),
            ..Default::default()
        };

        let resp = egui::SidePanel::right("colors")
            .frame(frame)
            .resizable(false)
            .max_width(ctx.app.sidepanel.box_width * 1.2)
            .default_width(ctx.app.sidepanel.box_width)
            .show(ctx.egui, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    self.side_ui(ctx, ui);
                })
            });
        ctx.app.sidepanel.response_size = resp.response.rect.size();
    }

    fn side_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.side_panel_palette_picker(ctx, ui);
            ui.add_space(HALF_SPACE);

            let resp = self.side_panel_button_toolbar(ctx, ui);
            ctx.app.sidepanel.box_width = resp.response.rect.width() * 1.3;

            self.side_panel_palette_name(ctx, ui);
            ui.add_space(SPACE);
            self.side_panel_colors_column(ctx, ui);
        });
    }

    fn side_panel_palette_picker(
        &mut self,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) -> egui::InnerResponse<()> {
        let mut selected_palette = ctx.app.palettes.current_idx();
        ui.horizontal(|ui| {
            let resp = egui::ComboBox::from_id_source("side-panel-palette-chooser")
                .width(ctx.app.sidepanel.box_width * 0.69)
                .show_index(ui, &mut selected_palette, ctx.app.palettes.len(), |i| {
                    let mut name = ctx.app.palettes[i].name.clone();
                    Self::format_palette_name(&mut name);
                    name
                });
            if resp.changed() {
                ctx.app.palettes.move_to_idx(selected_palette);
            }
        })
    }

    fn format_palette_name(name: &mut String) {
        if name.len() > Self::MAX_NAME_LEN {
            name.truncate(Self::MAX_NAME_LEN);
            name.push_str("...");
        }
    }

    fn side_panel_button_toolbar(
        &mut self,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) -> egui::InnerResponse<()> {
        ui.horizontal(|ui| {
            if ui
                .button(icon::ADD)
                .on_hover_text("Add a new palette")
                .clicked()
            {
                ctx.app.palettes.append_empty();
                ctx.app.palettes.move_to_last();
            }
            if ui
                .button(icon::CLEAR)
                .on_hover_text("Clear colors")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                ctx.app.palettes.current_mut().palette.clear();
            }
            if ui
                .button(icon::EXPORT)
                .on_hover_text("Export")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                self.export_window.show = true;
                self.export_window.export_palette = Some(ctx.app.palettes.current().clone());
            }
            if ui
                .button(icon::COPY)
                .on_hover_text("Copy all colors to clipboard")
                .on_hover_cursor(CursorIcon::Alias)
                .clicked()
            {
                let _ = save_to_clipboard(ctx.app.palettes.current().palette.as_hex_list());
            }
            #[allow(clippy::collapsible_if)]
            if ui
                .button(icon::EDIT)
                .on_hover_text("Change palette name")
                .on_hover_cursor(CursorIcon::Text)
                .clicked()
            {
                ctx.app.sidepanel.edit_palette_name = !ctx.app.sidepanel.edit_palette_name;
                ctx.app.sidepanel.trigger_edit_focus = ctx.app.sidepanel.edit_palette_name;
            }
            if ui
                .button(icon::DELETE)
                .on_hover_text("Delete current palette")
                .clicked()
            {
                ctx.app.palettes.remove_current();
            }
        })
    }

    fn side_panel_palette_name(
        &mut self,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) -> egui::InnerResponse<()> {
        let current_palette = ctx.app.palettes.current_mut();
        let name_text = if current_palette.name.is_empty() {
            "Current palette".to_string()
        } else {
            current_palette.name.clone()
        };
        ui.scope(|ui| {
            if ctx.app.sidepanel.edit_palette_name {
                let mut edit_name = current_palette.name.clone();
                let width = (edit_name.len() * Self::NAME_MULTIPLIER)
                    .max(Self::NAME_MIN_WIDTH)
                    .min(Self::NAME_MAX_WIDTH) as f32;
                let resp = egui::TextEdit::singleline(&mut edit_name)
                    .desired_width(width)
                    .show(ui);
                if ctx.app.sidepanel.trigger_edit_focus {
                    resp.response.request_focus();
                    ctx.app.sidepanel.trigger_edit_focus = false;
                }
                current_palette.name = edit_name;
                if ui
                    .button(icon::APPLY)
                    .on_hover_text("Finish editing")
                    .clicked()
                {
                    ctx.app.sidepanel.edit_palette_name = false;
                }
            } else {
                let heading = Label::new(RichText::new(name_text).heading());
                ui.add(heading);
            }
        })
    }

    fn side_panel_colors_column(
        &mut self,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) -> egui::InnerResponse<()> {
        let current_palette = ctx.app.palettes.current().clone();
        let mut src_row = None;
        let mut dst_row = None;

        let display_strings: Vec<_> = current_palette
            .palette
            .iter()
            .map(|c| ctx.app.display_color(c))
            .collect();
        let max_len = display_strings
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or_default();
        let box_width = (max_len * 11).max((ctx.app.sidepanel.box_width * 0.64) as usize) as f32;

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
                                        ctx.app.clipboard_color(&self.picker.current_color),
                                    );
                                }
                                if ui
                                    .button(icon::DELETE)
                                    .on_hover_text("Delete this color")
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    ctx.app.palettes.current_mut().palette.remove(color);
                                }
                            });
                            ui.vertical(|ui| {
                                ui.monospace(color_str);
                                let help = format!(
                                    "{}\n\nDrag and drop to change the order of colors",
                                    color_str
                                );

                                let size = vec2(box_width, box_width / 2.);
                                let tex_allocator = &mut ctx.tex_allocator();
                                drag_source(ui, color_id, |ui| {
                                    render_color(
                                        ui,
                                        tex_allocator,
                                        &mut self.texture_manager,
                                        color.color32(),
                                        size,
                                        Some(&help),
                                        false,
                                    );
                                });
                            });
                        });
                        ctx.app.sidepanel.box_width = box_response.response.rect.width();
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
                    let palette = &mut ctx.app.palettes.current_mut().palette;
                    if let Some(it) = palette.remove_pos(src_row) {
                        palette.insert(dst_row, it);
                    }
                }
            }
        }

        resp
    }
}
