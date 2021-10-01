use crate::color::Luv;
use egui::color::{Color32, Hsva, Rgba};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

impl Lch {
    pub fn new(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h }
    }
}

impl From<Lch> for Rgba {
    fn from(lch: Lch) -> Rgba {
        Color32::from(lch).into()
    }
}

impl From<Lch> for Hsva {
    fn from(lch: Lch) -> Hsva {
        Color32::from(lch).into()
    }
}

impl From<Rgba> for Lch {
    fn from(rgba: Rgba) -> Lch {
        Color32::from(rgba).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Luv> for Lch {
    fn from(color: Luv) -> Self {
        let u = color.u;
        let v = color.v;
        let c = (u.powi(2) + v.powi(2)).sqrt();
        let vu_atan = f32::atan2(v, u);
        let h = if vu_atan >= 0. {
            vu_atan
        } else {
            vu_atan + 360.
        };

        Lch {
            l: color.l,
            c: if c.is_nan() { 0. } else { c },
            h: if h.is_nan() { 0. } else { h },
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Lch> for Luv {
    fn from(color: Lch) -> Self {
        let c = color.c;
        let h = color.h;

        let u = c * h.to_radians().cos();
        let v = c * h.to_radians().sin();

        Luv {
            l: if color.l.is_nan() { 0. } else { color.l },
            u: if u.is_nan() { 0. } else { u },
            v: if v.is_nan() { 0. } else { v },
        }
    }
}

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
