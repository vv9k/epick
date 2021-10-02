mod cmyk;
mod gradient;
mod hsl;
mod lch;
mod luv;
mod xyz;

pub use gradient::Gradient;

pub use cmyk::Cmyk;
pub use hsl::Hsl;
pub use lch::Lch;
pub use luv::Luv;
pub use xyz::Xyz;

use egui::color::{Color32, Hsva, HsvaGamma, Rgba};

// Reference D65 whitepoint
pub const D65_X: f32 = 0.312713;
pub const D65_Y: f32 = 0.329016;
pub const D65_Z: f32 = 0.35827;
pub const D65_U: f32 = 4. * D65_X / (D65_X + 15. * D65_Y + 3. * D65_Z);
pub const D65_V: f32 = 9. * D65_X / (D65_X + 15. * D65_Y + 3. * D65_Z);
pub const CIE_E: f32 = 216. / 24389.;
pub const CIE_K: f32 = 24389. / 27.;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Cmyk(Cmyk),
    Rgb(Rgba),
    Hsv(Hsva),
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
        let color = Color32::from(*self);
        format!("{:02x}{:02x}{:02x}", color.r(), color.g(), color.b())
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        parse_hex(hex).map(|(r, g, b)| Color::Rgb(Color32::from_rgb(r, g, b).into()))
    }

    pub fn as_32(&self) -> Color32 {
        Color32::from(*self)
    }

    pub fn as_hue_offset(&self, offset: f32) -> Color {
        let mut hsv = Hsva::from(*self);

        hsv.h = (hsv.h + offset) % 1.;
        Self::Hsv(hsv)
    }

    pub fn rgba(&self) -> Rgba {
        self.into()
    }

    pub fn hsva(&self) -> Hsva {
        self.into()
    }

    pub fn luv(&self) -> Luv {
        self.into()
    }

    pub fn lch(&self) -> Lch {
        (*self).into()
    }

    pub fn xyz(&self) -> Xyz {
        (*self).into()
    }

    pub fn hsl(&self) -> Hsl {
        (*self).into()
    }

    pub fn cmyk(&self) -> Cmyk {
        self.into()
    }

    pub fn shades(&self, total: u8) -> Vec<Color> {
        let base = self.as_32();
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
        let base = self.as_32();
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
        let color = self.as_32();
        if color == Color32::BLACK {
            return Color32::WHITE.into();
        } else if color == Color32::WHITE {
            return Color32::BLACK.into();
        }

        self.as_hue_offset(0.5)
    }

    pub fn triadic(&self) -> (Color, Color) {
        (
            self.as_hue_offset(120. / 360.),
            self.as_hue_offset(240. / 360.),
        )
    }

    pub fn tetradic(&self) -> (Color, Color, Color) {
        (
            self.as_hue_offset(0.25),
            self.complementary(),
            self.as_hue_offset(0.75),
        )
    }

    pub fn analogous(&self) -> (Color, Color) {
        (self.as_hue_offset(-1. / 12.), self.as_hue_offset(1. / 12.))
    }

    pub fn split_complementary(&self) -> (Color, Color) {
        (
            self.as_hue_offset(150. / 360.),
            self.as_hue_offset(240. / 360.),
        )
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
impl From<&Color> for Cmyk {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}
impl From<&Color> for Luv {
    fn from(c: &Color) -> Self {
        (*c).into()
    }
}
impl From<&Color> for Hsl {
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

impl From<Color> for Rgba {
    fn from(c: Color) -> Rgba {
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

impl From<Color> for Hsva {
    fn from(c: Color) -> Hsva {
        match c {
            Color::Cmyk(c) => c.into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => c,
            Color::Luv(c) => c.into(),
            Color::Xyz(c) => c.into(),
            Color::Lch(c) => c.into(),
            Color::Hsl(c) => c.into(),
        }
    }
}

impl From<Color> for HsvaGamma {
    fn from(c: Color) -> HsvaGamma {
        match c {
            Color::Cmyk(c) => Color32::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => c.into(),
            Color::Luv(c) => Color32::from(c).into(),
            Color::Xyz(c) => Color32::from(c).into(),
            Color::Lch(c) => Color32::from(c).into(),
            Color::Hsl(c) => Color32::from(c).into(),
        }
    }
}

impl From<Color> for Cmyk {
    fn from(c: Color) -> Cmyk {
        match c {
            Color::Cmyk(c) => c,
            Color::Rgb(c) => Color32::from(c).into(),
            Color::Hsv(c) => Color32::from(c).into(),
            Color::Luv(c) => Color32::from(c).into(),
            Color::Xyz(c) => Color32::from(c).into(),
            Color::Lch(c) => Color32::from(c).into(),
            Color::Hsl(c) => Color32::from(c).into(),
        }
    }
}

impl From<Color> for Xyz {
    fn from(c: Color) -> Xyz {
        match c {
            Color::Cmyk(c) => Color32::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => Color32::from(c).into(),
            Color::Luv(c) => Color32::from(c).into(),
            Color::Xyz(c) => c,
            Color::Lch(c) => Color32::from(c).into(),
            Color::Hsl(c) => Color32::from(c).into(),
        }
    }
}

impl From<Color> for Luv {
    fn from(c: Color) -> Luv {
        match c {
            Color::Cmyk(c) => Color32::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => Color32::from(c).into(),
            Color::Luv(c) => c,
            Color::Xyz(c) => c.into(),
            Color::Lch(c) => Color32::from(c).into(),
            Color::Hsl(c) => Color32::from(c).into(),
        }
    }
}

impl From<Color> for Lch {
    fn from(c: Color) -> Lch {
        match c {
            Color::Cmyk(c) => Color32::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => Color32::from(c).into(),
            Color::Luv(c) => c.into(),
            Color::Xyz(c) => Color32::from(c).into(),
            Color::Lch(c) => c,
            Color::Hsl(c) => Color32::from(c).into(),
        }
    }
}

impl From<Color> for Hsl {
    fn from(c: Color) -> Hsl {
        match c {
            Color::Cmyk(c) => Color32::from(c).into(),
            Color::Rgb(c) => Color32::from(c).into(),
            Color::Hsv(c) => c.into(),
            Color::Luv(c) => Color32::from(c).into(),
            Color::Xyz(c) => Color32::from(c).into(),
            Color::Lch(c) => Color32::from(c).into(),
            Color::Hsl(c) => c,
        }
    }
}

impl From<Cmyk> for Color {
    fn from(c: Cmyk) -> Color {
        Color::Cmyk(c)
    }
}
impl From<Rgba> for Color {
    fn from(c: Rgba) -> Color {
        Color::Rgb(c)
    }
}
impl From<Color32> for Color {
    fn from(c: Color32) -> Color {
        Color::Rgb(Rgba::from(c))
    }
}
impl From<Hsva> for Color {
    fn from(c: Hsva) -> Color {
        Color::Hsv(c)
    }
}

impl From<Xyz> for Color {
    fn from(c: Xyz) -> Self {
        Color::Xyz(c)
    }
}

impl From<Luv> for Color {
    fn from(c: Luv) -> Color {
        Color::Luv(c)
    }
}

impl From<Lch> for Color {
    fn from(c: Lch) -> Color {
        Color::Lch(c)
    }
}

impl From<Hsl> for Color {
    fn from(c: Hsl) -> Color {
        Color::Hsl(c)
    }
}