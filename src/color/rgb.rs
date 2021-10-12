#![allow(clippy::many_single_char_names)]
use crate::color::colorspace::ColorSpace;
use crate::color::hsv::Hsv;
use crate::color::{Cmyk, Color, Hsl, Lch, Luv, Xyz, U8_MAX};
use egui::color::{Hsva, HsvaGamma};
use egui::{Color32, Rgba};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Rgb {
    /// Takes in values in the range 0.0 ..= 1.0 and returns an RGB color.
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        let red = if red.is_nan() {
            0.
        } else if red > 1. {
            red / U8_MAX
        } else {
            red
        };
        let green = if green.is_nan() {
            0.
        } else if green > 1. {
            green / U8_MAX
        } else {
            green
        };
        let blue = if blue.is_nan() {
            0.
        } else if blue > 1. {
            blue / U8_MAX
        } else {
            blue
        };

        Self {
            r: red,
            g: green,
            b: blue,
        }
    }

    #[inline(always)]
    /// Returns Red value in the range 0.0 ..= 1.0
    pub fn r(&self) -> f32 {
        self.r
    }

    #[inline(always)]
    /// Returns Green value in the range 0.0 ..= 1.0
    pub fn g(&self) -> f32 {
        self.g
    }

    #[inline(always)]
    /// Returns Blue value in the range 0.0 ..= 1.0
    pub fn b(&self) -> f32 {
        self.b
    }

    #[inline(always)]
    /// Returns Red value in the range 0.0 ..= 255.0
    pub fn r_scaled(&self) -> f32 {
        self.r * U8_MAX
    }

    #[inline(always)]
    /// Returns Green value in the range 0.0 ..= 255.0
    pub fn g_scaled(&self) -> f32 {
        self.g * U8_MAX
    }

    #[inline(always)]
    /// Returns Blue value in the range 0.0 ..= 255.0
    pub fn b_scaled(&self) -> f32 {
        self.b * U8_MAX
    }
}

//####################################################################################################

impl From<Rgb> for Color32 {
    fn from(rgb: Rgb) -> Self {
        egui::Rgba::from(rgb).into()
    }
}

impl From<Color32> for Rgb {
    fn from(color: Color32) -> Self {
        Rgba::from(color).into()
    }
}

impl From<Rgb> for Hsva {
    fn from(rgb: Rgb) -> Self {
        Hsv::from(rgb).into()
    }
}

impl From<Hsva> for Rgb {
    fn from(color: Hsva) -> Self {
        Rgba::from(color).into()
    }
}

impl From<Rgb> for HsvaGamma {
    fn from(rgb: Rgb) -> Self {
        Hsva::from(Hsv::from(rgb)).into()
    }
}

impl From<Rgba> for Rgb {
    fn from(rgb: Rgba) -> Self {
        Rgb::new(rgb.r(), rgb.g(), rgb.b())
    }
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Self {
        Rgba::from_rgb(rgb.r(), rgb.g(), rgb.b())
    }
}

//####################################################################################################

impl From<Color> for Rgb {
    fn from(c: Color) -> Rgb {
        match c {
            Color::Cmyk(c) => c.into(),
            Color::Rgb(c) => c,
            Color::Hsv(c) => c.into(),
            Color::Luv(c) => c.into(),
            Color::Xyz(c) => c.into(),
            Color::Lch(c) => c.into(),
            Color::Hsl(c) => c.into(),
        }
    }
}

impl From<&Color> for Rgb {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Cmyk> for Rgb {
    fn from(cmyk: Cmyk) -> Self {
        let k = cmyk.k();
        let r = (1. - cmyk.c()) * (1. - k);
        let g = (1. - cmyk.m()) * (1. - k);
        let b = (1. - cmyk.y()) * (1. - k);
        Rgb::new(r, g, b)
    }
}

impl From<Hsl> for Rgb {
    fn from(hsl: Hsl) -> Rgb {
        Hsv::from(hsl).into()
    }
}

impl From<Hsv> for Rgb {
    #[rustfmt::skip]
    fn from(hsv: Hsv) -> Self {
        let h = hsv.h();
        let h = (h.fract() + 1.).fract() * 6.;
        let s = hsv.s();
        let v = hsv.v();

        let f = h - h.floor();
        let p = v * (1. - s);
        let q = v * (1. - f * s);
        let t = v * (1. - (1. - f) * s);

         match h.floor() as i32 % 6 {
            0 => Rgb::new(v,  t,  p  ),
            1 => Rgb::new(q,  v,  p  ),
            2 => Rgb::new(p,  v,  t  ),
            3 => Rgb::new(p,  q,  v  ),
            4 => Rgb::new(t,  p,  v  ),
            5 => Rgb::new(v,  p,  q  ),
            _ => Rgb::new(0., 0., 0. ),
        }
    }
}

impl From<Lch> for Rgb {
    fn from(lch: Lch) -> Rgb {
        Xyz::from(Luv::from(lch)).into()
    }
}

impl From<Luv> for Rgb {
    fn from(luv: Luv) -> Rgb {
        Xyz::from(luv).into()
    }
}

impl From<Xyz> for Rgb {
    fn from(color: Xyz) -> Self {
        color.as_rgb(ColorSpace::SRGB.rgb_matrix())
    }
}

//####################################################################################################

#[cfg(test)]
mod tests {
    use crate::color::{Cmyk, Hsv, Rgb};

    #[test]
    fn cmyk_to_rgb() {
        macro_rules! test_case {
            (Cmyk: $c:expr, $m:expr, $y:expr, $k:expr ;Rgb: $r:expr, $g:expr, $b:expr) => {
                let cmyk = Cmyk::new($c, $m, $y, $k);
                let expected = Rgb::new($r, $g, $b);
                let got = Rgb::from(cmyk);
                assert_eq!(got, expected);
            };
        }

        test_case!(
            Cmyk: 0., 0., 0., 1.;
            Rgb: 0., 0., 0.
        );
        test_case!(
            Cmyk: 0., 0., 0., 0.;
            Rgb: 1., 1., 1.
        );
        test_case!(
            Cmyk: 0.93, 0.33, 0., 0.;
            Rgb: 0.06999999, 0.66999996, 1.
        );
        test_case!(
            Cmyk: 0.08, 0., 0.06, 0.27;
            Rgb: 0.67160004, 0.73, 0.6862
        );
    }

    #[test]
    fn hsv_to_rgb() {
        macro_rules! test_case {
            (Hsv: $h:expr, $s:expr, $v:expr ;Rgb: $r:expr, $g:expr, $b:expr) => {
                let hsv = Hsv::new($h, $s, $v);
                let expected = Rgb::new($r, $g, $b);
                let got = Rgb::from(hsv);
                assert_eq!(got, expected);
            };
        }

        test_case!(
            Hsv: 0., 0., 0.;
            Rgb: 0., 0., 0.
        );
        test_case!(
            Hsv: 0.5, 0., 0.;
            Rgb: 0., 0., 0.
        );
        test_case!(
            Hsv: 1., 0., 0.;
            Rgb: 0., 0., 0.
        );
        test_case!(
            Hsv: 1., 1., 0.;
            Rgb: 0., 0., 0.
        );
        test_case!(
            Hsv: 0., 1., 1.;
            Rgb: 255., 0., 0.
        );
        test_case!(
            Hsv: 1./6., 1., 1.;
            Rgb: 255., 255., 0.
        );
        test_case!(
            Hsv: 1./3., 1., 1.;
            Rgb: 0., 255., 0.
        );
        test_case!(
            Hsv: 1./2., 1., 1.;
            Rgb: 0., 255., 255.
        );
        test_case!(
            Hsv: 2./3., 1., 1.;
            Rgb: 0., 0., 255.
        );
        test_case!(
            Hsv: 1., 1., 1.;
            Rgb: 255., 0., 0.
        );
    }
}
