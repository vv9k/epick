use crate::app::ui::layout::HarmonyLayout;
use crate::app::ui::windows::{self, WINDOW_X_OFFSET, WINDOW_Y_OFFSET};
use crate::app::ui::DOUBLE_SPACE;
use crate::app::{App, ColorHarmony};
use crate::color::Gradient;

use eframe::egui::TextStyle;
use egui::{vec2, Grid, Slider, Ui};
use egui::{CollapsingHeader, ComboBox, Window};

impl App {
    pub fn hues_window(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if self.hues_window.is_open {
            let offset = ctx.style().spacing.slider_width * WINDOW_X_OFFSET;
            let mut is_open = true;
            let is_dark_mode = ctx.style().visuals.dark_mode;
            Window::new("Hues")
                .frame(windows::default_frame(is_dark_mode))
                .default_pos((offset, WINDOW_Y_OFFSET))
                .collapsible(false)
                .vscroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    windows::apply_default_style(ui, is_dark_mode);
                    self.hues_window.sliders(ui);

                    let color = &self.picker.current_color;
                    let hues = color.hues(self.hues_window.num_of_hues, self.hues_window.hues_step);
                    let size = vec2(
                        self.hues_window.hue_color_size,
                        self.hues_window.hue_color_size,
                    );

                    hues.iter().for_each(|hue| {
                        self.color_box_label_side(hue, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.hues_window.is_open = false;
            }
        }
    }

    pub fn tints_window(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if self.tints_window.is_open {
            let offset = ctx.style().spacing.slider_width * WINDOW_X_OFFSET;
            let pos = (offset, WINDOW_Y_OFFSET);
            let mut is_open = true;
            let is_dark_mode = ctx.style().visuals.dark_mode;
            Window::new("Tints")
                .frame(windows::default_frame(is_dark_mode))
                .collapsible(false)
                .default_pos(pos)
                .vscroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    windows::apply_default_style(ui, is_dark_mode);
                    self.tints_window.sliders(ui);

                    let color = &self.picker.current_color;
                    let tints = color.tints(self.tints_window.num_of_tints);
                    let size = vec2(
                        self.tints_window.tint_color_size,
                        self.tints_window.tint_color_size,
                    );
                    tints.iter().for_each(|tint| {
                        self.color_box_label_side(tint, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.tints_window.is_open = false;
            }
        }
    }

    pub fn shades_window(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if self.shades_window.is_open {
            let offset = ctx.style().spacing.slider_width * WINDOW_X_OFFSET;
            let mut is_open = true;
            let is_dark_mode = ctx.style().visuals.dark_mode;
            Window::new("Shades")
                .frame(windows::default_frame(is_dark_mode))
                .collapsible(false)
                .default_pos((offset, WINDOW_Y_OFFSET))
                .vscroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    windows::apply_default_style(ui, is_dark_mode);
                    self.shades_window.sliders(ui);

                    let color = self.picker.current_color;
                    let shades = color.shades(self.shades_window.num_of_shades);
                    let size = vec2(
                        self.shades_window.shade_color_size,
                        self.shades_window.shade_color_size,
                    );

                    shades.iter().for_each(|shade| {
                        self.color_box_label_side(shade, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.shades_window.is_open = false;
            }
        }
    }

    fn harmony_layout_combobox(&mut self, ui: &mut Ui) {
        ComboBox::from_label("Harmony layout")
            .selected_text(self.settings_window.settings.harmony_layout.as_ref())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.settings_window.settings.harmony_layout,
                    HarmonyLayout::Square,
                    HarmonyLayout::Square.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings_window.settings.harmony_layout,
                    HarmonyLayout::Line,
                    HarmonyLayout::Line.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings_window.settings.harmony_layout,
                    HarmonyLayout::Stacked,
                    HarmonyLayout::Stacked.as_ref(),
                );
                ui.selectable_value(
                    &mut self.settings_window.settings.harmony_layout,
                    HarmonyLayout::Gradient,
                    HarmonyLayout::Gradient.as_ref(),
                );
            });
    }

    pub fn harmonies(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        let color_size = self.settings_window.settings.harmony_color_size;
        CollapsingHeader::new("Harmonies")
            .text_style(TextStyle::Heading)
            .default_open(true)
            .show(ui, |ui| {
                let size = vec2(color_size, color_size);
                const BORDER_OFFSET: f32 = 8.;
                let gradient_size = vec2(
                    color_size * 4. + BORDER_OFFSET,
                    color_size * 2. + BORDER_OFFSET,
                );
                let dbl_width = vec2(
                    color_size * 2. + BORDER_OFFSET,
                    color_size,
                );
                let dbl_height = vec2(
                    color_size,
                    color_size * 2. + BORDER_OFFSET,
                );

                let dbl_width_third_height = vec2(
                    color_size * 2. + BORDER_OFFSET,
                    color_size * 2. / 3.,
                );
                let dbl_height_third_width = vec2(
                    color_size * 2. / 3.,
                    color_size * 2. + BORDER_OFFSET,
                );

                let half_height = vec2(
                    color_size + BORDER_OFFSET,
                    color_size * 1. / 2.,
                );
                let half_width = vec2(
                    color_size * 1. / 2.,
                    color_size + BORDER_OFFSET,
                );

                macro_rules! cb {
                    ($color:ident, $size:expr, $ui:ident, $display_labels:ident) => {
                        $ui.scope(|ui| {
                            if $display_labels {
                                self.color_box_label_under(&$color, $size, ui, tex_allocator);
                            } else {
                                ui.vertical(|ui| {
                                    self.color_box_no_label(&$color, $size, ui, tex_allocator);
                                });
                            }
                        });
                    };
                    ($color:ident, $ui:ident, $display_labels:ident) => {
                        cb!($color, size, $ui, $display_labels)
                    };
                }

                let color = self.picker.current_color;
                let harmony = &mut self.settings_window.settings.harmony;
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
                self.harmony_layout_combobox(ui);
                ui.add(
                    Slider::new(
                        &mut self.settings_window.settings.harmony_color_size,
                        20.0..=ui.available_width() / 4.,
                    )
                    .clamp_to_range(true)
                    .text("color size"),
                );
                ui.checkbox(
                    &mut self.settings_window.settings.harmony_display_color_label,
                    "Display color labels",
                );
                ui.add_space(DOUBLE_SPACE);
                macro_rules! colors_in_layout {
                    ($ui:ident, $c1:ident, $c2: ident, $c3:ident, $display:ident, $s1:ident, $s2:ident)  => {
                        let ui = $ui;
                        let display_labels = $display;
                        let dbl_width_third_height = $s1;
                        let dbl_height_third_width = $s2;
                        match self.settings_window.settings.harmony_layout {
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
                                    self.gradient_box(&gradient, gradient_size, ui, tex_allocator);
                                });
                            }
                        }
                    };
                    ($ui:ident, $c1:ident, $c2: ident, $c3:ident, $c4:ident, $display:ident, $stacked_size:ident, $line_size:ident)  => {
                        let ui = $ui;
                        let display_labels = $display;
                        let stacked_size = $stacked_size;
                        let line_size = $line_size;
                        match self.settings_window.settings.harmony_layout {
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
                                    self.gradient_box(&gradient, gradient_size, ui, tex_allocator);
                                });
                            }
                        }
                    }
                }
                let display_label = self.settings_window.settings.harmony_display_color_label;
                match self.settings_window.settings.harmony {
                    ColorHarmony::Complementary => {
                        let compl = color.complementary();
                        Grid::new("complementary").spacing((0., 0.)).show(ui, |ui| {
                            match self.settings_window.settings.harmony_layout {
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
                                        self.gradient_box(&gradient, gradient_size, ui, tex_allocator);
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
