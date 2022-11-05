use crate::{
    app::{
        window::{self, WINDOW_X_OFFSET, WINDOW_Y_OFFSET},
        App, ColorHarmony, FrameCtx,
    },
    color::{Color, Gradient},
    ui::{
        colorbox::{ColorBox, COLORBOX_PICK_TOOLTIP},
        layout::HarmonyLayout,
    },
};

use egui::{vec2, Grid, Slider, Ui, Vec2};
use egui::{CollapsingHeader, ComboBox, Window};

macro_rules! scheme_window_impl {
    ($title:literal, $self:ident, $ctx:ident, $win:ident, $size_field:ident, $colors:expr) => {{
        if $self.windows.$win.is_open {
            let offset = $ctx.egui.style().spacing.slider_width * WINDOW_X_OFFSET;
            let mut is_open = true;
            let is_dark_mode = $ctx.egui.style().visuals.dark_mode;
            Window::new($title)
                .frame(window::default_frame(is_dark_mode))
                .default_pos((offset, WINDOW_Y_OFFSET))
                .collapsible(false)
                .vscroll(true)
                .open(&mut is_open)
                .show($ctx.egui, |ui| {
                    window::apply_default_style(ui, is_dark_mode);
                    $self.windows.$win.sliders(ui);

                    let colors = $colors;
                    let size = vec2(
                        $self.windows.$win.$size_field,
                        $self.windows.$win.$size_field,
                    );

                    let base_cb = ColorBox::builder()
                        .hover_help(COLORBOX_PICK_TOOLTIP)
                        .label(true)
                        .size(size);
                    colors.iter().for_each(|color| {
                        let cb = base_cb.clone().color(*color).build();
                        ui.horizontal(|ui| {
                            cb.display($ctx, ui);
                        });
                    });
                });

            if !is_open {
                $self.windows.$win.is_open = false;
            }
        }
    }};
}

fn cb(
    color: Color,
    display_labels: bool,
    size: Option<Vec2>,
    ctx: &mut FrameCtx<'_>,
    ui: &mut Ui,
) -> egui::Response {
    let color_size = ctx.app.settings.harmony_color_size;
    let size = size.unwrap_or_else(|| vec2(color_size, color_size));
    ui.scope(|ui| {
        let colorbox = ColorBox::builder()
            .color(color)
            .size(size)
            .hover_help(COLORBOX_PICK_TOOLTIP)
            .label(display_labels)
            .build();
        ui.vertical(|ui| {
            colorbox.display(ctx, ui);
        });
    })
    .response
}

impl App {
    pub fn hues_window(&mut self, ctx: &mut FrameCtx<'_>) {
        scheme_window_impl!(
            "Hues",
            self,
            ctx,
            hues,
            hue_color_size,
            ctx.app
                .picker
                .current_color
                .hues(self.windows.hues.num_of_hues, self.windows.hues.hues_step)
        );
    }

    pub fn tints_window(&mut self, ctx: &mut FrameCtx<'_>) {
        scheme_window_impl!(
            "Tints",
            self,
            ctx,
            tints,
            tint_color_size,
            ctx.app
                .picker
                .current_color
                .tints(self.windows.tints.num_of_tints)
        );
    }

    pub fn shades_window(&mut self, ctx: &mut FrameCtx<'_>) {
        scheme_window_impl!(
            "Shades",
            self,
            ctx,
            shades,
            shade_color_size,
            ctx.app
                .picker
                .current_color
                .shades(self.windows.shades.num_of_shades)
        );
    }

    pub fn harmonies_header(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        CollapsingHeader::new("Harmonies")
            .default_open(true)
            .show(ui, |ui| {
                self.harmony_combobox(ctx, ui);
                self.harmony_layout_combobox(ctx, ui);
                ui.add(
                    Slider::new(
                        &mut ctx.app.settings.harmony_color_size,
                        20.0..=ui.available_width() / 4.,
                    )
                    .clamp_to_range(true)
                    .text("color size"),
                );
                ui.checkbox(
                    &mut ctx.app.settings.harmony_display_color_label,
                    "Display color labels",
                );
                ui.checkbox(
                    &mut ctx.app.settings.harmony_display_box,
                    "Display color box",
                );
            });
    }

    pub fn display_harmonies(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        let color_size = ctx.app.settings.harmony_color_size;
        let gradient_size = vec2(color_size * 4., color_size * 2.);
        let dbl_width = vec2(color_size * 2., color_size);
        let dbl_width_third_height = vec2(color_size * 2., color_size * 2. / 3.);
        let dbl_height_third_width = vec2(color_size * 2. / 3., color_size * 2.);
        let dbl_height = vec2(color_size, color_size * 2.);
        let half_height = vec2(color_size, color_size * 1. / 2.);
        let half_width = vec2(color_size * 1. / 2., color_size);

        let display_labels = ctx.app.settings.harmony_display_color_label;
        let color = ctx.app.picker.current_color;
        match ctx.app.settings.harmony {
            ColorHarmony::Complementary => {
                let compl = color.complementary();
                Grid::new("complementary").spacing((0., 0.)).show(ui, |ui| {
                    match ctx.app.settings.harmony_layout {
                        HarmonyLayout::Square => {
                            cb(color, display_labels, None, ctx, ui);
                            cb(compl, display_labels, None, ctx, ui);
                        }
                        HarmonyLayout::Stacked => {
                            cb(color, display_labels, Some(dbl_width), ctx, ui);
                            ui.end_row();
                            cb(compl, display_labels, Some(dbl_width), ctx, ui);
                        }
                        HarmonyLayout::Line => {
                            cb(color, display_labels, Some(dbl_height), ctx, ui);
                            cb(compl, display_labels, Some(dbl_height), ctx, ui);
                        }
                        HarmonyLayout::Gradient => {
                            let gradient = Gradient::from_colors([color, compl]);
                            ui.vertical(|ui| {
                                self.gradient_box(ctx, &gradient, gradient_size, ui, false);
                            });
                        }
                    }
                    ui.end_row();
                });
            }
            ColorHarmony::Triadic => {
                let tri = color.triadic();
                Grid::new("triadic").spacing((0., 0.)).show(ui, |ui| {
                    let c1 = tri.0;
                    let c2 = tri.1;
                    self.three_colors_in_layout(
                        color,
                        c1,
                        c2,
                        dbl_width_third_height,
                        dbl_height_third_width,
                        display_labels,
                        ctx,
                        ui,
                    );
                });
            }
            ColorHarmony::Tetradic => {
                let tetr = color.tetradic();
                Grid::new("tetradic").spacing((0., 0.)).show(ui, |ui| {
                    let c1 = tetr.0;
                    let c2 = tetr.1;
                    let c3 = tetr.2;
                    self.four_colors_in_layout(
                        color,
                        c1,
                        c2,
                        c3,
                        half_height,
                        half_width,
                        display_labels,
                        ctx,
                        ui,
                    );
                });
            }
            ColorHarmony::Analogous => {
                let an = color.analogous();
                Grid::new("analogous").spacing((0., 0.)).show(ui, |ui| {
                    let c1 = an.0;
                    let c2 = an.1;
                    self.three_colors_in_layout(
                        color,
                        c1,
                        c2,
                        dbl_width_third_height,
                        dbl_height_third_width,
                        display_labels,
                        ctx,
                        ui,
                    );
                });
            }
            ColorHarmony::SplitComplementary => {
                let sc = color.split_complementary();
                Grid::new("split-complementary")
                    .spacing((0., 0.))
                    .show(ui, |ui| {
                        let c1 = sc.0;
                        let c2 = sc.1;
                        self.three_colors_in_layout(
                            color,
                            c1,
                            c2,
                            dbl_width_third_height,
                            dbl_height_third_width,
                            display_labels,
                            ctx,
                            ui,
                        );
                    });
            }
            ColorHarmony::Square => {
                let s = color.square();
                Grid::new("square").spacing((0., 0.)).show(ui, |ui| {
                    let c1 = s.0;
                    let c2 = s.1;
                    let c3 = s.2;
                    self.four_colors_in_layout(
                        color,
                        c1,
                        c2,
                        c3,
                        half_height,
                        half_width,
                        display_labels,
                        ctx,
                        ui,
                    );
                });
            }
            ColorHarmony::Monochromatic => {
                let mono = color.monochromatic();
                Grid::new("monochromatic").spacing((0., 0.)).show(ui, |ui| {
                    let c1 = mono.0;
                    let c2 = mono.1;
                    let c3 = mono.2;
                    self.four_colors_in_layout(
                        color,
                        c1,
                        c2,
                        c3,
                        half_height,
                        half_width,
                        display_labels,
                        ctx,
                        ui,
                    );
                });
            }
        }
    }

    fn harmony_layout_combobox(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ComboBox::from_label("Harmony layout")
            .selected_text(ctx.app.settings.harmony_layout.as_ref())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut ctx.app.settings.harmony_layout,
                    HarmonyLayout::Square,
                    HarmonyLayout::Square.as_ref(),
                );
                ui.selectable_value(
                    &mut ctx.app.settings.harmony_layout,
                    HarmonyLayout::Line,
                    HarmonyLayout::Line.as_ref(),
                );
                ui.selectable_value(
                    &mut ctx.app.settings.harmony_layout,
                    HarmonyLayout::Stacked,
                    HarmonyLayout::Stacked.as_ref(),
                );
                ui.selectable_value(
                    &mut ctx.app.settings.harmony_layout,
                    HarmonyLayout::Gradient,
                    HarmonyLayout::Gradient.as_ref(),
                );
            });
    }

    fn harmony_combobox(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        let harmony = &mut ctx.app.settings.harmony;
        ComboBox::from_label("Choose a harmony")
            .selected_text(harmony.as_ref())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    harmony,
                    ColorHarmony::Complementary,
                    ColorHarmony::Complementary.as_ref(),
                );
                ui.selectable_value(
                    harmony,
                    ColorHarmony::Triadic,
                    ColorHarmony::Triadic.as_ref(),
                );
                ui.selectable_value(
                    harmony,
                    ColorHarmony::Tetradic,
                    ColorHarmony::Tetradic.as_ref(),
                );
                ui.selectable_value(
                    harmony,
                    ColorHarmony::Analogous,
                    ColorHarmony::Analogous.as_ref(),
                );
                ui.selectable_value(
                    harmony,
                    ColorHarmony::SplitComplementary,
                    ColorHarmony::SplitComplementary.as_ref(),
                );
                ui.selectable_value(harmony, ColorHarmony::Square, ColorHarmony::Square.as_ref());
                ui.selectable_value(
                    harmony,
                    ColorHarmony::Monochromatic,
                    ColorHarmony::Monochromatic.as_ref(),
                );
            });
    }

    fn three_colors_in_layout(
        &mut self,
        c1: Color,
        c2: Color,
        c3: Color,
        size_stacked: Vec2,
        size_line: Vec2,
        display_labels: bool,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) {
        let color_size = ctx.app.settings.harmony_color_size;
        let dbl_width = vec2(color_size * 2., color_size);
        let gradient_size = vec2(color_size * 4., color_size * 2.);

        match ctx.app.settings.harmony_layout {
            HarmonyLayout::Square => {
                cb(c1, display_labels, Some(dbl_width), ctx, ui);
                ui.end_row();
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing = (0., 0.).into();
                    cb(c2, display_labels, None, ctx, ui);
                    cb(c3, display_labels, None, ctx, ui);
                });
            }
            HarmonyLayout::Stacked => {
                cb(c1, display_labels, Some(size_stacked), ctx, ui);
                ui.end_row();
                cb(c2, display_labels, Some(size_stacked), ctx, ui);
                ui.end_row();
                cb(c3, display_labels, Some(size_stacked), ctx, ui);
            }
            HarmonyLayout::Line => {
                cb(c1, display_labels, Some(size_line), ctx, ui);
                cb(c2, display_labels, Some(size_line), ctx, ui);
                cb(c3, display_labels, Some(size_line), ctx, ui);
            }
            HarmonyLayout::Gradient => {
                ui.vertical(|ui| {
                    let gradient = Gradient::from_colors([c1, c2, c3]);
                    self.gradient_box(ctx, &gradient, gradient_size, ui, false);
                });
            }
        }
    }

    fn four_colors_in_layout(
        &mut self,
        c1: Color,
        c2: Color,
        c3: Color,
        c4: Color,
        size_stacked: Vec2,
        size_line: Vec2,
        display_labels: bool,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
    ) {
        let color_size = ctx.app.settings.harmony_color_size;
        let gradient_size = vec2(color_size * 4., color_size * 2.);
        match ctx.app.settings.harmony_layout {
            HarmonyLayout::Square => {
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing = (0., 0.).into();
                    cb(c1, display_labels, None, ctx, ui);
                    cb(c2, display_labels, None, ctx, ui);
                });
                ui.end_row();
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing = (0., 0.).into();
                    cb(c3, display_labels, None, ctx, ui);
                    cb(c4, display_labels, None, ctx, ui);
                });
            }
            HarmonyLayout::Stacked => {
                cb(c1, display_labels, Some(size_stacked), ctx, ui);
                ui.end_row();
                cb(c2, display_labels, Some(size_stacked), ctx, ui);
                ui.end_row();
                cb(c3, display_labels, Some(size_stacked), ctx, ui);
                ui.end_row();
                cb(c4, display_labels, Some(size_stacked), ctx, ui);
            }
            HarmonyLayout::Line => {
                cb(c1, display_labels, Some(size_line), ctx, ui);
                cb(c2, display_labels, Some(size_line), ctx, ui);
                cb(c3, display_labels, Some(size_line), ctx, ui);
                cb(c4, display_labels, Some(size_line), ctx, ui);
            }
            HarmonyLayout::Gradient => {
                ui.vertical(|ui| {
                    let gradient = Gradient::from_colors([c1, c2, c3, c4]);
                    self.gradient_box(ctx, &gradient, gradient_size, ui, false);
                });
            }
        }
    }
}
