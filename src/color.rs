use egui::color::*;
use egui::lerp;
use std::cmp::Ordering;

#[derive(Clone, Copy, Default, Debug)]
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
        let r = (255. * (1. - cmyk.c) * (1. - cmyk.k)) as u8;
        let g = (255. * (1. - cmyk.m) * (1. - cmyk.k)) as u8;
        let b = (255. * (1. - cmyk.y) * (1. - cmyk.k)) as u8;
        Color32::from_rgb(r, g, b)
    }
}

impl From<Color32> for Cmyk {
    fn from(color: Color32) -> Self {
        let _r: f32 = color.r() as f32 / 255.;
        let _g: f32 = color.g() as f32 / 255.;
        let _b: f32 = color.b() as f32 / 255.;
        let k = 1.
            - [_r, _g, _b]
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .unwrap();
        let c = (1. - _r - k) / (1. - k);
        let m = (1. - _g - k) / (1. - k);
        let y = (1. - _b - k) / (1. - k);

        Cmyk::new(c, m, y, k)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
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

pub fn parse_color(hex: &str) -> Option<Color32> {
    if hex.len() == 6 {
        if let Some((r, g, b)) = parse_hex(&hex) {
            return Some(Color32::from_rgb(r, g, b));
        }
    }

    None
}

pub fn color_as_hex(color: &Color32) -> String {
    format!("{:02x}{:02x}{:02x}", color.r(), color.g(), color.b())
}

pub fn create_shades(base: &Color32, total: u8) -> Vec<Color32> {
    if total == 0 {
        return vec![base.clone()];
    }
    let mut step_total = total.saturating_sub(1);
    if step_total == 0 {
        step_total = 1;
    }
    let mut base_r = base.r();
    let mut base_g = base.g();
    let mut base_b = base.b();
    let step_r = base_r / step_total;
    let step_g = base_g / step_total;
    let step_b = base_b / step_total;

    (0..total)
        .into_iter()
        .map(|_| {
            let c = Color32::from_rgb(base_r, base_g, base_b);
            base_r = base_r.saturating_sub(step_r);
            base_g = base_g.saturating_sub(step_g);
            base_b = base_b.saturating_sub(step_b);
            c
        })
        .collect()
}
