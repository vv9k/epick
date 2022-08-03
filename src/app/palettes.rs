use crate::color::Palette;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct NamedPalette {
    pub name: String,
    pub palette: Palette,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Palettes {
    palettes: Vec<NamedPalette>,
    current_idx: usize,
}

impl Default for Palettes {
    fn default() -> Self {
        Self::new(NamedPalette::default())
    }
}

impl Palettes {
    pub const STORAGE_KEY: &'static str = "epick.saved.palettes";
    pub const FILE_NAME: &'static str = "palettes.yaml";

    pub fn new(palette: NamedPalette) -> Self {
        Self {
            palettes: vec![palette],
            current_idx: 0,
        }
    }

    pub fn current(&self) -> &NamedPalette {
        // SAFETY: Palettes always keeps at least one palette thus it is always accessible
        unsafe { self.palettes.get_unchecked(self.current_idx) }
    }

    pub fn current_mut(&mut self) -> &mut NamedPalette {
        // SAFETY: Palettes always keeps at least one palette thus it is always accessible
        unsafe { self.palettes.get_unchecked_mut(self.current_idx) }
    }

    pub fn len(&self) -> usize {
        self.palettes.len()
    }

    /// Moves current index to the next palette if such exists
    pub fn next(&mut self) {
        if self.current_idx < self.palettes.len() - 1 {
            self.current_idx += 1;
        }
    }

    /// Moves current index to the previous palette
    pub fn prev(&mut self) {
        if self.current_idx > 0 {
            self.current_idx -= 1;
        }
    }

    pub fn move_to_last(&mut self) {
        self.current_idx = self.palettes.len() - 1;
    }

    pub fn move_to_name(&mut self, name: impl AsRef<str>) {
        let name = name.as_ref();
        if let Some(idx) = self.palettes.iter().position(|p| p.name == name) {
            self.current_idx = idx;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &NamedPalette> {
        self.palettes.iter()
    }

    pub fn append_empty(&mut self) {
        let palette = NamedPalette::default();
        self.add(palette);
    }

    pub fn add(&mut self, palette: NamedPalette) -> bool {
        if !self.palettes.iter().any(|p| p.name == palette.name) {
            self.palettes.push(palette);
            return true;
        }
        false
    }

    pub fn insert(&mut self, i: usize, palette: NamedPalette) {
        if !self.palettes.iter().any(|p| p.name == palette.name) {
            self.palettes.insert(i, palette);
            if i <= self.current_idx {
                self.next();
            }
        }
    }

    pub fn remove_pos(&mut self, i: usize) -> Option<NamedPalette> {
        if i < self.palettes.len() {
            let removed = self.palettes.remove(i);
            if self.palettes.is_empty() {
                self.palettes.push(NamedPalette::default());
                self.current_idx = 0;
            }
            if i <= self.current_idx {
                self.prev();
            }
            Some(removed)
        } else {
            None
        }
    }

    pub fn remove(&mut self, palette: &NamedPalette) -> Option<NamedPalette> {
        self.palettes
            .iter()
            .position(|p| p == palette)
            .and_then(|i| self.remove_pos(i))
    }

    pub fn remove_current(&mut self) {
        self.remove_pos(self.current_idx);
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
        let mut data = Vec::with_capacity(128);
        serde_yaml::to_writer(&mut data, &self).context("failed to serialize saved colors")?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, Rgb};
    const C1: crate::color::Color = Color::Rgb(Rgb::new_unchecked(0., 0., 0.));
    const C2: crate::color::Color = Color::Rgb(Rgb::new_unchecked(0., 1., 0.));
    const C3: crate::color::Color = Color::Rgb(Rgb::new_unchecked(1., 0., 1.));

    fn test_palettes() -> (NamedPalette, NamedPalette, NamedPalette, NamedPalette) {
        let p1 = NamedPalette {
            palette: Palette::from_iter([C1]),
            name: "p1".into(),
        };
        let p2 = NamedPalette {
            palette: Palette::from_iter([C1, C2]),
            name: "p2".into(),
        };
        let p3 = NamedPalette {
            palette: Palette::from_iter([C1, C2, C3]),
            name: "p3".into(),
        };
        let p4 = NamedPalette {
            palette: Palette::from_iter([C3]),
            name: "p4".into(),
        };
        (p1, p2, p3, p4)
    }

    #[test]
    fn navigation() {
        let (p1, p2, p3, p4) = test_palettes();
        let mut palettes = Palettes::new(p1.clone());

        assert_eq!(palettes.current(), &p1);
        palettes.next();
        assert_eq!(palettes.current(), &p1);
        palettes.prev();
        assert_eq!(palettes.current(), &p1);

        palettes.add(p2.clone());
        assert_eq!(palettes.current(), &p1);
        palettes.next();
        assert_eq!(palettes.current(), &p2);
        palettes.next();
        assert_eq!(palettes.current(), &p2);

        palettes.add(p3.clone());
        palettes.add(p4.clone());
        palettes.next();
        assert_eq!(palettes.current(), &p3);
        palettes.next();
        assert_eq!(palettes.current(), &p4);
        palettes.prev();
        assert_eq!(palettes.current(), &p3);
        palettes.prev();
        assert_eq!(palettes.current(), &p2);
        palettes.prev();
        assert_eq!(palettes.current(), &p1);
        palettes.prev();
        assert_eq!(palettes.current(), &p1);

        palettes.move_to_last();
        assert_eq!(palettes.current(), &p4);

        palettes.move_to_name(&p2.name);
        assert_eq!(palettes.current(), &p2);
        palettes.move_to_name(&p1.name);
        assert_eq!(palettes.current(), &p1);
    }

    #[test]
    fn append() {
        let (p1, _, _, _) = test_palettes();
        let mut palettes = Palettes::new(p1);
        assert_eq!(palettes.len(), 1);
        palettes.append_empty();
        assert_eq!(palettes.len(), 2);
        palettes.move_to_last();
        assert_eq!(palettes.current(), &NamedPalette::default());
    }

    #[test]
    fn removal() {
        let (p1, p2, p3, p4) = test_palettes();
        let mut palettes = Palettes::new(p1.clone());
        palettes.add(p1);
        palettes.add(p2);
        palettes.add(p3);
        palettes.add(p4);
    }

    #[test]
    fn addition() {
        let (p1, p2, p3, p4) = test_palettes();
        let mut palettes = Palettes::new(p1.clone());
        palettes.add(p1);
        palettes.add(p2);
        palettes.add(p3);
        palettes.add(p4);
    }
}
