use crate::color::hsv::Hsv;
use crate::color::rgb::Rgb;
use crate::color::{CIEColor, Cmyk, Color};
use egui::color::{Color32, Hsva, Rgba};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
    h: f32,
    s: f32,
    l: f32,
}

impl Hsl {
    /// Takes in values in the range 0.0 ..= 1.0 and returns an HSL color
    pub fn new(hue: f32, saturation: f32, light: f32) -> Self {
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
        let light = if light.is_nan() {
            0.
        } else if light > 1. {
            light / 100.
        } else {
            light
        };
        Self {
            h: hue,
            s: saturation,
            l: light,
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
    /// Returns Light in the range of 0.0 ..= 1.0
    pub fn l(&self) -> f32 {
        self.l
    }

    /// Returns Hue in the range of 0.0 ..= 360.0
    pub fn h_scaled(&self) -> f32 {
        self.h * 360.
    }

    /// Returns Saturation in the range of 0.0 ..= 100.0
    pub fn s_scaled(&self) -> f32 {
        self.s * 100.
    }

    /// Returns Light in the range of 0.0 ..= 100.0
    pub fn l_scaled(&self) -> f32 {
        self.l * 100.
    }
}

//####################################################################################################

impl From<Hsl> for Color32 {
    fn from(color: Hsl) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Color32> for Hsl {
    fn from(color: Color32) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Hsl> for Hsva {
    fn from(color: Hsl) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Hsva> for Hsl {
    fn from(color: Hsva) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Hsl> for Rgba {
    fn from(color: Hsl) -> Self {
        Hsv::from(color).into()
    }
}

impl From<Rgba> for Hsl {
    fn from(color: Rgba) -> Self {
        Hsv::from(color).into()
    }
}

//####################################################################################################

impl From<Color> for Hsl {
    fn from(c: Color) -> Hsl {
        match c {
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => c.into(),
            Color::Luv(c, ws) => c.to_rgb(ws).into(),
            Color::Xyz(c, ws) => c.to_rgb(ws).into(),
            Color::Lch(c, ws) => c.to_rgb(ws).into(),
            Color::Hsl(c) => c,
        }
    }
}

impl From<&Color> for Hsl {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Cmyk> for Hsl {
    fn from(color: Cmyk) -> Self {
        Hsv::from(color).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Hsv> for Hsl {
    fn from(color: Hsv) -> Self {
        let h = color.h();
        let s = color.s();
        let v = color.v();

        let mut l = (2. - s) * v;
        let mut ss = s * v;
        if l <= 1. {
            ss /= l;
        } else {
            ss /= 2. - l;
        }
        l /= 2.;

        Hsl::new(h, ss, l)
    }
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Hsl {
        Hsv::from(rgb).into()
    }
}
