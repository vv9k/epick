use crate::app::render::{tex_color, TextureManager};
use crate::app::SavedColors;
use crate::color::{color_as_hex, complementary, create_shades, create_tints, triadic};
use crate::save_to_clipboard;

use egui::{color::Color32, ComboBox, Vec2};
use egui::{vec2, ScrollArea, Slider, Ui};

fn color_tooltip(color: &Color32) -> String {
    format!(
        "#{}\n\nPrimary click: set current\nMiddle click: save color\nSecondary click: copy hex",
        color_as_hex(&color)
    )
}

#[derive(Debug, PartialEq)]
pub enum SchemeType {
    Complementary,
    Triadic,
}

pub struct SchemeGenerator {
    pub numof_shades: u8,
    pub numof_tints: u8,
    pub shade_color_size: f32,
    pub tint_color_size: f32,
    pub base_color: Option<Color32>,
    pub tex_mngr: TextureManager,
    pub scheme_ty: SchemeType,
}

impl Default for SchemeGenerator {
    fn default() -> Self {
        Self {
            numof_shades: 6,
            numof_tints: 6,
            shade_color_size: 100.,
            tint_color_size: 100.,
            base_color: None,
            tex_mngr: TextureManager::default(),
            scheme_ty: SchemeType::Complementary,
        }
    }
}

impl SchemeGenerator {
    pub fn set_cur_color(&mut self, color: Color32) {
        self.base_color = Some(color);
    }

    fn color_box(
        &mut self,
        color: &Color32,
        size: Vec2,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
        with_label: bool,
    ) {
        let hex = color_as_hex(&color);
        ui.add_space(7.);
        ui.horizontal(|ui| {
            let color_box = tex_color(
                ui,
                tex_allocator,
                &mut self.tex_mngr,
                color.clone(),
                size,
                Some(&color_tooltip(&color)),
            );
            if let Some(color_box) = color_box {
                if with_label {
                    ui.monospace(format!("#{}", hex));
                }

                if color_box.clicked() {
                    self.set_cur_color(color.clone());
                }

                if color_box.middle_clicked() {
                    saved_colors.add(color.clone());
                }

                if color_box.secondary_clicked() {
                    let _ = save_to_clipboard(hex);
                }
            }
        });
    }

    pub fn tints(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
    ) {
        ui.vertical(|ui| {
            ui.heading("Tints");
            if let Some(color) = self.base_color {
                let tints = create_tints(&color, self.numof_tints);
                ui.add(Slider::new(&mut self.numof_tints, u8::MIN..=25).text("# of tints"));
                ui.add(Slider::new(&mut self.tint_color_size, 20.0..=200.).text("color size"));

                let size = vec2(self.tint_color_size, self.tint_color_size);
                ScrollArea::auto_sized()
                    .id_source("tints scroll")
                    .show(ui, |ui| {
                        tints.iter().for_each(|tint| {
                            self.color_box(tint, size, ui, tex_allocator, saved_colors, true);
                        });
                    });
            } else {
                ui.label("Select a color from saved colors");
            }
        });
    }
    pub fn shades(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
    ) {
        ui.vertical(|ui| {
            ui.heading("Shades");
            if let Some(color) = self.base_color {
                let shades = create_shades(&color, self.numof_shades);
                ui.add(Slider::new(&mut self.numof_shades, u8::MIN..=25).text("# of shades"));
                ui.add(Slider::new(&mut self.shade_color_size, 20.0..=200.).text("color size"));

                let size = vec2(self.shade_color_size, self.shade_color_size);
                ScrollArea::auto_sized()
                    .id_source("shades scroll")
                    .show(ui, |ui| {
                        shades.iter().for_each(|shade| {
                            self.color_box(shade, size, ui, tex_allocator, saved_colors, true);
                        });
                    });
            } else {
                ui.label("Select a color from saved colors");
            }
        });
    }

    pub fn schemes(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
    ) {
        ui.heading("Schemes");
        ComboBox::from_label("Choose a type").show_ui(ui, |ui| {
            ui.selectable_value(
                &mut self.scheme_ty,
                SchemeType::Complementary,
                "Complementary",
            );
            ui.selectable_value(&mut self.scheme_ty, SchemeType::Triadic, "Triadic");
        });

        if let Some(color) = self.base_color {
            match self.scheme_ty {
                SchemeType::Complementary => {
                    let compl = complementary(&color);
                    ui.vertical(|ui| {
                        ui.scope(|mut ui| {
                            self.color_box(
                                &color,
                                vec2(250., 250.),
                                &mut ui,
                                tex_allocator,
                                saved_colors,
                                false,
                            );
                        });
                        ui.scope(|mut ui| {
                            self.color_box(
                                &compl,
                                vec2(250., 250.),
                                &mut ui,
                                tex_allocator,
                                saved_colors,
                                false,
                            );
                        });
                    });
                }
                SchemeType::Triadic => {
                    let tri = triadic(&color);
                    ui.vertical(|ui| {
                        ui.scope(|mut ui| {
                            self.color_box(
                                &tri.0,
                                vec2(250., 250.),
                                &mut ui,
                                tex_allocator,
                                saved_colors,
                                false,
                            );
                        });
                        ui.scope(|mut ui| {
                            self.color_box(
                                &tri.1,
                                vec2(250., 250.),
                                &mut ui,
                                tex_allocator,
                                saved_colors,
                                false,
                            );
                        });
                        ui.scope(|mut ui| {
                            self.color_box(
                                &color,
                                vec2(250., 250.),
                                &mut ui,
                                tex_allocator,
                                saved_colors,
                                false,
                            )
                        });
                    });
                }
            }
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
    ) {
        ui.columns(3, |columns| {
            self.shades(&mut columns[0], tex_allocator, saved_colors);
            self.tints(&mut columns[1], tex_allocator, saved_colors);
            self.schemes(&mut columns[2], tex_allocator, saved_colors);
        });
    }
}
