use crate::app::{ColorPicker, SchemeType, SideTab};
use egui::{vec2, Slider, Ui};
use egui::{CollapsingHeader, ComboBox, Window};

impl ColorPicker {
    pub fn hues(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(SideTab::Hues) = self.side_panel_visible {
            let mut is_open = true;
            Window::new("Hues")
                .anchor(egui::Align2::RIGHT_TOP, vec2(0., 0.))
                .collapsible(false)
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    let color = &self.cur_color;
                    let hues = color.hues(self.numof_hues, self.hues_step);
                    ui.add(Slider::new(&mut self.hues_step, 0.01..=0.1).text("step"));
                    let max_hues = (0.5 / self.hues_step).round() as u8;
                    if self.numof_hues > max_hues {
                        self.numof_hues = max_hues;
                    }
                    ui.add(Slider::new(&mut self.numof_hues, u8::MIN..=max_hues).text("# of hues"));
                    ui.add(Slider::new(&mut self.hue_color_size, 20.0..=200.).text("color size"));

                    let size = vec2(self.hue_color_size, self.hue_color_size);
                    hues.iter().for_each(|hue| {
                        self.color_box_label_side(hue, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.side_panel_visible = None;
            }
        }
    }

    pub fn tints(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(SideTab::Tints) = self.side_panel_visible {
            let mut is_open = true;
            Window::new("Tints")
                .anchor(egui::Align2::RIGHT_TOP, vec2(0., 0.))
                .collapsible(false)
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    let color = &self.cur_color;
                    let tints = color.tints(self.numof_tints);
                    ui.add(Slider::new(&mut self.numof_tints, u8::MIN..=50).text("# of tints"));
                    ui.add(Slider::new(&mut self.tint_color_size, 20.0..=200.).text("color size"));

                    let size = vec2(self.tint_color_size, self.tint_color_size);
                    tints.iter().for_each(|tint| {
                        self.color_box_label_side(tint, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.side_panel_visible = None;
            }
        }
    }

    pub fn shades(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(SideTab::Shades) = self.side_panel_visible {
            let mut is_open = true;
            Window::new("Shades")
                .anchor(egui::Align2::RIGHT_TOP, vec2(0., 0.))
                .collapsible(false)
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    let color = self.cur_color;
                    let shades = color.shades(self.numof_shades);
                    ui.add(Slider::new(&mut self.numof_shades, u8::MIN..=50).text("# of shades"));
                    ui.add(Slider::new(&mut self.shade_color_size, 20.0..=200.).text("color size"));

                    let size = vec2(self.shade_color_size, self.shade_color_size);
                    shades.iter().for_each(|shade| {
                        self.color_box_label_side(shade, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.side_panel_visible = None;
            }
        }
    }

    pub fn schemes(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        CollapsingHeader::new("Schemes")
            .default_open(true)
            .show(ui, |ui| {
                let size = vec2(self.scheme_color_size, self.scheme_color_size);
                let double_size = vec2(
                    (self.scheme_color_size + ui.spacing().item_spacing.x) * 2.,
                    self.scheme_color_size,
                );

                macro_rules! cb {
                    ($color:ident, $size:expr, $ui:ident) => {
                        $ui.scope(|mut ui| {
                            self.color_box_label_under(&$color, $size, &mut ui, tex_allocator);
                        });
                    };
                    ($color:ident, $ui:ident) => {
                        cb!($color, size, $ui)
                    };
                }

                let color = self.cur_color;
                ComboBox::from_label("Choose a type")
                    .selected_text(self.scheme_ty.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.scheme_ty,
                            SchemeType::Complementary,
                            SchemeType::Complementary.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.scheme_ty,
                            SchemeType::Triadic,
                            SchemeType::Triadic.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.scheme_ty,
                            SchemeType::Tetradic,
                            SchemeType::Tetradic.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.scheme_ty,
                            SchemeType::Analogous,
                            SchemeType::Analogous.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.scheme_ty,
                            SchemeType::SplitComplementary,
                            SchemeType::SplitComplementary.as_ref(),
                        );
                    });
                ui.add(Slider::new(&mut self.scheme_color_size, 100.0..=250.).text("color size"));
                match self.scheme_ty {
                    SchemeType::Complementary => {
                        let compl = color.complementary();
                        ui.horizontal(|ui| {
                            cb!(color, ui);
                            cb!(compl, ui);
                        });
                    }
                    SchemeType::Triadic => {
                        let tri = color.triadic();
                        ui.vertical(|ui| {
                            let c1 = tri.0;
                            let c2 = tri.1;
                            cb!(color, double_size, ui);
                            ui.horizontal(|ui| {
                                cb!(c1, ui);
                                cb!(c2, ui);
                            });
                        });
                    }
                    SchemeType::Tetradic => {
                        let tetr = color.tetradic();
                        ui.vertical(|ui| {
                            let c1 = &tetr.0;
                            let c2 = &tetr.1;
                            let c3 = &tetr.2;
                            ui.horizontal(|ui| {
                                cb!(color, ui);
                                cb!(c1, ui);
                            });
                            ui.horizontal(|ui| {
                                cb!(c2, ui);
                                cb!(c3, ui);
                            });
                        });
                    }
                    SchemeType::Analogous => {
                        let an = color.analogous();
                        ui.vertical(|ui| {
                            let c1 = an.0;
                            let c2 = an.1;
                            cb!(color, double_size, ui);
                            ui.horizontal(|ui| {
                                cb!(c1, ui);
                                cb!(c2, ui);
                            });
                        });
                    }
                    SchemeType::SplitComplementary => {
                        let sc = color.split_complementary();
                        ui.vertical(|ui| {
                            let c1 = sc.0;
                            let c2 = sc.1;
                            cb!(color, double_size, ui);
                            ui.horizontal(|ui| {
                                cb!(c1, ui);
                                cb!(c2, ui);
                            });
                        });
                    }
                }
            });
    }
}
