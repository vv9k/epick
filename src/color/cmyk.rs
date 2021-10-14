use egui::color::{Color32, Hsva, Rgba};

use crate::color::hsv::Hsv;
use crate::color::rgb::Rgb;
use crate::color::{CIEColor, Color, Hsl};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Cmyk {
    c: f32,
    m: f32,
    y: f32,
    k: f32,
}

impl Cmyk {
    pub fn new(c: f32, m: f32, y: f32, k: f32) -> Self {
        let c = if c.is_nan() {
            0.
        } else if c > 1. {
            c / 100.
        } else {
            c
        };
        let m = if m.is_nan() {
            0.
        } else if m > 1. {
            m / 100.
        } else {
            m
        };
        let y = if y.is_nan() {
            0.
        } else if y > 1. {
            y / 100.
        } else {
            y
        };
        let k = if k.is_nan() {
            0.
        } else if k > 1. {
            k / 100.
        } else {
            k
        };
        Self { c, m, y, k }
    }

    #[inline(always)]
    /// Returns Cyan value in the range of 0.0 ..= 1.0
    pub fn c(&self) -> f32 {
        self.c
    }

    #[inline(always)]
    /// Returns Magenta value in the range of 0.0 ..= 1.0
    pub fn m(&self) -> f32 {
        self.m
    }

    #[inline(always)]
    /// Returns Yellow value in the range of 0.0 ..= 1.0
    pub fn y(&self) -> f32 {
        self.y
    }

    #[inline(always)]
    /// Returns Key value in the range of 0.0 ..= 1.0
    pub fn k(&self) -> f32 {
        self.k
    }

    /// Returns Cyan value in the range of 0.0 ..= 100.0
    pub fn c_scaled(&self) -> f32 {
        self.c * 100.
    }

    /// Returns Magenta value in the range of 0.0 ..= 100.0
    pub fn m_scaled(&self) -> f32 {
        self.m * 100.
    }

    /// Returns Yellow value in the range of 0.0 ..= 100.0
    pub fn y_scaled(&self) -> f32 {
        self.y * 100.
    }

    /// Returns Key value in the range of 0.0 ..= 100.0
    pub fn k_scaled(&self) -> f32 {
        self.k * 100.
    }
}

//####################################################################################################

impl From<Cmyk> for Hsva {
    fn from(color: Cmyk) -> Hsva {
        Rgb::from(color).into()
    }
}

impl From<Hsva> for Cmyk {
    fn from(color: Hsva) -> Cmyk {
        Rgb::from(color).into()
    }
}

impl From<Color32> for Cmyk {
    fn from(color: Color32) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Cmyk> for Color32 {
    fn from(color: Cmyk) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Cmyk> for Rgba {
    fn from(color: Cmyk) -> Rgba {
        Rgb::from(color).into()
    }
}

impl From<Rgba> for Cmyk {
    fn from(color: Rgba) -> Cmyk {
        Rgb::from(color).into()
    }
}

//####################################################################################################

impl From<Color> for Cmyk {
    fn from(c: Color) -> Cmyk {
        match c {
            Color::Rgb(c) => c.into(),
            Color::Cmyk(c) => c,
            Color::Hsv(c) => Rgb::from(c).into(),
            Color::Hsl(c) => Rgb::from(c).into(),
            Color::Xyz(c, ws) => c.to_rgb(ws).into(),
            Color::Luv(c, ws) => c.to_rgb(ws).into(),
            Color::LchUV(c, ws) => c.to_rgb(ws).into(),
            Color::Lab(c, ws) => c.to_rgb(ws).into(),
        }
    }
}

impl From<&Color> for Cmyk {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Hsl> for Cmyk {
    fn from(color: Hsl) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsv> for Cmyk {
    fn from(color: Hsv) -> Self {
        Rgb::from(color).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Rgb> for Cmyk {
    fn from(color: Rgb) -> Self {
        let r: f32 = color.r();
        let g: f32 = color.g();
        let b: f32 = color.b();
        let rgb = [r, g, b];
        let k = 1. - rgb.iter().copied().fold(f32::NAN, f32::max);

        if (k - 1.).abs() < f32::EPSILON {
            return Cmyk::new(0., 0., 0., k);
        }

        let c = (1. - r - k) / (1. - k);
        let m = (1. - g - k) / (1. - k);
        let y = (1. - b - k) / (1. - k);

        Cmyk::new(c, m, y, k)
    }
}
