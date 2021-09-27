mod gradient;
pub use gradient::Gradient;

use egui::color::{Color32, Hsva, HsvaGamma, Rgba};
use std::cmp::Ordering;

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

//################################################################################

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

#[allow(clippy::many_single_char_names)]
impl From<Color32> for Cmyk {
    fn from(color: Color32) -> Self {
        let r: f32 = 1. - (color.r() as f32 / u8::MAX as f32);
        let g: f32 = 1. - (color.g() as f32 / u8::MAX as f32);
        let b: f32 = 1. - (color.b() as f32 / u8::MAX as f32);
        let rgb = [r, g, b];
        let k = rgb
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap();

        if (*k - 1.).abs() < f32::EPSILON {
            return Cmyk::new(0., 0., 0., *k);
        }

        let c = (r - k) / (1. - k);
        let m = (g - k) / (1. - k);
        let y = (b - k) / (1. - k);

        Cmyk::new(
            if c.is_nan() { 0. } else { c },
            if m.is_nan() { 0. } else { m },
            if y.is_nan() { 0. } else { y },
            if k.is_nan() { 0. } else { *k },
        )
    }
}

//################################################################################

type RgbSpaceMatrix = [[f32; 3]; 3];

const ADOBE_RGB: RgbSpaceMatrix = [
    [0.5767309, 0.185554, 0.1881852],
    [0.2973769, 0.6273491, 0.0752741],
    [0.0270343, 0.0706872, 0.9911085],
];

const ADOBE_RGB_INVERSE: RgbSpaceMatrix = [
    [2.041369, -0.5649464, -0.3446944],
    [-0.969266, 1.8760108, 0.0415560],
    [0.0134474, -0.1183897, 1.0154096],
];

//################################################################################

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
        let r = color.r() as f32 / u8::MAX as f32;
        let g = color.g() as f32 / u8::MAX as f32;
        let b = color.b() as f32 / u8::MAX as f32;

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

//################################################################################

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Luv {
    pub l: f32,
    pub u: f32,
    pub v: f32,
}

impl Luv {
    pub fn new(l: f32, u: f32, v: f32) -> Self {
        Self { l, u, v }
    }
}

impl From<Luv> for Rgba {
    fn from(luv: Luv) -> Rgba {
        Color32::from(luv).into()
    }
}

impl From<Luv> for Hsva {
    fn from(luv: Luv) -> Hsva {
        Color32::from(luv).into()
    }
}

impl From<Rgba> for Luv {
    fn from(rgba: Rgba) -> Luv {
        Color32::from(rgba).into()
    }
}

// Reference D65 whitepoint
const D65_X: f32 = 0.312713;
const D65_Y: f32 = 0.329016;
const D65_Z: f32 = 0.35827;
const D65_U: f32 = 4. * D65_X / (D65_X + 15. * D65_Y + 3. * D65_Z);
const D65_V: f32 = 9. * D65_X / (D65_X + 15. * D65_Y + 3. * D65_Z);
const CIE_E: f32 = 216. / 24389.;
const CIE_K: f32 = 24389. / 27.;

#[allow(clippy::many_single_char_names)]
impl From<Xyz> for Luv {
    fn from(color: Xyz) -> Self {
        let x = color.x;
        let y = color.y;
        let z = color.z;

        let u = 4. * x / (x + 15. * y + 3. * z);
        let v = 9. * x / (x + 15. * y + 3. * z);

        let yr = y / D65_Y;

        let l = if yr > CIE_E {
            116. * yr.cbrt() - 16.
        } else {
            CIE_K * yr
        };

        let u = 13. * l * (u - D65_U);
        let v = 13. * l * (v - D65_V);

        Luv {
            l: if l.is_nan() { 0. } else { l },
            u: if u.is_nan() { 0. } else { u },
            v: if v.is_nan() { 0. } else { v },
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Luv> for Xyz {
    fn from(color: Luv) -> Self {
        let l = color.l;
        let u = color.u;
        let v = color.v;

        let y = if l > CIE_K * CIE_E {
            ((l + 16.) / 116.).powi(3)
        } else {
            l / CIE_K
        };

        let a = ((52. * l / (u + 13. * l * D65_U)) - 1.) / 3.;
        let b = -5. * y;
        let c = -(1.0f32 / 3.);
        let d = y * ((39. * l / (v + 13. * l * D65_V)) - 5.);

        let x = (d - b) / (a - c);
        let z = x * a + b;

        Xyz {
            x: if x.is_nan() { 0. } else { x },
            y: if y.is_nan() { 0. } else { y },
            z: if z.is_nan() { 0. } else { z },
        }
    }
}

impl From<Luv> for Color32 {
    fn from(color: Luv) -> Self {
        Xyz::from(color).into()
    }
}

impl From<Color32> for Luv {
    fn from(color: Color32) -> Self {
        Xyz::from(color).into()
    }
}

//################################################################################

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

impl Lch {
    pub fn new(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h }
    }
}

impl From<Lch> for Rgba {
    fn from(lch: Lch) -> Rgba {
        Color32::from(lch).into()
    }
}

impl From<Lch> for Hsva {
    fn from(lch: Lch) -> Hsva {
        Color32::from(lch).into()
    }
}

impl From<Rgba> for Lch {
    fn from(rgba: Rgba) -> Lch {
        Color32::from(rgba).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Luv> for Lch {
    fn from(color: Luv) -> Self {
        let u = color.u;
        let v = color.v;
        let c = (u.powi(2) + v.powi(2)).sqrt();
        let vu_atan = f32::atan2(v, u);
        let h = if vu_atan >= 0. {
            vu_atan
        } else {
            vu_atan + 360.
        };

        Lch {
            l: color.l,
            c: if c.is_nan() { 0. } else { c },
            h: if h.is_nan() { 0. } else { h },
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Lch> for Luv {
    fn from(color: Lch) -> Self {
        let c = color.c;
        let h = color.h;

        let u = c * h.to_radians().cos();
        let v = c * h.to_radians().sin();

        Luv {
            l: if color.l.is_nan() { 0. } else { color.l },
            u: if u.is_nan() { 0. } else { u },
            v: if v.is_nan() { 0. } else { v },
        }
    }
}

impl From<Lch> for Color32 {
    fn from(color: Lch) -> Self {
        Luv::from(color).into()
    }
}

impl From<Color32> for Lch {
    fn from(color: Color32) -> Self {
        Luv::from(color).into()
    }
}
//################################################################################

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl Hsl {
    pub fn new(hue: f32, saturation: f32, light: f32) -> Self {
        Self {
            h: hue,
            s: saturation,
            l: light,
        }
    }
}

impl From<Hsl> for Rgba {
    fn from(hsl: Hsl) -> Rgba {
        Color32::from(hsl).into()
    }
}

impl From<Rgba> for Hsl {
    fn from(rgba: Rgba) -> Hsl {
        Color32::from(rgba).into()
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Hsva> for Hsl {
    fn from(color: Hsva) -> Self {
        let h = color.h;
        let s = color.s;
        let v = color.v;

        let mut l = (2. - s) * v;
        let mut ss = s * v;
        if l <= 1. {
            ss /= l;
        } else {
            ss /= 2. - l;
        }
        l /= 2.;

        Hsl {
            h: if h.is_nan() { 0. } else { h },
            s: if ss.is_nan() { 0. } else { ss },
            l: if l.is_nan() { 0. } else { l },
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl From<Hsl> for Hsva {
    fn from(color: Hsl) -> Self {
        let h = color.h;
        let mut ss = color.s;
        let l = color.l * 2.;

        if l <= 1. {
            ss *= l;
        } else {
            ss *= 2. - l;
        }

        let v = (l + ss) / 2.;
        let s = (2. * ss) / (l + ss);

        Hsva {
            h: if h.is_nan() { 0. } else { h },
            s: if s.is_nan() { 0. } else { s },
            v: if v.is_nan() { 0. } else { v },
            a: 1.,
        }
    }
}

impl From<Hsl> for Color32 {
    fn from(color: Hsl) -> Self {
        Hsva::from(color).into()
    }
}

impl From<Color32> for Hsl {
    fn from(color: Color32) -> Self {
        Hsva::from(color).into()
    }
}
