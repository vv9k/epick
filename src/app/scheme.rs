use crate::{
    app::{
        windows::{self, WINDOW_X_OFFSET, WINDOW_Y_OFFSET},
        App, ColorHarmony, FrameCtx,
    },
    color::Gradient,
    ui::{
        colorbox::{ColorBox, COLORBOX_PICK_TOOLTIP},
        layout::HarmonyLayout,
        DOUBLE_SPACE,
    },
};

use egui::{vec2, Grid, Slider, Ui};
use egui::{CollapsingHeader, ComboBox, Window};

macro_rules! scheme_window_impl {
    ($title:literal, $self:ident, $ctx:ident, $win:ident, $size_field:ident, $colors:expr) => {{
        if $self.$win.is_open {
            let offset = $ctx.egui.style().spacing.slider_width * WINDOW_X_OFFSET;
            let mut is_open = true;
            let is_dark_mode = $ctx.egui.style().visuals.dark_mode;
            Window::new($title)
                .frame(windows::default_frame(is_dark_mode))
                .default_pos((offset, WINDOW_Y_OFFSET))
                .collapsible(false)
                .vscroll(true)
                .open(&mut is_open)
                .show($ctx.egui, |ui| {
                    windows::apply_default_style(ui, is_dark_mode);
                    $self.$win.sliders(ui);

                    let colors = $colors;
                    let size = vec2($self.$win.$size_field, $self.$win.$size_field);

                    let base_cb = ColorBox::builder()
                        .hover_help(COLORBOX_PICK_TOOLTIP)
                        .label(true)
                        .size(size);
                    colors.iter().for_each(|color| {
                        let cb = base_cb.clone().color(*color).build();
                        ui.horizontal(|ui| {
                            $self.display_color_box(cb, $ctx, ui);
                        });
                    });
                });

            if !is_open {
                $self.$win.is_open = false;
            }
        }
    }};
}

impl App {
    pub fn hues_window(&mut self, ctx: &mut FrameCtx<'_>) {
        scheme_window_impl!(
            "Hues",
            self,
            ctx,
            hues_window,
            hue_color_size,
            self.picker
                .current_color
                .hues(self.hues_window.num_of_hues, self.hues_window.hues_step)
        );
    }

    pub fn tints_window(&mut self, ctx: &mut FrameCtx<'_>) {
        scheme_window_impl!(
            "Tints",
            self,
            ctx,
            tints_window,
            tint_color_size,
            self.picker
                .current_color
                .tints(self.tints_window.num_of_tints)
        );
    }

    pub fn shades_window(&mut self, ctx: &mut FrameCtx<'_>) {
        scheme_window_impl!(
            "Shades",
            self,
            ctx,
            shades_window,
            shade_color_size,
            self.picker
                .current_color
                .shades(self.shades_window.num_of_shades)
        );
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

    pub fn harmonies(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        let color_size = ctx.app.settings.harmony_color_size;
        CollapsingHeader::new("Harmonies")
            .default_open(true)
            .show(ui, |ui| {
                let size = vec2(color_size, color_size);
                let gradient_size = vec2(
                    color_size * 4. ,
                    color_size * 2. ,
                );
                let dbl_width = vec2(
                    color_size * 2. ,
                    color_size,
                );
                let dbl_height = vec2(
                    color_size,
                    color_size * 2. ,
                );

                let dbl_width_third_height = vec2(
                    color_size * 2. ,
                    color_size * 2. / 3.,
                );
                let dbl_height_third_width = vec2(
                    color_size * 2. / 3.,
                    color_size * 2. ,
                );

                let half_height = vec2(
                    color_size ,
                    color_size * 1. / 2.,
                );
                let half_width = vec2(
                    color_size * 1. / 2.,
                    color_size ,
                );

                macro_rules! cb {
                    ($color:ident, $size:expr, $ui:ident, $display_labels:ident) => {
                        $ui.scope(|ui| {
                            let colorbox = ColorBox::builder()
                                .color($color)
                                .size($size)
                                .label($display_labels).build();
                            ui.vertical(|ui| {
                                self.display_color_box(colorbox, ctx, ui);
                            });
                        });
                    };
                    ($color:ident, $ui:ident, $display_labels:ident) => {
                        cb!($color, size, $ui, $display_labels)
                    };
                }

                let color = self.picker.current_color;
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
                        ui.selectable_value(
                            harmony,
                            ColorHarmony::Square,
                            ColorHarmony::Square.as_ref(),
                        );
                        ui.selectable_value(
                            harmony,
                            ColorHarmony::Monochromatic,
                            ColorHarmony::Monochromatic.as_ref(),
                        );
                    });
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
                ui.add_space(DOUBLE_SPACE);
                macro_rules! colors_in_layout {
                    ($ui:ident, $c1:ident, $c2: ident, $c3:ident, $display:ident, $s1:ident, $s2:ident)  => {
                        let ui = $ui;
                        let display_labels = $display;
                        let dbl_width_third_height = $s1;
                        let dbl_height_third_width = $s2;
                        match ctx.app.settings.harmony_layout {
                            HarmonyLayout::Square => {
                                cb!($c1, dbl_width, ui, display_labels);
                                ui.end_row();
                                ui.scope(|ui| {
                                    ui.spacing_mut().item_spacing = (0., 0.).into();
                                    cb!($c2, ui, display_labels);
                                    cb!($c3, ui, display_labels);

                                });

                            }
                            HarmonyLayout::Stacked => {
                                cb!($c1, dbl_width_third_height, ui, display_labels);
                                ui.end_row();
                                cb!($c2, dbl_width_third_height, ui, display_labels);
                                ui.end_row();
                                cb!($c3, dbl_width_third_height, ui, display_labels);
                            }
                            HarmonyLayout::Line => {
                                cb!($c1, dbl_height_third_width, ui, display_labels);
                                cb!($c2, dbl_height_third_width, ui, display_labels);
                                cb!($c3, dbl_height_third_width, ui, display_labels);
                            }
                            HarmonyLayout::Gradient => {
                                ui.vertical(|ui| {
                                    let gradient = Gradient::from_colors([$c1, $c2, $c3]);
                                    self.gradient_box(ctx, &gradient, gradient_size, ui, false);
                                });
                            }
                        }
                    };
                    ($ui:ident, $c1:ident, $c2: ident, $c3:ident, $c4:ident, $display:ident, $stacked_size:ident, $line_size:ident)  => {
                        let ui = $ui;
                        let display_labels = $display;
                        let stacked_size = $stacked_size;
                        let line_size = $line_size;
                        match ctx.app.settings.harmony_layout {
                            HarmonyLayout::Square => {
                                ui.scope(|ui| {
                                    ui.spacing_mut().item_spacing = (0., 0.).into();
                                    cb!($c1, ui, display_labels);
                                    cb!($c2, ui, display_labels);

                                });
                                ui.end_row();
                                ui.scope(|ui| {
                                    ui.spacing_mut().item_spacing = (0., 0.).into();
                                    cb!($c3, ui, display_labels);
                                    cb!($c4, ui, display_labels);
                                });

                            }
                            HarmonyLayout::Stacked => {
                                cb!($c1, stacked_size, ui, display_labels);
                                ui.end_row();
                                cb!($c2, stacked_size, ui, display_labels);
                                ui.end_row();
                                cb!($c3, stacked_size, ui, display_labels);
                                ui.end_row();
                                cb!($c4, stacked_size, ui, display_labels);
                            }
                            HarmonyLayout::Line => {
                                cb!($c1, line_size, ui, display_labels);
                                cb!($c2, line_size, ui, display_labels);
                                cb!($c3, line_size, ui, display_labels);
                                cb!($c4, line_size, ui, display_labels);
                            }
                            HarmonyLayout::Gradient => {
                                ui.vertical(|ui| {
                                    let gradient = Gradient::from_colors([$c1, $c2, $c3, $c4]);
                                    self.gradient_box(ctx, &gradient, gradient_size, ui, false);
                                });
                            }
                        }
                    }
                }
                let display_label = ctx.app.settings.harmony_display_color_label;
                match ctx.app.settings.harmony {
                    ColorHarmony::Complementary => {
                        let compl = color.complementary();
                        Grid::new("complementary").spacing((0., 0.)).show(ui, |ui| {
                            match ctx.app.settings.harmony_layout {
                                HarmonyLayout::Square => {
                                    cb!(color, ui,  display_label);
                                    cb!(compl, ui, display_label);
                                }
                                HarmonyLayout::Stacked => {
                                    cb!(color, dbl_width, ui, display_label);
                                    ui.end_row();
                                    cb!(compl, dbl_width, ui, display_label);
                                }
                                HarmonyLayout::Line => {
                                    cb!(color, dbl_height, ui, display_label);
                                    cb!(compl, dbl_height, ui, display_label);
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
                            colors_in_layout!(ui, color, c1, c2, display_label, dbl_width_third_height, dbl_height_third_width);
                        });
                    }
                    ColorHarmony::Tetradic => {
                        let tetr = color.tetradic();
                        Grid::new("tetradic").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = tetr.0;
                            let c2 = tetr.1;
                            let c3 = tetr.2;
                            colors_in_layout!(ui, color, c1, c2, c3, display_label, half_height, half_width);
                        });
                    }
                    ColorHarmony::Analogous => {
                        let an = color.analogous();
                        Grid::new("analogous").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = an.0;
                            let c2 = an.1;
                            colors_in_layout!(ui, color, c1, c2, display_label, dbl_width_third_height, dbl_height_third_width);
                        });
                    }
                    ColorHarmony::SplitComplementary => {
                        let sc = color.split_complementary();
                        Grid::new("split-complementary")
                            .spacing((0., 0.))
                            .show(ui, |ui| {
                                let c1 = sc.0;
                                let c2 = sc.1;
                                colors_in_layout!(ui, color, c1, c2, display_label, dbl_width_third_height, dbl_height_third_width);
                            });
                    }
                    ColorHarmony::Square => {
                        let s = color.square();
                        Grid::new("square").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = s.0;
                            let c2 = s.1;
                            let c3 = s.2;
                            colors_in_layout!(ui, color, c1, c2, c3, display_label, half_height, half_width);
                        });
                    }
                    ColorHarmony::Monochromatic => {
                        let mono = color.monochromatic();
                        Grid::new("monochromatic").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = mono.0;
                            let c2 = mono.1;
                            let c3 = mono.2;
                            colors_in_layout!(ui, color, c1, c2, c3, display_label, half_height, half_width);
                        });
                    }
                }
            });
    }
}
