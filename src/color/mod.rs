mod cmyk;
mod gradient;
mod hsl;
mod hsv;
mod illuminant;
mod lch;
mod luv;
mod rgb;
mod working_space;
mod xyz;

pub use gradient::Gradient;

pub use cmyk::Cmyk;
pub use hsl::Hsl;
pub use hsv::Hsv;
pub use lch::Lch;
pub use luv::Luv;
pub use rgb::Rgb;
pub use xyz::Xyz;

use egui::color::{Color32, Hsva, HsvaGamma, Rgba};

pub const CIE_E: f32 = 216. / 24389.;
pub const CIE_K: f32 = 24389. / 27.;
pub const U8_MAX: f32 = u8::MAX as f32;
pub const U8_MIN: f32 = u8::MIN as f32;

//################################################################################

const fn hex_val(ch: u8) -> u8 {
    match ch {
        b'0'..=b'9' => ch - 48,
        b'A'..=b'F' => ch - 55,
        b'a'..=b'f' => ch - 87,
        _ => 0,
    }
}

const fn hex_chars_to_u8(ch: (u8, u8)) -> u8 {
    let mut result = 0;
    result |= hex_val(ch.0);
    result <<= 4;
    result |= hex_val(ch.1);
    result
}

pub fn parse_hex(color: &str) -> Option<(u8, u8, u8)> {
    let mut bytes = color.as_bytes().chunks(2);

    Some((
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
    ))
}

pub fn contrast_color(color: impl Into<Rgba>) -> Color32 {
    if color.into().intensity() < 0.5 {
        Color32::WHITE
    } else {
        Color32::BLACK
    }
}

//################################################################################

#[derive(Debug, PartialEq)]
pub enum ColorHarmony {
    Complementary,
    Triadic,
    Tetradic,
    Analogous,
    SplitComplementary,
    Square,
    Monochromatic,
}

impl AsRef<str> for ColorHarmony {
    fn as_ref(&self) -> &str {
        match &self {
            ColorHarmony::Complementary => "complementary",
            ColorHarmony::Triadic => "triadic",
            ColorHarmony::Tetradic => "tetradic",
            ColorHarmony::Analogous => "analogous",
            ColorHarmony::SplitComplementary => "split complementary",
            ColorHarmony::Square => "square",
            ColorHarmony::Monochromatic => "monochromatic",
        }
    }
}

//################################################################################

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayFormat {
    Hex,
    HexUpercase,
    CssRgb,
    CssHsl,
}

impl AsRef<str> for DisplayFormat {
    fn as_ref(&self) -> &str {
        match &self {
            DisplayFormat::Hex => "hex",
            DisplayFormat::HexUpercase => "hex uppercase",
            DisplayFormat::CssRgb => "css rgb",
            DisplayFormat::CssHsl => "css hsl",
        }
    }
}

//################################################################################

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Cmyk(Cmyk),
    Rgb(Rgb),
    Hsv(Hsv),
    Luv(Luv),
    Xyz(Xyz),
    Lch(Lch),
    Hsl(Hsl),
}

impl Color {
    pub fn black() -> Self {
        Self::Rgb(Color32::BLACK.into())
    }

    pub fn white() -> Self {
        Self::Rgb(Color32::WHITE.into())
    }

    pub fn as_hex(&self) -> String {
        let color = self.as_rgb_triplet();
        format!("#{:02x}{:02x}{:02x}", color.0, color.1, color.2)
    }

    pub fn as_css_rgb(&self) -> String {
        let color = self.as_rgb_triplet();
        format!("rgb({}, {}, {})", color.0, color.1, color.2)
    }

    pub fn as_css_hsl(&self) -> String {
        let color = self.hsl();
        format!(
            "hsl({}, {}%, {}%)",
            color.h_scaled() as u16,
            color.s_scaled() as u16,
            color.l_scaled() as u16
        )
    }

    pub fn display(&self, format: DisplayFormat) -> String {
        match format {
            DisplayFormat::Hex => self.as_hex(),
            DisplayFormat::HexUpercase => self.as_hex().to_uppercase(),
            DisplayFormat::CssRgb => self.as_css_rgb(),
            DisplayFormat::CssHsl => self.as_css_hsl(),
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        parse_hex(hex).map(|(r, g, b)| Color::Rgb(Rgb::new(r as f32, g as f32, b as f32)))
    }

    pub fn as_hue_offset(&self, offset: f32) -> Color {
        let mut hsv = self.hsv();
        hsv.offset_hue(offset);
        Self::Hsv(hsv)
    }

    pub fn as_saturation_offset(&self, offset: f32) -> Color {
        let mut hsv = self.hsv();
        hsv.offset_saturation(offset);
        Self::Hsv(hsv)
    }

    pub fn as_rgb_triplet(&self) -> (u8, u8, u8) {
        let color = self.rgb();
        (
            color.r_scaled().floor() as u8,
            color.g_scaled().floor() as u8,
            color.b_scaled().floor() as u8,
        )
    }

    pub fn color32(&self) -> Color32 {
        Color32::from(*self)
    }

    pub fn hsva(&self) -> Hsva {
        self.into()
    }

    pub fn rgba(&self) -> Rgba {
        self.into()
    }

    pub fn cmyk(&self) -> Cmyk {
        self.into()
    }

    pub fn hsl(&self) -> Hsl {
        (*self).into()
    }

    pub fn hsv(&self) -> Hsv {
        self.into()
    }

    pub fn luv(&self) -> Luv {
        self.into()
    }

    pub fn lch(&self) -> Lch {
        (*self).into()
    }

    pub fn rgb(&self) -> Rgb {
        self.into()
    }

    pub fn xyz(&self) -> Xyz {
        (*self).into()
    }

    pub fn shades(&self, total: u8) -> Vec<Color> {
        let base = self.color32();
        if total == 0 {
            return vec![*self];
        }
        let mut step_total = total.saturating_sub(1) as f32;
        if step_total == 0. {
            step_total = 1.;
        }
        let mut base_r = base.r();
        let mut base_g = base.g();
        let mut base_b = base.b();
        let step_r = (base_r as f32 / step_total).ceil() as u8;
        let step_g = (base_g as f32 / step_total).ceil() as u8;
        let step_b = (base_b as f32 / step_total).ceil() as u8;

        (0..total)
            .into_iter()
            .map(|_| {
                let c = Color32::from_rgb(base_r, base_g, base_b);
                base_r = base_r.saturating_sub(step_r);
                base_g = base_g.saturating_sub(step_g);
                base_b = base_b.saturating_sub(step_b);
                Color::Rgb(c.into())
            })
            .collect()
    }

    pub fn tints(&self, total: u8) -> Vec<Color> {
        let base = self.color32();
        if total == 0 {
            return vec![*self];
        }
        let mut step_total = total.saturating_sub(1) as f32;
        if step_total == 0. {
            step_total = 1.;
        }
        let mut base_r = base.r();
        let mut base_g = base.g();
        let mut base_b = base.b();
        let step_r = ((u8::MAX - base_r) as f32 / step_total).ceil() as u8;
        let step_g = ((u8::MAX - base_g) as f32 / step_total).ceil() as u8;
        let step_b = ((u8::MAX - base_b) as f32 / step_total).ceil() as u8;

        (0..total)
            .into_iter()
            .map(|_| {
                let c = Color32::from_rgb(base_r, base_g, base_b);
                base_r = base_r.saturating_add(step_r);
                base_g = base_g.saturating_add(step_g);
                base_b = base_b.saturating_add(step_b);
                Color::Rgb(c.into())
            })
            .collect()
    }

    pub fn hues(&self, total: u8, step: f32) -> Vec<Color> {
        let mut colors = Vec::new();
        let hsva = Hsva::from(*self);
        for i in (0..=total).rev() {
            let mut _h = hsva;
            _h.h -= step * i as f32;
            colors.push(_h.into());
        }

        for i in 1..=total {
            let mut _h = hsva;
            _h.h += step * i as f32;
            colors.push(_h.into());
        }

        colors
    }

    pub fn complementary(&self) -> Color {
        let color = self.color32();
        if color == Color32::BLACK {
            return Color32::WHITE.into();
        } else if color == Color32::WHITE {
            return Color32::BLACK.into();
        }

        self.as_hue_offset(6. / 12.)
    }

    pub fn triadic(&self) -> (Color, Color) {
        (self.as_hue_offset(4. / 12.), self.as_hue_offset(8. / 12.))
    }

    pub fn tetradic(&self) -> (Color, Color, Color) {
        (
            self.as_hue_offset(2. / 12.),
            self.complementary(),
            self.as_hue_offset(8. / 12.),
        )
    }

    pub fn analogous(&self) -> (Color, Color) {
        (self.as_hue_offset(-1. / 12.), self.as_hue_offset(1. / 12.))
    }

    pub fn split_complementary(&self) -> (Color, Color) {
        (self.as_hue_offset(5. / 12.), self.as_hue_offset(7. / 12.))
    }

    pub fn square(&self) -> (Color, Color, Color) {
        (
            self.as_hue_offset(3. / 12.),
            self.complementary(),
            self.as_hue_offset(9. / 12.),
        )
    }

    pub fn monochromatic(&self) -> (Color, Color, Color) {
        (
            self.as_saturation_offset(-3. / 12.),
            self.as_saturation_offset(-6. / 12.),
            self.as_saturation_offset(-9. / 12.),
        )
    }
}

//##################################################################################################

impl From<&Color> for Color32 {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<&Color> for Rgba {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<&Color> for Hsva {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}

impl From<Color> for Color32 {
    fn from(c: Color) -> Color32 {
        match c {
            Color::Cmyk(c) => c.into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => c.into(),
            Color::Luv(c) => c.into(),
            Color::Xyz(c) => c.into(),
            Color::Lch(c) => c.into(),
            Color::Hsl(c) => c.into(),
        }
    }
}

impl From<Color32> for Color {
    fn from(c: Color32) -> Color {
        Color::Rgb(c.into())
    }
}

macro_rules! convert_color {
    ($c:ident) => {
        match $c {
            Color::Cmyk(c) => Rgb::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => Rgb::from(c).into(),
            Color::Luv(c) => Rgb::from(c).into(),
            Color::Xyz(c) => Rgb::from(c).into(),
            Color::Lch(c) => Rgb::from(c).into(),
            Color::Hsl(c) => Rgb::from(c).into(),
        }
    };
}

impl From<Color> for Rgba {
    fn from(c: Color) -> Rgba {
        convert_color!(c)
    }
}

impl From<Rgba> for Color {
    fn from(c: Rgba) -> Color {
        Color::Rgb(c.into())
    }
}

impl From<Hsva> for Color {
    fn from(c: Hsva) -> Color {
        Color::Hsv(c.into())
    }
}

impl From<Color> for Hsva {
    fn from(c: Color) -> Hsva {
        convert_color!(c)
    }
}

impl From<Color> for HsvaGamma {
    fn from(c: Color) -> HsvaGamma {
        convert_color!(c)
    }
}

//##################################################################################################

impl From<Cmyk> for Color {
    fn from(c: Cmyk) -> Color {
        Color::Cmyk(c)
    }
}

impl From<Hsl> for Color {
    fn from(c: Hsl) -> Color {
        Color::Hsl(c)
    }
}

impl From<Hsv> for Color {
    fn from(c: Hsv) -> Color {
        Color::Hsv(c)
    }
}

impl From<Lch> for Color {
    fn from(c: Lch) -> Color {
        Color::Lch(c)
    }
}

impl From<Luv> for Color {
    fn from(c: Luv) -> Color {
        Color::Luv(c)
    }
}

impl From<Rgb> for Color {
    fn from(c: Rgb) -> Color {
        Color::Rgb(c)
    }
}

impl From<Xyz> for Color {
    fn from(c: Xyz) -> Self {
        Color::Xyz(c)
    }
}

//##################################################################################################

#[cfg(test)]
mod tests {
    use super::parse_hex;
    #[test]
    fn parses_hex() {
        macro_rules! test_case {
            ($hex:literal, $r:expr, $g:expr, $b:expr) => {
                let parsed = parse_hex($hex).unwrap();
                assert_eq!($r, parsed.0);
                assert_eq!($g, parsed.1);
                assert_eq!($b, parsed.2);
            };
        }

        test_case!("000000", 0, 0, 0);
        test_case!("ffffff", 255, 255, 255);
        test_case!("abbaaf", 171, 186, 175);
        test_case!("12abff", 18, 171, 255);
    }
}
