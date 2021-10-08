use egui::Color32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
        }
    }
}

impl From<egui::Rgba> for Rgb {
    fn from(rgb: egui::Rgba) -> Self {
        Self {
            r: rgb.r(),
            g: rgb.g(),
            b: rgb.b(),
        }
    }
}

impl From<Rgb> for egui::Rgba {
    fn from(rgb: Rgb) -> Self {
        egui::Rgba::from_rgb(rgb.r, rgb.g, rgb.b)
    }
}

impl From<Rgb> for Color32 {
    fn from(rgb: Rgb) -> Self {
        egui::Rgba::from(rgb).into()
    }
}
