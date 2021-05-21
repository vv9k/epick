use crate::app::render::{tex_color, TextureManager};
use crate::app::SavedColors;
use crate::color::{color_as_hex, create_shades, create_tints};
use crate::save_to_clipboard;

use egui::{color::Color32, Vec2};
use egui::{vec2, ScrollArea, Slider, Ui};

pub struct SchemeGenerator {
    pub numof_shades: u8,
    pub numof_tints: u8,
    pub shade_color_size: f32,
    pub tint_color_size: f32,
    pub base_color: Option<Color32>,
    pub tex_mngr: TextureManager,
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
    ) {
        let hex = color_as_hex(&color);
        ui.add_space(7.);
        ui.horizontal(|ui| {
                                let help = format!("#{}\n\nPrimary click: set current\nMiddle click: save color\nSecondary click: copy hex", hex);
                                let color_box = tex_color(
                                    ui,
                                    tex_allocator,
                                    &mut self.tex_mngr,
                                    color.clone(),
                                    size,
                                    Some(&help),
                                );
                                if let Some(color_box) = color_box {
                                    ui.monospace(format!("#{}", hex));

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
                ui.add(Slider::new(&mut self.numof_tints, u8::MIN..=50).text("# of tints"));
                ui.add(Slider::new(&mut self.tint_color_size, 20.0..=100.).text("color size"));

                let size = vec2(self.tint_color_size, self.tint_color_size);
                ScrollArea::auto_sized()
                    .id_source("tints scroll")
                    .show(ui, |ui| {
                        tints.iter().for_each(|tint| {
                            self.color_box(tint, size, ui, tex_allocator, saved_colors);
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
                ui.add(Slider::new(&mut self.numof_shades, u8::MIN..=50).text("# of shades"));
                ui.add(Slider::new(&mut self.shade_color_size, 20.0..=100.).text("color size"));

                let size = vec2(self.shade_color_size, self.shade_color_size);
                ScrollArea::auto_sized()
                    .id_source("shades scroll")
                    .show(ui, |ui| {
                        shades.iter().for_each(|shade| {
                            self.color_box(shade, size, ui, tex_allocator, saved_colors);
                        });
                    });
            } else {
                ui.label("Select a color from saved colors");
            }
        });
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
        saved_colors: &mut SavedColors,
    ) {
        ui.columns(2, |columns| {
            self.shades(&mut columns[0], tex_allocator, saved_colors);
            self.tints(&mut columns[1], tex_allocator, saved_colors);
        });
    }
}
