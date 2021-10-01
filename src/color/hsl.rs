use egui::color::{Color32, Hsva, Rgba};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl Hsl {
    pub fn new(hue: f32, saturation: f32, light: f32) -> Self {
        Self {
            h: hue,
            s: saturation,
            l: light,
        }
    }
}

impl From<Hsl> for Rgba {
    fn from(hsl: Hsl) -> Rgba {
        Color32::from(hsl).into()
    }
}

impl From<Rgba> for Hsl {
    fn from(rgba: Rgba) -> Hsl {
        Color32::from(rgba).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Hsva> for Hsl {
    fn from(color: Hsva) -> Self {
        let h = color.h;
        let s = color.s;
        let v = color.v;

        let mut l = (2. - s) * v;
        let mut ss = s * v;
        if l <= 1. {
            ss /= l;
        } else {
            ss /= 2. - l;
        }
        l /= 2.;

        Hsl {
            h: if h.is_nan() { 0. } else { h },
            s: if ss.is_nan() { 0. } else { ss },
            l: if l.is_nan() { 0. } else { l },
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Hsl> for Hsva {
    fn from(color: Hsl) -> Self {
        let h = color.h;
        let mut ss = color.s;
        let l = color.l * 2.;

        if l <= 1. {
            ss *= l;
        } else {
            ss *= 2. - l;
        }

        let v = (l + ss) / 2.;
        let s = (2. * ss) / (l + ss);

        Hsva {
            h: if h.is_nan() { 0. } else { h },
            s: if s.is_nan() { 0. } else { s },
            v: if v.is_nan() { 0. } else { v },
            a: 1.,
        }
    }
}

impl From<Hsl> for Color32 {
    fn from(color: Hsl) -> Self {
        Hsva::from(color).into()
    }
}

impl From<Color32> for Hsl {
    fn from(color: Color32) -> Self {
        Hsva::from(color).into()
    }
}
