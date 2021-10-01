use egui::color::{Color32, Hsva, Rgba};
use std::cmp::Ordering;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Cmyk {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}

impl Cmyk {
    pub fn new(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self { c, m, y, k }
    }
}

impl From<Cmyk> for Color32 {
    fn from(cmyk: Cmyk) -> Self {
        let r = (255. * (1. - (cmyk.c * (1. - cmyk.k) + cmyk.k))).round() as u8;
        let g = (255. * (1. - (cmyk.m * (1. - cmyk.k) + cmyk.k))).round() as u8;
        let b = (255. * (1. - (cmyk.y * (1. - cmyk.k) + cmyk.k))).round() as u8;
        Color32::from_rgb(r, g, b)
    }
}

impl From<Cmyk> for Rgba {
    fn from(cmyk: Cmyk) -> Rgba {
        Color32::from(cmyk).into()
    }
}

impl From<Cmyk> for Hsva {
    fn from(cmyk: Cmyk) -> Hsva {
        Color32::from(cmyk).into()
    }
}

impl From<Rgba> for Cmyk {
    fn from(rgba: Rgba) -> Cmyk {
        Color32::from(rgba).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Color32> for Cmyk {
    fn from(color: Color32) -> Self {
        let r: f32 = 1. - (color.r() as f32 / u8::MAX as f32);
        let g: f32 = 1. - (color.g() as f32 / u8::MAX as f32);
        let b: f32 = 1. - (color.b() as f32 / u8::MAX as f32);
        let rgb = [r, g, b];
        let k = rgb
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap();

        if (*k - 1.).abs() < f32::EPSILON {
            return Cmyk::new(0., 0., 0., *k);
        }

        let c = (r - k) / (1. - k);
        let m = (g - k) / (1. - k);
        let y = (b - k) / (1. - k);

        Cmyk::new(
            if c.is_nan() { 0. } else { c },
            if m.is_nan() { 0. } else { m },
            if y.is_nan() { 0. } else { y },
            if k.is_nan() { 0. } else { *k },
        )
    }
}
