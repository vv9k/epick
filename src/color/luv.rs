use crate::color::illuminant::Illuminant;
use crate::color::rgb::Rgb;
use crate::color::{CIEColor, Lch, RgbWorkingSpace, Xyz, CIE_E, CIE_K};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Luv {
    l: f32,
    u: f32,
    v: f32,
}

impl Luv {
    pub fn new(l: f32, u: f32, v: f32) -> Self {
        let l = if l.is_nan() { 0. } else { l };
        let u = if u.is_nan() { 0. } else { u };
        let v = if v.is_nan() { 0. } else { v };

        Self { l, u, v }
    }

    #[inline(always)]
    /// Returns Light
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
}

impl CIEColor for Luv {
    fn to_rgb(self, ws: RgbWorkingSpace) -> Rgb {
        Xyz::from(self).to_rgb(ws)
    }

    fn from_rgb(rgb: Rgb, ws: RgbWorkingSpace) -> Self {
        Xyz::from_rgb(rgb, ws).into()
    }
}

//####################################################################################################

#[allow(clippy::many_single_char_names)]
impl From<Lch> for Luv {
    fn from(color: Lch) -> Self {
        let l = color.l();
        let c = color.c();
        let h = color.h().to_radians();

        let u = c * h.cos();
        let v = c * h.sin();

        Luv::new(l, u, v)
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
