use crate::color::rgb::Rgb;
use crate::color::{CIEColor, Illuminant, Lab, RgbWorkingSpace, Xyz};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LchAB {
    l: f32,
    c: f32,
    h: f32,
}

impl LchAB {
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
    /// Returns Hue in the range of 0.0 ..= 360.0
    pub fn h(&self) -> f32 {
        self.h
    }

    pub fn from_xyz(color: Xyz, reference_white: Illuminant) -> Self {
        Lab::from_xyz(color, reference_white).into()
    }

    pub fn to_xyz(self, reference_white: Illuminant) -> Xyz {
        Lab::from(self).to_xyz(reference_white)
    }
}

impl CIEColor for LchAB {
    fn to_rgb(self, ws: RgbWorkingSpace) -> Rgb {
        self.to_xyz(ws.reference_whitepoint()).to_rgb(ws)
    }

    fn from_rgb(rgb: Rgb, ws: RgbWorkingSpace) -> Self {
        Self::from_xyz(Xyz::from_rgb(rgb, ws), ws.reference_whitepoint())
    }
}

//####################################################################################################

#[allow(clippy::many_single_char_names)]
impl From<Lab> for LchAB {
    fn from(color: Lab) -> Self {
        let arctan_ba = f32::atan2(color.b(), color.a()).to_degrees();
        let l = color.l();
        let c = (color.a().powi(2) + color.b().powi(2)).sqrt();
        let h = if arctan_ba >= 0. {
            arctan_ba
        } else {
            arctan_ba + 360.
        };

        Self::new(l, c, h)
    }
}
