use crate::{
    app::App,
    color::NamedPalette,
    context::FrameCtx,
    save_to_clipboard,
    ui::{
        colorbox::{ColorBox, COLORBOX_DRAG_TOOLTIP},
        drag_source, drop_target, icon, DragInfo, SPACE,
    },
};

use egui::{CursorIcon, Id, Label, RichText, ScrollArea, Ui};

impl App {
    pub fn palettes_ui(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ScrollArea::new([true, true]).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut ctx.app.palettes_tab_display_label,
                    "Display color labels",
                );
            });
            ui.add(
                egui::Slider::new(&mut ctx.app.palettes_tab_color_size, 25.0..=100.)
                    .clamp_to_range(true)
                    .text("color size"),
            );
            let mut palette_src_row = None;
            let mut palette_dst_row = None;

            for (i, palette) in ctx.app.palettes.clone().iter().enumerate() {
                if palette.palette.is_empty() {
                    continue;
                }
                let resp = self.display_palette(palette, ctx, ui);
                if ctx.egui.memory().is_anything_being_dragged() {
                    if resp.inner.is_drag_source {
                        palette_src_row = Some(i);
                    } else if resp.inner.is_drop_target {
                        palette_dst_row = Some(i);
                    }
                }
            }
            if let Some(src_row) = palette_src_row {
                if let Some(dst_row) = palette_dst_row {
                    if ui.input().pointer.any_released() {
                        ctx.app.palettes.swap(src_row, dst_row);
                    }
                }
            }
        });
    }

    fn display_palette(
        &mut self,
        palette: &NamedPalette,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) -> egui::InnerResponse<DragInfo> {
        let mut is_drag_source = false;
        let mut is_drop_target = false;
        let mut resp = drop_target(ui, true, |ui| {
            let palette_id = egui::Id::new(&palette.name);
            drag_source(ui, palette_id, |ui| {
                if ui.memory().is_being_dragged(palette_id) {
                    is_drag_source = true;
                }
                if palette.palette.is_empty() {
                    return;
                }
                self.display_palette_buttons(palette, ctx, ui);
                self.display_palette_colors(palette, ctx, ui);
                ui.add_space(SPACE);
            });
            DragInfo::default()
        });
        let is_being_dragged = ui.memory().is_anything_being_dragged();
        if is_being_dragged && resp.response.hovered() {
            is_drop_target = true;
        }
        resp.inner = DragInfo {
            is_drag_source,
            is_drop_target,
        };
        resp
    }

    fn display_palette_buttons(
        &mut self,
        palette: &NamedPalette,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) -> egui::InnerResponse<()> {
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
        })
    }

    fn display_palette_colors(
        &mut self,
        palette: &NamedPalette,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) -> egui::InnerResponse<()> {
        egui::Grid::new(&palette.name)
            .spacing((2.5, 0.))
            .show(ui, |ui| {
                let mut color_src_row = None;
                let mut color_dst_row = None;
                for (i, color) in palette.palette.iter().enumerate() {
                    let resp = drop_target(ui, true, |ui| {
                        let color_id = Id::new(&palette.name).with(i);
                        drag_source(ui, color_id, |ui| {
                            let cb = ColorBox::builder()
                                .size((
                                    ctx.app.palettes_tab_color_size,
                                    ctx.app.palettes_tab_color_size,
                                ))
                                .color(*color)
                                .label(ctx.app.palettes_tab_display_label)
                                .hover_help(COLORBOX_DRAG_TOOLTIP)
                                .build();
                            ui.vertical(|ui| {
                                cb.display(ctx, ui);
                            });
                        });
                        if ui.memory().is_being_dragged(color_id) {
                            color_src_row = Some(i);
                        }
                    });
                    let is_being_dragged = ui.memory().is_anything_being_dragged();
                    if is_being_dragged && resp.response.hovered() {
                        color_dst_row = Some(i);
                    }
                }
                if let Some(src_row) = color_src_row {
                    if let Some(dst_row) = color_dst_row {
                        if ui.input().pointer.any_released() {
                            ctx.app.palettes.move_to_name(&palette.name);
                            let palette = &mut ctx.app.palettes.current_mut().palette;
                            if let Some(it) = palette.remove_pos(src_row) {
                                palette.insert(dst_row, it);
                            }
                        }
                    }
                }
            })
    }
}
