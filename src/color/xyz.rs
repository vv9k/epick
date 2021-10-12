use egui::color::{Color32, Hsva, Rgba};

use crate::color::hsv::Hsv;
use crate::color::illuminant::Illuminant;
use crate::color::rgb::Rgb;
use crate::color::{
    colorspace::{ColorSpace, InverseRgbSpaceMatrix, RgbSpaceMatrix},
    Cmyk, Color, Hsl, Luv, CIE_E, CIE_K, U8_MAX,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xyz {
    x: f32,
    y: f32,
    z: f32,
}

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let x = if x.is_nan() { 0. } else { x };
        let y = if y.is_nan() { 0. } else { y };
        let z = if z.is_nan() { 0. } else { z };

        Self { x, y, z }
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.y
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn as_rgb(&self, space_matrix: RgbSpaceMatrix) -> Rgb {
        let r =
            self.x * space_matrix[0][0] + self.y * space_matrix[0][1] + self.z * space_matrix[0][2];
        let g =
            self.x * space_matrix[1][0] + self.y * space_matrix[1][1] + self.z * space_matrix[1][2];
        let b =
            self.x * space_matrix[2][0] + self.y * space_matrix[2][1] + self.z * space_matrix[2][2];

        fn srgb_compand(num: f32) -> f32 {
            if num <= 0.0031308 {
                num * 12.92
            } else {
                1.055 * num.powf(1. / 2.4) - 0.055
            }
        }

        let r = srgb_compand(r);
        let g = srgb_compand(g);
        let b = srgb_compand(b);

        Rgb::new(r, g, b)
    }

    #[allow(clippy::many_single_char_names)]
    pub fn from_rgb(color: (f32, f32, f32), space_matrix: InverseRgbSpaceMatrix) -> Self {
        let r = color.0;
        let g = color.1;
        let b = color.2;
        fn inverse_srgb_compand(num: f32) -> f32 {
            if num <= 0.04045 {
                num / 12.92
            } else {
                ((num + 0.055) / 1.055).powf(2.4)
            }
        }

        let r = inverse_srgb_compand(r);
        let g = inverse_srgb_compand(g);
        let b = inverse_srgb_compand(b);

        let x = (r * space_matrix[0][0] + g * space_matrix[0][1] + b * space_matrix[0][2]) * 100.;
        let y = (r * space_matrix[1][0] + g * space_matrix[1][1] + b * space_matrix[1][2]) * 100.;
        let z = (r * space_matrix[2][0] + g * space_matrix[2][1] + b * space_matrix[2][2]) * 100.;

        Xyz { x, y, z }
    }

    #[inline(always)]
    pub fn u(&self) -> f32 {
        4. * self.x / (self.x + 15. * self.y + 3. * self.z)
    }

    #[inline(always)]
    pub fn v(&self) -> f32 {
        9. * self.y / (self.x + 15. * self.y + 3. * self.z)
    }
}

//####################################################################################################

impl From<Color> for Xyz {
    fn from(c: Color) -> Xyz {
        match c {
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => Rgb::from(c).into(),
            Color::Luv(c) => Rgb::from(c).into(),
            Color::Xyz(c) => c,
            Color::Lch(c) => Rgb::from(c).into(),
            Color::Hsl(c) => Rgb::from(c).into(),
        }
    }
}

impl From<Xyz> for Color32 {
    fn from(color: Xyz) -> Self {
        color.as_rgb(ColorSpace::SRGB.rgb_matrix()).into()
    }
}

impl From<Color32> for Xyz {
    fn from(color: Color32) -> Self {
        let r = color.r() as f32 / U8_MAX;
        let g = color.g() as f32 / U8_MAX;
        let b = color.b() as f32 / U8_MAX;
        let color = (r, g, b);
        Xyz::from_rgb(color, ColorSpace::SRGB.rgb_matrix_inverse())
    }
}

impl From<Xyz> for Hsva {
    fn from(color: Xyz) -> Hsva {
        Color32::from(color).into()
    }
}

impl From<Hsva> for Xyz {
    fn from(color: Hsva) -> Xyz {
        Color32::from(color).into()
    }
}

impl From<Xyz> for Rgba {
    fn from(color: Xyz) -> Rgba {
        Color32::from(color).into()
    }
}

impl From<Rgba> for Xyz {
    fn from(color: Rgba) -> Xyz {
        Color32::from(color).into()
    }
}

//####################################################################################################

impl From<Cmyk> for Xyz {
    fn from(color: Cmyk) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsl> for Xyz {
    fn from(color: Hsl) -> Self {
        Rgb::from(color).into()
    }
}

impl From<Hsv> for Xyz {
    fn from(color: Hsv) -> Self {
        Rgb::from(color).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Luv> for Xyz {
    fn from(color: Luv) -> Self {
        let l = color.l();
        let u = color.u();
        let v = color.v();

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

        Xyz::new(x, y, z)
    }
}

impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Self {
        Xyz::from_rgb(
            (rgb.r(), rgb.g(), rgb.b()),
            ColorSpace::SRGB.rgb_matrix_inverse(),
        )
    }
}
