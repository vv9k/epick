use crate::color::illuminant::Illuminant;
use crate::color::rgb::Rgb;
use crate::color::xyy::xyY;
use crate::color::{working_space::RgbWorkingSpace, CIEColor, Luv, CIE_E, CIE_K};

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

    pub fn x_scaled(&self) -> f32 {
        self.x * 100.
    }

    pub fn y_scaled(&self) -> f32 {
        self.y * 100.
    }

    pub fn z_scaled(&self) -> f32 {
        self.z * 100.
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

impl CIEColor for Xyz {
    fn to_rgb(self, working_space: RgbWorkingSpace) -> Rgb {
        let space_matrix = working_space.inverse_rgb_matrix().0;

        let r =
            self.x * space_matrix[0][0] + self.y * space_matrix[0][1] + self.z * space_matrix[0][2];
        let g =
            self.x * space_matrix[1][0] + self.y * space_matrix[1][1] + self.z * space_matrix[1][2];
        let b =
            self.x * space_matrix[2][0] + self.y * space_matrix[2][1] + self.z * space_matrix[2][2];

        let rgb = Rgb::new(r, g, b);
        working_space.compand_channels(rgb)
    }

    #[allow(clippy::many_single_char_names)]
    fn from_rgb(rgb: Rgb, working_space: RgbWorkingSpace) -> Self {
        let space_matrix = working_space.rgb_matrix().0;

        let rgb = working_space.inverse_compand_channels(rgb);

        let r = rgb.r();
        let g = rgb.g();
        let b = rgb.b();

        let x = r * space_matrix[0][0] + g * space_matrix[0][1] + b * space_matrix[0][2];
        let y = r * space_matrix[1][0] + g * space_matrix[1][1] + b * space_matrix[1][2];
        let z = r * space_matrix[2][0] + g * space_matrix[2][1] + b * space_matrix[2][2];

        Self { x, y, z }
    }
}

//####################################################################################################

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

impl From<xyY> for Xyz {
    fn from(color: xyY) -> Self {
        let x = color.x();
        let y = color.y();
        let yy = color.yy();

        if y == 0. {
            return Self {
                x: 0.,
                y: 0.,
                z: 0.,
            };
        }

        let xx = x * yy / y;
        let zz = (1. - x - y) * yy / y;

        Self::new(xx, yy, zz)
    }
}

//####################################################################################################

#[cfg(test)]
mod tests {
    use super::{CIEColor, Rgb, RgbWorkingSpace, Xyz};

    #[test]
    fn rgb_to_xyz() {
        macro_rules! test_case {
            ($ws:expr; Rgb: $r:expr, $g:expr, $b:expr; Xyz: $x:expr, $y:expr, $z:expr) => {
                let expected = Xyz::new($x, $y, $z);
                let got = Xyz::from_rgb(Rgb::new($r, $g, $b), $ws);
                assert_eq!(got, expected);
            };
        }

        test_case!(
            RgbWorkingSpace::SRGB;
            Rgb: 0., 0., 0.;
            Xyz: 0., 0., 0.
        );
        test_case!(
            RgbWorkingSpace::SRGB;
            Rgb: 1., 1., 1.;
            Xyz: 0.95047003, 1.0000001, 1.08883
        );
    }
}
