use crate::color::illuminant::Illuminant;
use crate::color::{Xyz, CIE_E, CIE_K};
use egui::color::{Color32, Hsva, Rgba};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Luv {
    pub l: f32,
    pub u: f32,
    pub v: f32,
}

impl Luv {
    pub fn new(l: f32, u: f32, v: f32) -> Self {
        Self { l, u, v }
    }
}

impl From<Luv> for Rgba {
    fn from(luv: Luv) -> Rgba {
        Color32::from(luv).into()
    }
}

impl From<Luv> for Hsva {
    fn from(luv: Luv) -> Hsva {
        Color32::from(luv).into()
    }
}

impl From<Rgba> for Luv {
    fn from(rgba: Rgba) -> Luv {
        Color32::from(rgba).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Xyz> for Luv {
    fn from(color: Xyz) -> Self {
        let y = color.y;

        let u = color.u();
        let v = color.v();

        let yr = y / Illuminant::D65.xyz().y;

        let l = if yr > CIE_E {
            116. * yr.cbrt() - 16.
        } else {
            CIE_K * yr
        };

        let u = 13. * l * (u - Illuminant::D65.reference_u());
        let v = 13. * l * (v - Illuminant::D65.reference_v());

        Luv {
            l: if l.is_nan() { 0. } else { l },
            u: if u.is_nan() { 0. } else { u },
            v: if v.is_nan() { 0. } else { v },
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Luv> for Xyz {
    fn from(color: Luv) -> Self {
        let l = color.l;
        let u = color.u;
        let v = color.v;

        let y = if l > CIE_K * CIE_E {
            ((l + 16.) / 116.).powi(3)
        } else {
            l / CIE_K
        };

        let a = ((52. * l / (u + 13. * l * Illuminant::D65.reference_u())) - 1.) / 3.;
        let b = -5. * y;
        let c = -(1.0f32 / 3.);
        let d = y * ((39. * l / (v + 13. * l * Illuminant::D65.reference_v())) - 5.);

        let x = (d - b) / (a - c);
        let z = x * a + b;

        Xyz {
            x: if x.is_nan() { 0. } else { x },
            y: if y.is_nan() { 0. } else { y },
            z: if z.is_nan() { 0. } else { z },
        }
    }
}

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
