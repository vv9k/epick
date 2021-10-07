use egui::color::{Color32, Hsva, Rgba};

use super::U8_MAX;

pub type RgbSpaceMatrix = [[f32; 3]; 3];

pub const ADOBE_RGB: RgbSpaceMatrix = [
    [0.5767309, 0.185554, 0.1881852],
    [0.2973769, 0.6273491, 0.0752741],
    [0.0270343, 0.0706872, 0.9911085],
];

pub const ADOBE_RGB_INVERSE: RgbSpaceMatrix = [
    [2.041369, -0.5649464, -0.3446944],
    [-0.969266, 1.8760108, 0.0415560],
    [0.0134474, -0.1183897, 1.0154096],
];

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

        Rgba::from_rgb(r, g, b).into()
    }

    #[allow(clippy::many_single_char_names)]
    pub fn from_rgb(color: Color32, space_matrix: RgbSpaceMatrix) -> Self {
        let r = color.r() as f32 / U8_MAX;
        let g = color.g() as f32 / U8_MAX;
        let b = color.b() as f32 / U8_MAX;

        let x = r * space_matrix[0][0] + g * space_matrix[0][1] + b * space_matrix[0][2];
        let y = r * space_matrix[1][0] + g * space_matrix[1][1] + b * space_matrix[1][2];
        let z = r * space_matrix[2][0] + g * space_matrix[2][1] + b * space_matrix[2][2];

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
        color.as_rgb(ADOBE_RGB_INVERSE)
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Color32> for Xyz {
    fn from(color: Color32) -> Self {
        Xyz::from_rgb(color, ADOBE_RGB)
    }
}
