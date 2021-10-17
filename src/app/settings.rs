use crate::color::{ChromaticAdaptationMethod, DisplayFormat, Illuminant, RgbWorkingSpace};

#[derive(Debug)]
pub struct ColorSpaceSettings {
    pub rgb: bool,
    pub cmyk: bool,
    pub hsv: bool,
    pub hsl: bool,
    pub luv: bool,
    pub lch_uv: bool,
    pub lab: bool,
    pub lch_ab: bool,
}

impl Default for ColorSpaceSettings {
    fn default() -> Self {
        Self {
            rgb: true,
            cmyk: true,
            hsv: true,
            hsl: true,
            luv: false,
            lch_uv: false,
            lab: false,
            lch_ab: false,
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    pub color_display_format: DisplayFormat,
    pub color_spaces: ColorSpaceSettings,
    pub rgb_working_space: RgbWorkingSpace,
    pub chromatic_adaptation_method: ChromaticAdaptationMethod,
    pub illuminant: Illuminant,
}

impl Default for Settings {
    fn default() -> Self {
        let ws = RgbWorkingSpace::default();
        Self {
            color_display_format: DisplayFormat::Hex,
            color_spaces: ColorSpaceSettings::default(),
            rgb_working_space: ws,
            chromatic_adaptation_method: ChromaticAdaptationMethod::default(),
            illuminant: ws.reference_illuminant(),
        }
    }
}
