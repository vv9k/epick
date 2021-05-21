use crate::app::render::{tex_color, TextureManager};
use crate::color::{color_as_hex, create_shades};
use crate::save_to_clipboard;

use egui::color::Color32;
use egui::{vec2, ScrollArea, Slider, Ui};

pub struct SchemeGenerator {
    pub numof_shades: u8,
    pub color_size: f32,
    pub base_color: Option<Color32>,
    pub tex_mngr: TextureManager,
}

impl Default for SchemeGenerator {
    fn default() -> Self {
        Self {
            numof_shades: 5,
            color_size: 100.,
            base_color: None,
            tex_mngr: TextureManager::default(),
        }
    }
}

impl SchemeGenerator {
    pub fn set_cur_color(&mut self, color: Color32) {
        self.base_color = Some(color);
    }

    pub fn shades(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut Vec<(String, Color32)>,
    ) {
        if let Some(color) = self.base_color {
            ui.vertical(|ui| {
                ui.heading("Shades");
                ui.add(Slider::new(&mut self.numof_shades, u8::MIN..=50).text("# of shades"));
                ui.add(Slider::new(&mut self.color_size, 20.0..=100.).text("color size"));

                let size = vec2(self.color_size, self.color_size);
                ScrollArea::auto_sized().show(ui, |ui| {
                    create_shades(&color, self.numof_shades)
                        .iter()
                        .for_each(|shade| {
                            let hex = color_as_hex(&shade);
                            ui.add_space(7.);
                            ui.horizontal(|ui| {
                                let color_box = tex_color(
                                    ui,
                                    tex_allocator,
                                    &mut self.tex_mngr,
                                    shade.clone(),
                                    size,
                                    Some(&hex),
                                );
                                if let Some(color_box) = color_box {
                                    ui.monospace(format!("#{}", hex));

                                    if color_box.clicked() {
                                        self.set_cur_color(shade.clone());
                                    }

                                    if color_box.middle_clicked() {
                                        saved_colors.push((hex.clone(), shade.clone()));
                                    }

                                    if color_box.secondary_clicked() {
                                        let _ = save_to_clipboard(hex);
                                    }
                                }
                            });
                        });
                });
            });
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut Vec<(String, Color32)>,
    ) {
        ui.vertical(|ui| {
            self.shades(ui, tex_allocator, saved_colors);
        });
    }
}
