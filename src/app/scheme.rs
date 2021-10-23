use crate::app::ui::windows::{WINDOW_X_OFFSET, WINDOW_Y_OFFSET};
use crate::app::{App, ColorHarmony};

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
            Window::new("Hues")
                .default_pos((offset, WINDOW_Y_OFFSET))
                .collapsible(false)
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
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
            Window::new("Tints")
                .collapsible(false)
                .default_pos(pos)
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
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
            Window::new("Shades")
                .collapsible(false)
                .default_pos((offset, WINDOW_Y_OFFSET))
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
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

    pub fn harmonies(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        CollapsingHeader::new("Harmonies")
            .text_style(TextStyle::Heading)
            .default_open(true)
            .show(ui, |ui| {
                let size = vec2(self.picker.scheme_color_size, self.picker.scheme_color_size);
                const BORDER_OFFSET: f32 = 8.;
                let double_size = vec2(
                    self.picker.scheme_color_size * 2. + BORDER_OFFSET,
                    self.picker.scheme_color_size,
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
                ComboBox::from_label("Choose a type")
                    .selected_text(self.picker.color_harmony.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.picker.color_harmony,
                            ColorHarmony::Complementary,
                            ColorHarmony::Complementary.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.color_harmony,
                            ColorHarmony::Triadic,
                            ColorHarmony::Triadic.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.color_harmony,
                            ColorHarmony::Tetradic,
                            ColorHarmony::Tetradic.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.color_harmony,
                            ColorHarmony::Analogous,
                            ColorHarmony::Analogous.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.color_harmony,
                            ColorHarmony::SplitComplementary,
                            ColorHarmony::SplitComplementary.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.color_harmony,
                            ColorHarmony::Square,
                            ColorHarmony::Square.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.color_harmony,
                            ColorHarmony::Monochromatic,
                            ColorHarmony::Monochromatic.as_ref(),
                        );
                    });
                ui.checkbox(
                    &mut self.picker.display_harmony_color_label,
                    "Display color labels",
                );
                ui.add(
                    Slider::new(
                        &mut self.picker.scheme_color_size,
                        20.0..=ui.available_width() / 4.,
                    )
                    .clamp_to_range(true)
                    .text("color size"),
                );
                let display = self.picker.display_harmony_color_label;
                match self.picker.color_harmony {
                    ColorHarmony::Complementary => {
                        let compl = color.complementary();
                        Grid::new("complementary").spacing((0., 0.)).show(ui, |ui| {
                            cb!(color, ui, display);
                            cb!(compl, ui, display);
                            ui.end_row();
                        });
                    }
                    ColorHarmony::Triadic => {
                        let tri = color.triadic();
                        Grid::new("triadic").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = tri.0;
                            let c2 = tri.1;
                            cb!(color, double_size, ui, display);
                            ui.end_row();
                            ui.scope(|ui| {
                                ui.spacing_mut().item_spacing = (0., 0.).into();
                                cb!(c1, ui, display);
                                cb!(c2, ui, display);
                            });
                            ui.end_row();
                        });
                    }
                    ColorHarmony::Tetradic => {
                        let tetr = color.tetradic();
                        Grid::new("tetradic").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = &tetr.0;
                            let c2 = &tetr.1;
                            let c3 = &tetr.2;
                            cb!(color, ui, display);
                            cb!(c1, ui, display);
                            ui.end_row();
                            cb!(c2, ui, display);
                            cb!(c3, ui, display);
                            ui.end_row();
                        });
                    }
                    ColorHarmony::Analogous => {
                        let an = color.analogous();
                        Grid::new("analogous").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = an.0;
                            let c2 = an.1;
                            cb!(color, double_size, ui, display);
                            ui.end_row();
                            ui.scope(|ui| {
                                ui.spacing_mut().item_spacing = (0., 0.).into();
                                cb!(c1, ui, display);
                                cb!(c2, ui, display);
                            });
                            ui.end_row();
                        });
                    }
                    ColorHarmony::SplitComplementary => {
                        let sc = color.split_complementary();
                        Grid::new("split-complementary")
                            .spacing((0., 0.))
                            .show(ui, |ui| {
                                let c1 = sc.0;
                                let c2 = sc.1;
                                cb!(color, double_size, ui, display);
                                ui.end_row();
                                ui.scope(|ui| {
                                    ui.spacing_mut().item_spacing = (0., 0.).into();
                                    cb!(c1, ui, display);
                                    cb!(c2, ui, display);
                                });
                                ui.end_row();
                            });
                    }
                    ColorHarmony::Square => {
                        let s = color.square();
                        Grid::new("square").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = s.0;
                            let c2 = s.1;
                            let c3 = s.2;
                            cb!(color, ui, display);
                            cb!(c1, ui, display);
                            ui.end_row();
                            cb!(c2, ui, display);
                            cb!(c3, ui, display);
                            ui.end_row();
                        });
                    }
                    ColorHarmony::Monochromatic => {
                        let mono = color.monochromatic();
                        Grid::new("monochromatic").spacing((0., 0.)).show(ui, |ui| {
                            let c1 = mono.0;
                            let c2 = mono.1;
                            let c3 = mono.2;
                            cb!(color, ui, display);
                            cb!(c1, ui, display);
                            ui.end_row();
                            cb!(c2, ui, display);
                            cb!(c3, ui, display);
                            ui.end_row();
                        });
                    }
                }
            });
    }
}
