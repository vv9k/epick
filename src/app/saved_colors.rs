use crate::color::Color;

#[derive(Default, Debug)]
pub struct SavedColors(Vec<(String, Color)>);

impl SavedColors {
    pub fn add(&mut self, color: Color) -> bool {
        let hex = color.as_hex();
        if !self.0.iter().any(|(_hex, _)| _hex == &hex) {
            self.0.push((hex, color));
            return true;
        }
        false
    }

    pub fn insert(&mut self, i: usize, color: Color) {
        let color = (color.as_hex(), color);
        if !self.0.contains(&color) {
            self.0.insert(i, color);
        }
    }

    pub fn remove(&mut self, color: &Color) -> Option<(String, Color)> {
        self.0
            .iter()
            .position(|(_, col)| col == color)
            .map(|i| self.0.remove(i))
    }

    pub fn remove_pos(&mut self, i: usize) -> Option<(String, Color)> {
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

    pub fn as_gimp_palette(&self, name: &str) -> String {
        let mut gpl = format!("GIMP Palette\nName: {}.gpl\nColumns: 1\n#\n", name);
        for (i, (_, color)) in self.0.iter().enumerate() {
            let color = color.as_32();
            gpl.push_str(&format!(
                "{}\t{}\t{}\tcolor {}\n",
                color.r(),
                color.g(),
                color.b(),
                i
            ));
        }
        gpl
    }

    pub fn as_text_palette(&self) -> String {
        self.0.iter().fold(String::new(), |mut s, (hex, _)| {
            s.push('#');
            s.push_str(hex.as_str());
            s.push('\n');
            s
        })
    }
}

impl AsRef<[(String, Color)]> for SavedColors {
    fn as_ref(&self) -> &[(String, Color)] {
        self.0.as_ref()
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
