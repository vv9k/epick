use crate::color::{ChromaticAdaptationMethod, DisplayFormat, Illuminant, RgbWorkingSpace};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

fn enabled() -> bool {
    true
}

fn is_false(it: &bool) -> bool {
    !*it
}

fn is_true(it: &bool) -> bool {
    *it
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ColorSpaceSettings {
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub rgb: bool,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub cmyk: bool,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub hsv: bool,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub hsl: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub luv: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub lch_uv: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub lab: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub color_display_format: DisplayFormat,
    pub color_spaces: ColorSpaceSettings,
    pub rgb_working_space: RgbWorkingSpace,
    pub chromatic_adaptation_method: ChromaticAdaptationMethod,
    pub illuminant: Illuminant,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub cache_colors: bool,
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
            cache_colors: true,
        }
    }
}

impl Settings {
    /// Loads the settings from the configuration file located at `path`. The configuration file is
    /// expected to be a valid YAML file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let data = fs::read(path).context("failed to read configuration file")?;
        serde_yaml::from_slice(&data).context("failed to deserialize configuration")
    }

    /// Saves this settings as YAML file in the provided `path`.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let data = serde_yaml::to_vec(&self).context("failed to serialize settings")?;
        fs::write(path, &data).context("failed to write settings to file")
    }

    /// Returns system directory where configuration should be placed joined by the `name` parameter.
    pub fn dir(name: impl AsRef<str>) -> Option<PathBuf> {
        let name = name.as_ref();
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
    use crate::app::settings::Settings;
    use crate::color::{ChromaticAdaptationMethod, DisplayFormat, Illuminant, RgbWorkingSpace};
    use std::fs;

    #[test]
    fn loads_settings() {
        let tmp = tempdir::TempDir::new("settings-test").unwrap();
        let settings_str = r#"---
color_display_format:
  css-hsl:
    degree_symbol: true
color_spaces:
  hsv: false
  luv: true
  lab: true
rgb_working_space: Adobe
chromatic_adaptation_method: VonKries
illuminant: D50
"#;
        let path = tmp.path().join("settings.yaml");
        fs::write(&path, settings_str).unwrap();

        let settings = Settings::load(&path).unwrap();
        assert_eq!(settings.illuminant, Illuminant::D50);
        assert_eq!(settings.rgb_working_space, RgbWorkingSpace::Adobe);
        assert_eq!(
            settings.color_display_format,
            DisplayFormat::CssHsl {
                degree_symbol: true
            }
        );
        assert_eq!(
            settings.chromatic_adaptation_method,
            ChromaticAdaptationMethod::VonKries
        );

        assert!(settings.color_spaces.rgb);
        assert!(settings.color_spaces.cmyk);
        assert!(settings.color_spaces.hsl);
        assert!(settings.color_spaces.luv);
        assert!(settings.color_spaces.lab);
        assert!(!settings.color_spaces.hsv);
        assert!(!settings.color_spaces.lch_uv);
        assert!(!settings.color_spaces.lch_ab);
        assert!(settings.cache_colors);

        let path = tmp.path().join("new_settings.yaml");
        settings.save(&path).unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), settings_str);
    }
}
