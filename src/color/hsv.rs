#![allow(clippy::many_single_char_names)]
use crate::color::rgb::Rgb;
use crate::color::{CIEColor, Cmyk, Color, Hsl};
use egui::color::{Color32, Hsva, Rgba};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsv {
    h: f32,
    s: f32,
    v: f32,
}

impl Hsv {
    /// Takes in values in the range of 0.0 ..= 1.0 and returns a HSV color.
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        let hue = if hue.is_nan() {
            0.
        } else if hue > 1. {
            hue / 360.
        } else {
            hue
        };
        let saturation = if saturation.is_nan() {
            0.
        } else if saturation > 1. {
            saturation / 100.
        } else {
            saturation
        };
        let value = if value.is_nan() {
            0.
        } else if value > 1. {
            value / 100.
        } else {
            value
        };
        Self {
            h: hue,
            s: saturation,
            v: value,
        }
    }

    #[inline(always)]
    /// Returns Hue in the range of 0.0 ..= 1.0
    pub fn h(&self) -> f32 {
        self.h
    }

    #[inline(always)]
    /// Returns Saturation in the range of 0.0 ..= 1.0
    pub fn s(&self) -> f32 {
        self.s
    }

    #[inline(always)]
    /// Returns Value in the range of 0.0 ..= 1.0
    pub fn v(&self) -> f32 {
        self.v
    }

    /// Returns Hue in the range of 0.0 ..= 360.0
    pub fn h_scaled(&self) -> f32 {
        self.h * 360.
    }

    /// Returns Saturation in the range of 0.0 ..= 100.0
    pub fn s_scaled(&self) -> f32 {
        self.s * 100.
    }

    /// Returns Value in the range of 0.0 ..= 100.0
    pub fn v_scaled(&self) -> f32 {
        self.v * 100.
    }

    pub fn offset_hue(&mut self, offset: f32) {
        self.h = (self.h + offset) % 360.;
    }

    pub fn offset_saturation(&mut self, offset: f32) {
        self.s = (self.s + offset) % 100.;
    }
}

//####################################################################################################

impl From<Hsv> for Color32 {
    fn from(color: Hsv) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Color32> for Hsv {
    fn from(color: Color32) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsv> for Hsva {
    fn from(hsv: Hsv) -> Self {
        Hsva {
            h: hsv.h(),
            s: hsv.s(),
            v: hsv.v(),
            a: 1.,
        }
    }
}

impl From<Hsva> for Hsv {
    fn from(hsv: Hsva) -> Self {
        Self::new(hsv.h, hsv.s, hsv.v)
    }
}

impl From<Hsv> for Rgba {
    fn from(color: Hsv) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Rgba> for Hsv {
    fn from(color: Rgba) -> Self {
        Rgb::from(color).into()
    }
}

//####################################################################################################

impl From<Color> for Hsv {
    fn from(c: Color) -> Hsv {
        match c {
            Color::Rgb(c) => c.into(),
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Hsv(c) => c,
            Color::Hsl(c) => c.into(),
            Color::Xyz(c, ws) => c.to_rgb(ws).into(),
            Color::xyY(c, ws) => c.to_rgb(ws).into(),
            Color::Luv(c, ws) => c.to_rgb(ws).into(),
            Color::LchUV(c, ws) => c.to_rgb(ws).into(),
            Color::Lab(c, ws) => c.to_rgb(ws).into(),
            Color::LchAB(c, ws) => c.to_rgb(ws).into(),
        }
    }
}

impl From<&Color> for Hsv {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Cmyk> for Hsv {
    fn from(color: Cmyk) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsl> for Hsv {
    fn from(color: Hsl) -> Self {
        let h = color.h();
        let mut ss = color.s();
        let l = color.l() * 2.;

        if l <= 1. {
            ss *= l;
        } else {
            ss *= 2. - l;
        }

        let v = (l + ss) / 2.;
        let s = (2. * ss) / (l + ss);

        Hsv::new(h, s, v)
    }
}

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
        let r = rgb.r();
        let g = rgb.g();
        let b = rgb.b();
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let h = if delta == 0. {
            0.
        } else if (max - r).abs() < f32::EPSILON {
            (g - b) / (delta * 6.)
        } else if (max - g).abs() < f32::EPSILON {
            (b - r) / (delta * 6.) + 1.0 / 3.0
        } else {
            (r - g) / (delta * 6.) + 2.0 / 3.0
        };
        let h = (h + 1.).fract();

        let v = max;
        let s = if v == 0. { 0. } else { 1. - min / max };

        Hsv::new(h, s, v)
    }
}
