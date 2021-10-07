use egui::color::{Color32, Hsva, Rgba};

use crate::color::{
    colorspace::{ColorSpace, InverseRgbSpaceMatrix, RgbSpaceMatrix},
    U8_MAX,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xyz {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn as_rgb(&self, space_matrix: RgbSpaceMatrix) -> Color32 {
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

        Rgba::from_rgb(r, g, b).into()
    }

    #[allow(clippy::many_single_char_names)]
    pub fn from_rgb(color: Color32, space_matrix: InverseRgbSpaceMatrix) -> Self {
        let r = color.r() as f32 / U8_MAX;
        let g = color.g() as f32 / U8_MAX;
        let b = color.b() as f32 / U8_MAX;

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
}

impl From<Xyz> for Rgba {
    fn from(xyz: Xyz) -> Rgba {
        Color32::from(xyz).into()
    }
}

impl From<Xyz> for Hsva {
    fn from(xyz: Xyz) -> Hsva {
        Color32::from(xyz).into()
    }
}

impl From<Rgba> for Xyz {
    fn from(rgba: Rgba) -> Xyz {
        Color32::from(rgba).into()
    }
}

impl From<Xyz> for Color32 {
    fn from(color: Xyz) -> Self {
        color.as_rgb(ColorSpace::SRGB.rgb_matrix())
    }
}

impl From<Color32> for Xyz {
    fn from(color: Color32) -> Self {
        Xyz::from_rgb(color, ColorSpace::SRGB.rgb_matrix_inverse())
    }
}
