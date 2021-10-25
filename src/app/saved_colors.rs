use crate::color::Color;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct SavedColors(Vec<(String, Color)>);

impl SavedColors {
    pub const STORAGE_KEY: &'static str = "epick.saved.colors";
    pub const FILE_NAME: &'static str = "colors.yaml";

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

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_gimp_palette(&self, name: &str) -> String {
        let mut gpl = format!("GIMP Palette\nName: {}.gpl\nColumns: 1\n#\n", name);
        for (i, (_, color)) in self.0.iter().enumerate() {
            let color = color.color32();
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

    pub fn as_hex_list(&self) -> String {
        self.0.iter().fold(String::new(), |mut s, (hex, _)| {
            s.push_str(hex.as_str());
            s.push('\n');
            s
        })
    }

    /// Loads the saved colors from the specified file located at `path`. The file is expected to
    /// be a valid YAML file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let data = fs::read(path).context("failed to read saved colors file")?;
        serde_yaml::from_slice(&data).context("failed to deserialize saved colors file")
    }

    pub fn from_yaml_str(yaml: &str) -> Result<Self> {
        serde_yaml::from_str(yaml).context("failed to deserialize saved colors from YAML")
    }

    pub fn as_yaml_str(&self) -> Result<String> {
        serde_yaml::to_string(&self).context("failed to serialize saved colors as YAML")
    }

    /// Saves this colors as YAML file in the provided `path`.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let data = serde_yaml::to_vec(&self).context("failed to serialize saved colors")?;
        fs::write(path, &data).context("failed to write saved colors to a file")
    }

    /// Returns system directory where saved colors should be placed joined by the `name` parameter.
    pub fn dir(name: impl AsRef<str>) -> Option<PathBuf> {
        let name = name.as_ref();
        if let Some(dir) = dirs::cache_dir() {
            return Some(dir.join(name));
        }

        if let Some(dir) = dirs::config_dir() {
            return Some(dir.join(name));
        }

        if let Some(dir) = dirs::home_dir() {
            return Some(dir.join(name));
        }

        None
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
