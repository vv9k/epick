use crate::color::hsv::Hsv;
use crate::color::rgb::Rgb;
use crate::color::{Cmyk, Color, Hsl, Luv, Xyz};
use egui::color::{Color32, Hsva, Rgba};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lch {
    l: f32,
    c: f32,
    h: f32,
}

impl Lch {
    pub fn new(l: f32, c: f32, h: f32) -> Self {
        let l = if l.is_nan() { 0. } else { l };
        let c = if c.is_nan() { 0. } else { c };
        let h = if h.is_nan() { 0. } else { h };

        Self { l, c, h }
    }

    #[inline(always)]
    /// Returns Light
    pub fn l(&self) -> f32 {
        self.l
    }

    #[inline(always)]
    /// Returns Chroma
    pub fn c(&self) -> f32 {
        self.c
    }

    #[inline(always)]
    /// Returns Hue in the range of 0.0 ..= 1.0
    pub fn h(&self) -> f32 {
        self.h
    }

    /// Returns Hue scaled in the range of 0.0 ..= 360.0
    pub fn h_scaled(&self) -> f32 {
        self.h * 360.
    }
}

//####################################################################################################

impl From<Lch> for Color32 {
    fn from(color: Lch) -> Self {
        Luv::from(color).into()
    }
}

impl From<Color32> for Lch {
    fn from(color: Color32) -> Self {
        Luv::from(color).into()
    }
}

impl From<Lch> for Hsva {
    fn from(color: Lch) -> Hsva {
        Color32::from(color).into()
    }
}

impl From<Hsva> for Lch {
    fn from(color: Hsva) -> Self {
        Luv::from(color).into()
    }
}

impl From<Lch> for Rgba {
    fn from(color: Lch) -> Self {
        Luv::from(color).into()
    }
}

impl From<Rgba> for Lch {
    fn from(color: Rgba) -> Self {
        Luv::from(color).into()
    }
}

//####################################################################################################

impl From<Color> for Lch {
    fn from(c: Color) -> Lch {
        match c {
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => Rgb::from(c).into(),
            Color::Luv(c) => c.into(),
            Color::Xyz(c) => Luv::from(c).into(),
            Color::Lch(c) => c,
            Color::Hsl(c) => Rgb::from(c).into(),
        }
    }
}

impl From<Cmyk> for Lch {
    fn from(color: Cmyk) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsl> for Lch {
    fn from(color: Hsl) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsv> for Lch {
    fn from(color: Hsv) -> Self {
        Rgb::from(color).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Luv> for Lch {
    fn from(color: Luv) -> Self {
        let u = color.u();
        let v = color.v();
        let c = (u.powi(2) + v.powi(2)).sqrt();
        let vu_atan = f32::atan2(v, u).to_degrees();
        let h = if vu_atan >= 0. {
            vu_atan
        } else {
            vu_atan + 360.
        };

        Lch::new(color.l(), c, h)
    }
}

impl From<Rgb> for Lch {
    fn from(color: Rgb) -> Self {
        Luv::from(Xyz::from(color)).into()
    }
}

impl From<Xyz> for Lch {
    fn from(color: Xyz) -> Self {
        Rgb::from(color).into()
    }
}
