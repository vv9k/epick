#![allow(dead_code)]
#![allow(clippy::excessive_precision)]
use super::illuminant::Illuminant;
use crate::color::Rgb;

pub type RgbSpaceMatrix = [[f32; 3]; 3];
pub type InverseRgbSpaceMatrix = [[f32; 3]; 3];

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum RgbWorkingSpace {
    Adobe,
    Apple,
    CIE,
    ECI,
    NTSC,
    PAL,
    ProPhoto,
    SRGB,
    WideGamut,
}

impl RgbWorkingSpace {
    pub fn reference_whitepoint(&self) -> Illuminant {
        use RgbWorkingSpace::*;
        match &self {
            Adobe => Illuminant::D65,
            Apple => Illuminant::D65,
            CIE => Illuminant::E,
            ECI => Illuminant::D50,
            NTSC => Illuminant::C,
            PAL => Illuminant::D65,
            ProPhoto => Illuminant::D50,
            SRGB => Illuminant::D65,
            WideGamut => Illuminant::D50,
        }
    }

    pub fn rgb_matrix(&self) -> RgbSpaceMatrix {
        use RgbWorkingSpace::*;
        match &self {
            Adobe => ADOBE_RGB,
            Apple => APPLE_RGB,
            CIE => CIE_RGB,
            ECI => ECI_RGB,
            NTSC => NTSC_RGB,
            PAL => PAL_RGB,
            ProPhoto => PRO_PHOTO_RGB,
            Self::SRGB => crate::color::working_space::SRGB,
            WideGamut => WIDE_GAMUT_RGB,
        }
    }

    pub fn inverse_rgb_matrix(&self) -> RgbSpaceMatrix {
        use RgbWorkingSpace::*;
        match &self {
            Adobe => ADOBE_RGB_INVERSE,
            Apple => APPLE_RGB_INVERSE,
            CIE => CIE_RGB_INVERSE,
            ECI => ECI_RGB_INVERSE,
            NTSC => NTSC_RGB_INVERSE,
            PAL => PAL_RGB_INVERSE,
            ProPhoto => PRO_PHOTO_RGB_INVERSE,
            SRGB => SRGB_INVERSE,
            WideGamut => WIDE_GAMUT_RGB_INVERSE,
        }
    }

    pub fn gamma(&self) -> f32 {
        use RgbWorkingSpace::*;
        match &self {
            Adobe => 2.2,
            Apple => 1.8,
            CIE => 2.2,
            ECI => 3.,
            NTSC => 2.2,
            PAL => 2.2,
            ProPhoto => 1.8,
            SRGB => 2.2,
            WideGamut => 2.2,
        }
    }

    pub fn compand_channels(&self, color: Rgb) -> Rgb {
        use RgbWorkingSpace::*;
        match &self {
            Adobe | Apple | CIE | NTSC | PAL | ProPhoto | WideGamut => {
                color.gamma_compand(self.gamma())
            }
            ECI => color.l_compand(),
            SRGB => color.srgb_compand(),
        }
    }

    pub fn inverse_compand_channels(&self, color: Rgb) -> Rgb {
        use RgbWorkingSpace::*;
        match &self {
            Adobe | Apple | CIE | NTSC | PAL | ProPhoto | WideGamut => {
                color.inverse_gamma_compand(self.gamma())
            }
            ECI => color.inverse_l_compand(),
            SRGB => color.inverse_srgb_compand(),
        }
    }
}

//####################################################################################################

const ADOBE_RGB: RgbSpaceMatrix = [
    [0.5767309, 0.185554, 0.1881852],
    [0.2973769, 0.6273491, 0.0752741],
    [0.0270343, 0.0706872, 0.9911085],
];

const APPLE_RGB: RgbSpaceMatrix = [
    [0.4497288, 0.3162486, 0.1844926],
    [0.2446525, 0.6720283, 0.0833192],
    [0.0251848, 0.1411824, 0.9224628],
];
const BEST_RGB: RgbSpaceMatrix = [
    [0.6326696, 0.2045558, 0.1269946],
    [0.2284569, 0.7373523, 0.0341908],
    [0.0000000, 0.0095142, 0.8156958],
];

const BETA_RGB: RgbSpaceMatrix = [
    [0.6712537, 0.1745834, 0.1183829],
    [0.3032726, 0.6637861, 0.0329413],
    [0.0000000, 0.0407010, 0.784509],
];
const BRUCE_RGB: RgbSpaceMatrix = [
    [0.4674162, 0.2944512, 0.1886026],
    [0.2410115, 0.6835475, 0.0754410],
    [0.0219101, 0.0736128, 0.9933071],
];
const CIE_RGB: RgbSpaceMatrix = [
    [0.488718, 0.3106803, 0.2006017],
    [0.1762044, 0.8129847, 0.0108109],
    [0.0000000, 0.0102048, 0.9897952],
];
const COLOR_MATCH_RGB: RgbSpaceMatrix = [
    [0.5093439, 0.3209071, 0.1339691],
    [0.274884, 0.6581315, 0.0669845],
    [0.0242545, 0.1087821, 0.6921735],
];
const DON_RGB_4: RgbSpaceMatrix = [
    [0.6457711, 0.1933511, 0.1250978],
    [0.2783496, 0.6879702, 0.0336802],
    [0.0037113, 0.0179861, 0.8035125],
];
const ECI_RGB: RgbSpaceMatrix = [
    [0.6502043, 0.1780774, 0.1359384],
    [0.3202499, 0.6020711, 0.0776791],
    [0.0000000, 0.067839, 0.757371],
];
const EKTA_SPACE_PS5: RgbSpaceMatrix = [
    [0.5938914, 0.2729801, 0.0973485],
    [0.2606286, 0.7349465, 0.0044249],
    [0.0000000, 0.0419969, 0.7832131],
];
const NTSC_RGB: RgbSpaceMatrix = [
    [0.6068909, 0.1735011, 0.200348],
    [0.2989164, 0.586599, 0.1144845],
    [0.0000000, 0.0660957, 1.1162243],
];
const PAL_RGB: RgbSpaceMatrix = [
    [0.430619, 0.3415419, 0.1783091],
    [0.2220379, 0.7066384, 0.0713236],
    [0.0201853, 0.1295504, 0.9390944],
];
const PRO_PHOTO_RGB: RgbSpaceMatrix = [
    [0.7976749, 0.1351917, 0.0313534],
    [0.2880402, 0.7118741, 0.0000857],
    [0.0000000, 0.0000000, 0.82521],
];
const SMPTEC_RGB: RgbSpaceMatrix = [
    [0.3935891, 0.3652497, 0.1916313],
    [0.2124132, 0.7010437, 0.0865432],
    [0.0187423, 0.1119313, 0.9581563],
];
const SRGB: RgbSpaceMatrix = [
    [0.4124564, 0.3575761, 0.1804375],
    [0.2126729, 0.7151522, 0.0721750],
    [0.0193339, 0.119192, 0.9503041],
];
const WIDE_GAMUT_RGB: RgbSpaceMatrix = [
    [0.7161046, 0.1009296, 0.1471858],
    [0.2581874, 0.7249378, 0.0168748],
    [0.0000000, 0.0517813, 0.7734287],
];

const ADOBE_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [2.041369, -0.5649464, -0.3446944],
    [-0.969266, 1.8760108, 0.0415560],
    [0.0134474, -0.1183897, 1.0154096],
];
const APPLE_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [2.9515373, -1.2894116, -0.4738445],
    [-1.0851093, 1.9908566, 0.0372026],
    [0.0854934, -0.2694964, 1.0912975],
];
const BEST_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [1.7552599, -0.4836786, -0.253],
    [-0.5441336, 1.5068789, 0.0215528],
    [0.0063467, -0.0175761, 1.2256959],
];
const BETA_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [1.6832270, -0.4282363, -0.2360185],
    [-0.7710229, 1.7065571, 0.04469],
    [0.0400013, -0.0885376, 1.272364],
];
const BRUCE_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [2.7454669, -1.1358136, -0.4350269],
    [-0.969266, 1.8760108, 0.041556],
    [0.0112723, -0.1139754, 1.0132541],
];
const CIE_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [2.3706743, -0.9000405, -0.4706338],
    [-0.513885, 1.4253036, 0.0885814],
    [0.0052982, -0.0146949, 1.0093968],
];
const COLOR_MATCH_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [2.6422874, -1.2234270, -0.3930143],
    [-1.1119763, 2.0590183, 0.0159614],
    [0.0821699, -0.2807254, 1.4559877],
];
const DON_RGB_4_INVERSE: InverseRgbSpaceMatrix = [
    [1.7603902, -0.4881198, -0.2536126],
    [-0.7126288, 1.6527432, 0.0416715],
    [0.0078207, -0.0347411, 1.2447743],
];
const ECI_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [1.7827618, -0.4969847, -0.2690101],
    [-0.9593623, 1.9477962, -0.0275807],
    [0.0859317, -0.1744674, 1.3228273],
];
const EKTA_SPACE_PS5_INVERSE: InverseRgbSpaceMatrix = [
    [2.0043819, -0.7304844, -0.2450052],
    [-0.7110285, 1.6202126, 0.0792227],
    [0.0381263, -0.0868780, 1.2725438],
];
const NTSC_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [1.9099961, -0.5324542, -0.2882091],
    [-0.9846663, 1.999171, -0.0283082],
    [0.0583056, -0.1183781, 0.8975535],
];
const PAL_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [3.0628971, -1.3931791, -0.4757517],
    [-0.969266, 1.8760108, 0.041556],
    [0.0678775, -0.2288548, 1.069349],
];
const PRO_PHOTO_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [1.3459433, -0.2556075, -0.0511118],
    [-0.5445989, 1.5081673, 0.0205351],
    [0.0000000, 0.0000000, 1.2118128],
];
const SMPTEC_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [3.505396, -1.7394894, -0.543964],
    [-1.0690722, 1.9778245, 0.0351722],
    [0.0563200, -0.1970226, 1.0502026],
];
const SRGB_INVERSE: InverseRgbSpaceMatrix = [
    [3.2404542, -1.5371385, -0.4985314],
    [-0.969266, 1.8760108, 0.041556],
    [0.0556434, -0.2040259, 1.0572252],
];
const WIDE_GAMUT_RGB_INVERSE: InverseRgbSpaceMatrix = [
    [1.4628067, -0.1840623, -0.2743606],
    [-0.5217933, 1.4472381, 0.0677227],
    [0.0349342, -0.096893, 1.2884099],
];
