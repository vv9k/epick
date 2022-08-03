use crate::color::Color;

use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Palette(Vec<Color>);

impl Palette {
    pub fn iter(&self) -> impl Iterator<Item = &Color> {
        self.0.iter()
    }
    pub fn add(&mut self, color: Color) -> bool {
        if !self.0.iter().any(|clr| clr == &color) {
            self.0.push(color);
            return true;
        }
        false
    }

    pub fn insert(&mut self, i: usize, color: Color) {
        if !self.0.contains(&color) {
            self.0.insert(i, color);
        }
    }

    pub fn remove(&mut self, color: &Color) -> Option<Color> {
        self.0
            .iter()
            .position(|col| col == color)
            .map(|i| self.0.remove(i))
    }

    pub fn remove_pos(&mut self, i: usize) -> Option<Color> {
        if i < self.0.len() {
            Some(self.0.remove(i))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_gimp_palette(&self, name: &str) -> String {
        let mut gpl = format!("GIMP Palette\nName: {}.gpl\nColumns: 1\n#\n", name);
        for (i, color) in self.0.iter().enumerate() {
            let color = color.color32();
            let _ = writeln!(
                gpl,
                "{}\t{}\t{}\tcolor {}",
                color.r(),
                color.g(),
                color.b(),
                i
            );
        }
        gpl
    }

    pub fn as_hex_list(&self) -> String {
        self.0.iter().fold(String::new(), |mut s, color| {
            s.push_str(&color.as_hex());
            s.push('\n');
            s
        })
    }
}

impl std::iter::FromIterator<Color> for Palette {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PaletteFormat {
    Gimp,
    Text,
}

impl PaletteFormat {
    pub fn extension(&self) -> &str {
        match self {
            PaletteFormat::Gimp => "gpl",
            PaletteFormat::Text => "txt",
        }
    }
}

impl AsRef<str> for PaletteFormat {
    fn as_ref(&self) -> &str {
        match self {
            PaletteFormat::Gimp => "GIMP (gpl)",
            PaletteFormat::Text => "Hex list (txt)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgb;

    #[test]
    fn export_color_palette() {
        let mut colors = Palette::default();
        colors.add(Rgb::new_scaled(0, 0, 0).into());
        colors.add(Rgb::new_scaled(255, 0, 0).into());
        colors.add(Rgb::new_scaled(0, 255, 0).into());
        colors.add(Rgb::new_scaled(0, 0, 255).into());

        let want = r#"#000000
#ff0000
#00ff00
#0000ff
"#;
        assert_eq!(colors.as_hex_list(), want);

        let want = r#"GIMP Palette
Name: colors.gpl
Columns: 1
#
0	0	0	color 0
255	0	0	color 1
0	255	0	color 2
0	0	255	color 3
"#;

        assert_eq!(colors.as_gimp_palette("colors"), want);
    }
}
