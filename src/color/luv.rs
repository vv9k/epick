use crate::color::hsv::Hsv;
use crate::color::illuminant::Illuminant;
use crate::color::rgb::Rgb;
use crate::color::{Cmyk, Color, Hsl, Lch, Xyz, CIE_E, CIE_K};
use egui::color::{Color32, Hsva, Rgba};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Luv {
    pub l: f32,
    pub u: f32,
    pub v: f32,
}

impl Luv {
    pub fn new(l: f32, u: f32, v: f32) -> Self {
        let l = if l.is_nan() { 0. } else { l };
        let u = if u.is_nan() { 0. } else { u };
        let v = if v.is_nan() { 0. } else { v };

        Self { l, u, v }
    }

    #[inline(always)]
    /// Returns Light in the range of 0.0 ..= 1.
    pub fn l(&self) -> f32 {
        self.l
    }

    #[inline(always)]
    /// Returns U coordinate
    pub fn u(&self) -> f32 {
        self.u
    }

    #[inline(always)]
    /// Returns V coordinate
    pub fn v(&self) -> f32 {
        self.v
    }

    /// Returns Light scaled in the range of 0.0 ..= 100.0
    pub fn l_scaled(&self) -> f32 {
        self.l * 100.
    }
}

//####################################################################################################

impl From<Luv> for Color32 {
    fn from(color: Luv) -> Self {
        Xyz::from(color).into()
    }
}

impl From<Color32> for Luv {
    fn from(color: Color32) -> Self {
        Xyz::from(color).into()
    }
}

impl From<Luv> for Hsva {
    fn from(color: Luv) -> Self {
        Xyz::from(color).into()
    }
}

impl From<Hsva> for Luv {
    fn from(color: Hsva) -> Self {
        Xyz::from(color).into()
    }
}

impl From<Luv> for Rgba {
    fn from(color: Luv) -> Self {
        Xyz::from(color).into()
    }
}

impl From<Rgba> for Luv {
    fn from(color: Rgba) -> Self {
        Xyz::from(color).into()
    }
}

//####################################################################################################

impl From<Color> for Luv {
    fn from(c: Color) -> Luv {
        match c {
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => Rgb::from(c).into(),
            Color::Luv(c) => c,
            Color::Xyz(c) => c.into(),
            Color::Lch(c) => c.into(),
            Color::Hsl(c) => Rgb::from(c).into(),
        }
    }
}

impl From<&Color> for Luv {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Cmyk> for Luv {
    fn from(color: Cmyk) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsv> for Luv {
    fn from(color: Hsv) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsl> for Luv {
    fn from(color: Hsl) -> Self {
        Rgb::from(color).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Lch> for Luv {
    fn from(color: Lch) -> Self {
        let l = color.l();
        let c = color.c();
        let h = color.h();

        let u = c * h.to_radians().cos();
        let v = c * h.to_radians().sin();

        Luv::new(l, u, v)
    }
}

impl From<Rgb> for Luv {
    fn from(color: Rgb) -> Self {
        Xyz::from(color).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Xyz> for Luv {
    fn from(color: Xyz) -> Self {
        let y = color.y();
        let u = color.u();
        let v = color.v();

        let yr = y / Illuminant::D65.xyz().y();

        let l = if yr > CIE_E {
            116. * yr.cbrt() - 16.
        } else {
            CIE_K * yr
        };

        let u = 13. * l * (u - Illuminant::D65.reference_u());
        let v = 13. * l * (v - Illuminant::D65.reference_v());

        Luv::new(l, u, v)
    }
}
