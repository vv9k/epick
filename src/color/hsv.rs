use crate::color::rgb::Rgb;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl Hsv {
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        Self {
            h: hue,
            s: saturation,
            v: value,
        }
    }
}

impl From<Rgb> for Hsv {
    fn from(_: Rgb) -> Self {
        todo!()
    }
}

impl From<Hsv> for Rgb {
    fn from(_: Hsv) -> Self {
        todo!()
    }
}

impl From<Hsv> for egui::color::Hsva {
    fn from(hsv: Hsv) -> Self {
        egui::color::Hsva {
            h: hsv.h,
            s: hsv.s,
            v: hsv.v,
            a: 1.,
        }
    }
}

impl From<egui::color::Hsva> for Hsv {
    fn from(hsv: egui::color::Hsva) -> Self {
        Self {
            h: hsv.h,
            s: hsv.s,
            v: hsv.v,
        }
    }
}
