use egui::color::*;
use egui::lerp;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Cmyk(Cmyk),
    Rgb(Rgba),
    Hsv(Hsva),
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

    pub fn with_hue_offset(&self, offset: f32) -> Color {
        let mut hsv = Hsva::from(*self);

        hsv.h = (hsv.h + offset) % 1.;
        Self::Hsv(hsv)
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

        self.with_hue_offset(0.5)
    }

    pub fn triadic(&self) -> (Color, Color) {
        (
            self.with_hue_offset(120. / 360.),
            self.with_hue_offset(240. / 360.),
        )
    }

    pub fn tetradic(&self) -> (Color, Color, Color) {
        (
            self.with_hue_offset(0.25),
            self.complementary(),
            self.with_hue_offset(0.75),
        )
    }

    pub fn analogous(&self) -> (Color, Color) {
        (
            self.with_hue_offset(-1. / 12.),
            self.with_hue_offset(1. / 12.),
        )
    }

    pub fn split_complementary(&self) -> (Color, Color) {
        (
            self.with_hue_offset(150. / 360.),
            self.with_hue_offset(240. / 360.),
        )
    }
}

impl From<Color> for Color32 {
    fn from(c: Color) -> Color32 {
        match c {
            Color::Cmyk(c) => c.into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => c.into(),
        }
    }
}

impl From<Color> for Rgba {
    fn from(c: Color) -> Rgba {
        match c {
            Color::Cmyk(c) => c.into(),
            Color::Rgb(c) => c,
            Color::Hsv(c) => c.into(),
        }
    }
}

impl From<Color> for Hsva {
    fn from(c: Color) -> Hsva {
        match c {
            Color::Cmyk(c) => c.into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => c,
        }
    }
}

impl From<Color> for HsvaGamma {
    fn from(c: Color) -> HsvaGamma {
        match c {
            Color::Cmyk(c) => Color32::from(c).into(),
            Color::Rgb(c) => c.into(),
            Color::Hsv(c) => c.into(),
        }
    }
}

impl From<Color> for Cmyk {
    fn from(c: Color) -> Cmyk {
        match c {
            Color::Cmyk(c) => c,
            Color::Rgb(c) => Color32::from(c).into(),
            Color::Hsv(c) => Color32::from(c).into(),
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

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Cmyk {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}

impl Cmyk {
    pub fn new(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self { c, m, y, k }
    }
}

impl From<Cmyk> for Color32 {
    fn from(cmyk: Cmyk) -> Self {
        let r = (255. * (1. - (cmyk.c * (1. - cmyk.k) + cmyk.k))).round() as u8;
        let g = (255. * (1. - (cmyk.m * (1. - cmyk.k) + cmyk.k))).round() as u8;
        let b = (255. * (1. - (cmyk.y * (1. - cmyk.k) + cmyk.k))).round() as u8;
        Color32::from_rgb(r, g, b)
    }
}

impl From<Cmyk> for Rgba {
    fn from(cmyk: Cmyk) -> Rgba {
        Color32::from(cmyk).into()
    }
}

impl From<Cmyk> for Hsva {
    fn from(cmyk: Cmyk) -> Hsva {
        Color32::from(cmyk).into()
    }
}

impl From<Rgba> for Cmyk {
    fn from(rgba: Rgba) -> Cmyk {
        Color32::from(rgba).into()
    }
}

impl From<Color32> for Cmyk {
    fn from(color: Color32) -> Self {
        let _r: f32 = 1. - (color.r() as f32 / 255.);
        let _g: f32 = 1. - (color.g() as f32 / 255.);
        let _b: f32 = 1. - (color.b() as f32 / 255.);
        let rgb = [_r, _g, _b];
        let k = rgb
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap();

        if *k == 1. {
            return Cmyk::new(0., 0., 0., *k);
        }

        let c = (_r - k) / (1. - k);
        let m = (_g - k) / (1. - k);
        let y = (_b - k) / (1. - k);

        Cmyk::new(
            if c.is_nan() { 0. } else { c },
            if m.is_nan() { 0. } else { m },
            if y.is_nan() { 0. } else { y },
            if k.is_nan() { 0. } else { *k },
        )
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Gradient(pub Vec<Color32>);

#[allow(dead_code)]
impl Gradient {
    pub fn one_color(srgba: Color32) -> Self {
        Self(vec![srgba, srgba])
    }

    pub fn as_hex(&self) -> Option<String> {
        self.0
            .first()
            .map(|color| format!("{:02x}{:02x}{:02x}", color.r(), color.g(), color.b()))
    }

    pub fn texture_gradient(left: Color32, right: Color32) -> Self {
        Self(vec![left, right])
    }
    pub fn ground_truth_linear_gradient(left: Color32, right: Color32) -> Self {
        let left = Rgba::from(left);
        let right = Rgba::from(right);

        let n = 255;
        Self(
            (0..=n)
                .map(|i| {
                    let t = i as f32 / n as f32;
                    Color32::from(lerp(left..=right, t))
                })
                .collect(),
        )
    }

    /// Do premultiplied alpha-aware blending of the gradient on top of the fill color
    pub fn with_bg_fill(self, bg: Color32) -> Self {
        let bg = Rgba::from(bg);
        Self(
            self.0
                .into_iter()
                .map(|fg| {
                    let fg = Rgba::from(fg);
                    Color32::from(bg * (1.0 - fg.a()) + fg)
                })
                .collect(),
        )
    }

    pub fn to_pixel_row(&self) -> Vec<Color32> {
        self.0.clone()
    }
}

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
